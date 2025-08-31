//! Renderer con Vello (off-screen a RGBA8)

use super::Domain; // = crate::core::RenderDomain (alias definido en mod.rs)
use crate::actors::vello_adapter::VelloAdapter;

use vello::{Renderer as VelloRenderer, RendererOptions, RenderParams, AaConfig};
use vello::peniko::Color as PColor;
// 👇 usa SIEMPRE el wgpu re-exportado por Vello
use vello::wgpu;

pub struct RendererVello {
    device: wgpu::Device,
    queue: wgpu::Queue,
    vello: VelloRenderer,
}

impl RendererVello {
    pub fn new_offscreen() -> Self {
        pollster::block_on(async {
            let instance = wgpu::Instance::default();
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: None,
                    force_fallback_adapter: false,
                })
                .await
                .expect("No WGPU adapter");

            // 🚀 Usa los límites reales del adaptador (no downlevel_defaults)
            let required_limits = adapter.limits();

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("evo_ui_engine device"),
                        required_features: wgpu::Features::empty(),
                        required_limits,
                        // wgpu 24+ requiere memory_hints
                        memory_hints: wgpu::MemoryHints::default(),
                    },
                    None,
                )
                .await
                .expect("request_device failed");

            let vello = VelloRenderer::new(&device, RendererOptions::default())
                .expect("vello::Renderer::new failed");

            Self { device, queue, vello }
        })
    }

    /// Renderiza Snapshot -> RGBA8 (width*height*4)
    pub fn render_to_rgba8(
        &mut self,
        snapshot: Domain,          // <— usa el alias monádico del actor
        size: (u32, u32),
    ) -> Result<Vec<u8>, String> {
        let (width, height) = size;
        if width == 0 || height == 0 {
            return Ok(vec![]);
        }

        // 1) Snapshot -> Scene
        let scene = VelloAdapter::adapt(snapshot);

        // 2) Textura destino (requisitos de Vello)
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("offscreen color"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // 👇 Rgba8Unorm y STORAGE_BINDING (no SRGB / no RENDER_ATTACHMENT)
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        };
        let texture = self.device.create_texture(&texture_desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 3) Parámetros de render
        let params = RenderParams {
            base_color: PColor::from_rgba8(0, 0, 0, 0),
            width,
            height,
            antialiasing_method: AaConfig::Area,
        };

        // 4) Render a textura
        self.vello
            .render_to_texture(&self.device, &self.queue, &scene, &view, &params)
            .map_err(|e| format!("vello render_to_texture error: {:?}", e))?;

        // 5) Readback a CPU
        let bytes_pp = 4u32;
        let unpadded_bpr = width * bytes_pp;
        let padded_bpr = align_to_256(unpadded_bpr);

        let output_buffer_size = (padded_bpr * height) as u64;
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback buffer"),
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("copy encoder") });

        // wgpu 24+ usa TexelCopy*
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bpr),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );

        self.queue.submit(Some(encoder.finish()));

        // 6) Map + esperar callback con Maintain::Wait
        let slice = output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |res| {
            let _ = tx.send(res);
        });
        // bloquea hasta que termine el mapeo
        self.device.poll(wgpu::Maintain::Wait);
        match rx.recv().map_err(|_| "map_async channel dropped".to_string())? {
            Ok(()) => {}
            Err(e) => return Err(format!("map_async error: {:?}", e)),
        }

        let data = slice.get_mapped_range();
        let mut rgba = Vec::with_capacity((width * height * bytes_pp) as usize);
        for row in 0..height as usize {
            let start = row * padded_bpr as usize;
            let end = start + unpadded_bpr as usize;
            rgba.extend_from_slice(&data[start..end]);
        }
        drop(data);
        output_buffer.unmap();

        Ok(rgba)
    }
}

fn align_to_256(n: u32) -> u32 {
    const A: u32 = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT; // 256
    ((n + (A - 1)) / A) * A
}
