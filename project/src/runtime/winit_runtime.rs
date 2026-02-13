use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use crate::actors::event_router::{EventRouter, SystemEvent};
use crate::actors::input_mapper::InputMapper;
use crate::actors::snapshot_builder::SnapshotBuilder;
use crate::actors::ticker::Ticker;
use crate::actors::vello_adapter::VelloAdapter;
use crate::contracts::event::{Event, MouseButton};
use crate::runtime::app::{App, FrameContext, InputState};
use crate::runtime::toml_app::TomlApp;
use crate::ui_toml::UiTomlError;
use vello::peniko::Color as PColor;
use vello::wgpu;
use vello::{AaConfig, RenderParams, Renderer as VelloRenderer, RendererOptions};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, Ime, KeyEvent, MouseButton as WinitMouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::Key;
use winit::window::{Window, WindowAttributes, WindowId};

const BLIT_WGSL: &str = r#"
@group(0) @binding(0) var offscreen_tex: texture_2d<f32>;
@group(0) @binding(1) var offscreen_sampler: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VsOut {
    var out: VsOut;
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    let uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );

    out.pos = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = uvs[vertex_index];
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return textureSample(offscreen_tex, offscreen_sampler, in.uv);
}
"#;

#[derive(Debug)]
pub enum RuntimeError {
    Io(std::io::Error),
    Toml(UiTomlError),
    Wgpu(String),
    Other(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::Io(err) => write!(f, "io error: {err}"),
            RuntimeError::Toml(err) => write!(f, "toml error: {err}"),
            RuntimeError::Wgpu(err) => write!(f, "wgpu error: {err}"),
            RuntimeError::Other(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<std::io::Error> for RuntimeError {
    fn from(value: std::io::Error) -> Self {
        RuntimeError::Io(value)
    }
}

impl From<UiTomlError> for RuntimeError {
    fn from(value: UiTomlError) -> Self {
        RuntimeError::Toml(value)
    }
}

pub fn run_default() -> Result<(), RuntimeError> {
    run_from_path("ui.toml")
}

pub fn run_from_path(path: impl AsRef<Path>) -> Result<(), RuntimeError> {
    let path_buf = path.as_ref().to_path_buf();
    let app = TomlApp::new(&path_buf);
    run_app_from_path(path_buf, app)
}

pub fn run_app_from_path<A: App>(path: impl AsRef<Path>, app: A) -> Result<(), RuntimeError> {
    let event_loop =
        EventLoop::new().map_err(|err| RuntimeError::Other(format!("event loop error: {err}")))?;

    let mut runner = Runner::new(path.as_ref().to_string_lossy().into_owned(), app);
    if let Err(err) = event_loop.run_app(&mut runner) {
        return Err(RuntimeError::Other(format!("event loop run error: {err}")));
    }

    if let Some(err) = runner.runtime_error.take() {
        return Err(err);
    }

    Ok(())
}

enum RenderAction {
    Continue,
    ExitApp,
}

struct Runner<A: App> {
    app: A,
    ui_path: String,
    ticker: Ticker,
    input: InputState,
    pending_events: Vec<Event>,
    last_frame_at: Instant,
    fps_started_at: Instant,
    rendered_frames: u64,
    last_fps: Option<f64>,
    closing: bool,
    window: Option<Arc<Window>>,
    window_id: Option<WindowId>,
    gpu: Option<GpuState>,
    runtime_error: Option<RuntimeError>,
}

impl<A: App> Runner<A> {
    fn new(ui_path: String, app: A) -> Self {
        Self {
            app,
            ui_path,
            ticker: Ticker::new(Instant::now()),
            input: InputState::default(),
            pending_events: Vec::new(),
            last_frame_at: Instant::now(),
            fps_started_at: Instant::now(),
            rendered_frames: 0,
            last_fps: None,
            closing: false,
            window: None,
            window_id: None,
            gpu: None,
            runtime_error: None,
        }
    }

    fn queue_event(&mut self, event: Event) {
        self.pending_events.push(event);
    }

    fn current_window_size(&self) -> Option<(u32, u32)> {
        self.window.as_ref().map(|window| {
            let size = window.inner_size();
            (size.width.max(1), size.height.max(1))
        })
    }

    fn shutdown(&mut self, event_loop: &ActiveEventLoop) {
        if self.closing {
            return;
        }
        self.closing = true;

        if let Some(gpu) = self.gpu.as_mut() {
            gpu.release_surface_resources();
            gpu.device.poll(wgpu::Maintain::Wait);
        }

        if let Some(mut gpu) = self.gpu.take() {
            gpu.release_surface_resources();
            let device = gpu.device.clone();
            drop(gpu);
            device.poll(wgpu::Maintain::Wait);
        }

        self.window = None;
        event_loop.exit();
    }

    fn render(&mut self) -> RenderAction {
        if self.closing {
            return RenderAction::Continue;
        }

        let Some((width, height)) = self.current_window_size() else {
            return RenderAction::Continue;
        };
        self.input.window_width = width;
        self.input.window_height = height;

        let Some(gpu) = self.gpu.as_mut() else {
            return RenderAction::Continue;
        };

        if gpu.config.width != width || gpu.config.height != height {
            gpu.resize(width, height);
        }
        if gpu.config.width == 0 || gpu.config.height == 0 {
            return RenderAction::Continue;
        }

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_at);
        self.last_frame_at = now;

        let tick = self.ticker.tick();
        let mut events = Vec::with_capacity(self.pending_events.len() + 1);
        events.push(Event::Tick(tick.clone()));
        events.extend(std::mem::take(&mut self.pending_events));

        let ctx = FrameContext {
            tick_number: tick.number,
            dt,
            timestamp: tick.timestamp,
            window_width: width,
            window_height: height,
            fps: self.last_fps,
        };

        let scene = self.app.frame(&events, &ctx, &self.input);
        self.input.text_buffer = None;

        let snapshot = SnapshotBuilder::build(scene);
        let vello_scene = VelloAdapter::adapt(snapshot);

        let params = RenderParams {
            base_color: PColor::from_rgba8(0, 0, 0, 0),
            width,
            height,
            antialiasing_method: AaConfig::Area,
        };
        let Some(offscreen_view) = gpu.offscreen_view.as_ref() else {
            return RenderAction::Continue;
        };

        if let Err(err) = gpu
            .vello
            .render_to_texture(&gpu.device, &gpu.queue, &vello_scene, offscreen_view, &params)
        {
            eprintln!("Vello render error: {err:?}");
            return RenderAction::Continue;
        }

        let Some(surface) = gpu.surface.as_ref() else {
            return RenderAction::Continue;
        };

        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                gpu.reconfigure();
                return RenderAction::Continue;
            }
            Err(wgpu::SurfaceError::Timeout) => {
                return RenderAction::Continue;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                eprintln!("Surface out of memory; exiting app");
                return RenderAction::ExitApp;
            }
            Err(wgpu::SurfaceError::Other) => {
                eprintln!("Surface error: Other");
                return RenderAction::Continue;
            }
        };

        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("present encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("present pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&gpu.blit_pipeline);
            let Some(blit_bind_group) = gpu.blit_bind_group.as_ref() else {
                return RenderAction::Continue;
            };
            pass.set_bind_group(0, blit_bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        gpu.queue.submit(Some(encoder.finish()));
        gpu.device.poll(wgpu::Maintain::Poll);
        frame.present();

        self.rendered_frames += 1;
        if self.rendered_frames % 120 == 0 {
            let elapsed = self.fps_started_at.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let fps = 120.0 / elapsed;
                self.last_fps = Some(fps);
                println!("fps: {:.1}", fps);
            }
            self.fps_started_at = Instant::now();
        }

        RenderAction::Continue
    }
}

impl<A: App> ApplicationHandler for Runner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title(format!("evo_ui_engine | winit | {}", self.ui_path)),
                )
                .expect("failed to create window"),
        );

        let gpu = match pollster::block_on(GpuState::new(window.clone())) {
            Ok(gpu) => gpu,
            Err(err) => {
                eprintln!("GPU init error: {err}");
                self.runtime_error = Some(RuntimeError::Wgpu(err));
                event_loop.exit();
                return;
            }
        };

        self.input.window_width = gpu.config.width.max(1);
        self.input.window_height = gpu.config.height.max(1);

        self.window_id = Some(window.id());
        self.window = Some(window);
        self.gpu = Some(gpu);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if Some(window_id) != self.window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.queue_event(Event::Exit);
                self.shutdown(event_loop)
            }
            WindowEvent::Resized(_) => {
                let Some((width, height)) = self.current_window_size() else {
                    return;
                };
                self.input.window_width = width;
                self.input.window_height = height;
                if let Some(gpu) = self.gpu.as_mut() {
                    gpu.resize(width, height);
                }
                self.queue_event(Event::WindowResized { width, height });
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                let Some((width, height)) = self.current_window_size() else {
                    return;
                };
                self.input.window_width = width;
                self.input.window_height = height;
                if let Some(gpu) = self.gpu.as_mut() {
                    gpu.resize(width, height);
                }
                self.queue_event(Event::WindowResized { width, height });
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let system_event =
                    SystemEvent::MouseMove(position.x.round() as i32, position.y.round() as i32);
                let internal_event = EventRouter::interpret(system_event);
                let app_event = InputMapper::translate(internal_event);
                self.input.mouse_x = position.x.round() as i32;
                self.input.mouse_y = position.y.round() as i32;
                self.queue_event(app_event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let Some(mapped) = map_mouse_button(button) else {
                    return;
                };
                let x = self.input.mouse_x;
                let y = self.input.mouse_y;

                match state {
                    ElementState::Pressed => {
                        self.input.mouse_buttons_down.insert(mapped);
                        self.queue_event(Event::MouseDown {
                            button: mapped,
                            x,
                            y,
                        });
                    }
                    ElementState::Released => {
                        self.input.mouse_buttons_down.remove(&mapped);
                        self.queue_event(Event::MouseUp {
                            button: mapped,
                            x,
                            y,
                        });
                        self.queue_event(Event::MouseClicked);
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key = key_event_to_string(&event);
                match event.state {
                    ElementState::Pressed => {
                        self.input.keys_down.insert(key.clone());
                        self.queue_event(Event::KeyPressed(key));
                    }
                    ElementState::Released => {
                        self.input.keys_down.remove(&key);
                        self.queue_event(Event::KeyReleased(key));
                    }
                }
            }
            WindowEvent::Ime(Ime::Commit(text)) => {
                self.input.text_buffer = Some(text.clone());
                self.queue_event(Event::TextInput(text));
            }
            WindowEvent::RedrawRequested => {
                if self.closing {
                    return;
                }
                if let RenderAction::ExitApp = self.render() {
                    self.shutdown(event_loop);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if !self.closing {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
}

impl<A: App> Drop for Runner<A> {
    fn drop(&mut self) {
        if self.closing {
            return;
        }
        if let Some(mut gpu) = self.gpu.take() {
            gpu.release_surface_resources();
            let device = gpu.device.clone();
            drop(gpu);
            device.poll(wgpu::Maintain::Wait);
        }
    }
}

fn map_mouse_button(button: WinitMouseButton) -> Option<MouseButton> {
    match button {
        WinitMouseButton::Left => Some(MouseButton::Left),
        WinitMouseButton::Right => Some(MouseButton::Right),
        WinitMouseButton::Middle => Some(MouseButton::Middle),
        WinitMouseButton::Back => Some(MouseButton::Other(4)),
        WinitMouseButton::Forward => Some(MouseButton::Other(5)),
        WinitMouseButton::Other(v) => u8::try_from(v).ok().map(MouseButton::Other),
    }
}

fn key_event_to_string(event: &KeyEvent) -> String {
    match &event.logical_key {
        Key::Character(text) => text.to_string(),
        other => format!("{other:?}"),
    }
}

struct GpuState {
    surface: Option<wgpu::Surface<'static>>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    offscreen_texture: Option<wgpu::Texture>,
    offscreen_view: Option<wgpu::TextureView>,
    blit_pipeline: wgpu::RenderPipeline,
    blit_bind_group_layout: wgpu::BindGroupLayout,
    blit_bind_group: Option<wgpu::BindGroup>,
    sampler: wgpu::Sampler,
    vello: VelloRenderer,
}

impl GpuState {
    async fn new(window: Arc<Window>) -> Result<Self, String> {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window)
            .map_err(|e| format!("create_surface failed: {e}"))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| "request_adapter failed: no suitable adapter".to_string())?;

        let required_limits = adapter.limits();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("winit_window device"),
                    required_features: wgpu::Features::empty(),
                    required_limits,
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .map_err(|e| format!("request_device failed: {e}"))?;

        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .or_else(|| caps.formats.first().copied())
            .ok_or_else(|| "surface has no supported formats".to_string())?;
        let present_mode = if caps.present_modes.contains(&wgpu::PresentMode::Fifo) {
            wgpu::PresentMode::Fifo
        } else {
            *caps
                .present_modes
                .first()
                .ok_or_else(|| "surface has no supported present modes".to_string())?
        };
        let alpha_mode = *caps
            .alpha_modes
            .first()
            .ok_or_else(|| "surface has no supported alpha modes".to_string())?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let (offscreen_texture, offscreen_view) =
            Self::create_offscreen_texture(&device, config.width, config.height);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("offscreen sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("blit shader"),
            source: wgpu::ShaderSource::Wgsl(BLIT_WGSL.into()),
        });

        let blit_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("blit bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("blit pipeline layout"),
            bind_group_layouts: &[&blit_bind_group_layout],
            push_constant_ranges: &[],
        });

        let blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blit pipeline"),
            layout: Some(&blit_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        let blit_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blit bind group"),
            layout: &blit_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&offscreen_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let vello = VelloRenderer::new(&device, RendererOptions::default())
            .map_err(|e| format!("vello::Renderer::new failed: {e:?}"))?;

        Ok(Self {
            surface: Some(surface),
            device,
            queue,
            config,
            offscreen_texture: Some(offscreen_texture),
            offscreen_view: Some(offscreen_view),
            blit_pipeline,
            blit_bind_group_layout,
            blit_bind_group: Some(blit_bind_group),
            sampler,
            vello,
        })
    }

    fn release_surface_resources(&mut self) {
        self.blit_bind_group = None;
        self.offscreen_view = None;
        self.offscreen_texture = None;
        self.surface = None;
    }

    fn create_offscreen_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("vello offscreen texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    }

    fn reconfigure(&mut self) {
        if self.config.width == 0 || self.config.height == 0 {
            return;
        }
        if let Some(surface) = self.surface.as_ref() {
            surface.configure(&self.device, &self.config);
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            self.config.width = width;
            self.config.height = height;
            return;
        }

        self.config.width = width;
        self.config.height = height;
        let Some(surface) = self.surface.as_ref() else {
            return;
        };
        surface.configure(&self.device, &self.config);

        let (offscreen_texture, offscreen_view) =
            Self::create_offscreen_texture(&self.device, width, height);
        self.offscreen_texture = Some(offscreen_texture);
        self.offscreen_view = Some(offscreen_view);

        self.blit_bind_group = Some(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blit bind group"),
            layout: &self.blit_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        self.offscreen_view
                            .as_ref()
                            .expect("offscreen view should exist after resize"),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        }));
    }
}
