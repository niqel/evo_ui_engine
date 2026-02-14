use std::path::Path;
use std::time::{Duration, Instant, SystemTime};

use crate::contracts::event::Event;
use crate::contracts::scene::{Acetate, AcetateDesign, AcetateIO, AcetateStatus, Rect, Scene, SceneInfo};
use crate::core::Color;
use crate::runtime::app::{App, FrameContext, InputState, InputWants};
use crate::ui_toml::load_scene_from_file;

const BG_Z_INDEX: i32 = -100_000;

#[derive(Debug, Clone)]
struct SimpleAcetate {
    id: String,
    name: String,
    z_index: i32,
    area: Rect,
    design: AcetateDesign,
}

impl SimpleAcetate {
    fn new(id: &str, name: &str, z_index: i32, design: AcetateDesign) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            z_index,
            area: design.area.clone(),
            design,
        }
    }
}

impl Acetate for SimpleAcetate {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn area(&self) -> Rect {
        self.area.clone()
    }

    fn subscriptions(&self) -> Vec<crate::contracts::event::EventKind> {
        vec![]
    }

    fn react(&self, _event: &Event, _scene: &SceneInfo) -> Option<Box<dyn Acetate>> {
        None
    }

    fn perceive(&self, scene: &SceneInfo) -> SceneInfo {
        scene.clone()
    }

    fn output(&self) -> AcetateIO {
        AcetateIO {
            content: None,
            focus: false,
            status: AcetateStatus::Ready,
        }
    }

    fn design(&self) -> AcetateDesign {
        self.design.clone()
    }

    fn clone_box(&self) -> Box<dyn Acetate> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct LayerTemplate {
    id: String,
    name: String,
    z_index: i32,
    design: AcetateDesign,
}

#[derive(Clone, Default)]
struct SceneTemplate {
    layers: Vec<LayerTemplate>,
}

impl SceneTemplate {
    fn from_scene(scene: &Scene) -> Self {
        let layers = scene
            .acetates
            .iter()
            .map(|a| LayerTemplate {
                id: a.id(),
                name: a.name(),
                z_index: a.z_index(),
                design: a.design(),
            })
            .collect();

        Self { layers }
    }

}

pub struct TomlApp {
    scene_template: SceneTemplate,
    ui_path: String,
    last_mtime: Option<SystemTime>,
    last_reload_check: Instant,
}

impl TomlApp {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let ui_path = path.as_ref().to_string_lossy().into_owned();
        let scene_template = match load_scene_from_file(&ui_path) {
            Ok(scene) => {
                println!("Loaded scene from {ui_path}");
                SceneTemplate::from_scene(&scene)
            }
            Err(err) => {
                eprintln!("Failed to load {ui_path}: {err}");
                SceneTemplate::default()
            }
        };

        Self {
            scene_template,
            last_mtime: read_mtime(&ui_path),
            ui_path,
            last_reload_check: Instant::now(),
        }
    }

    fn maybe_hot_reload(&mut self) {
        if self.last_reload_check.elapsed() < Duration::from_millis(200) {
            return;
        }
        self.last_reload_check = Instant::now();

        let Some(mtime) = read_mtime(&self.ui_path) else {
            return;
        };

        let changed = match self.last_mtime {
            Some(last) => mtime > last,
            None => true,
        };
        if !changed {
            return;
        }

        match load_scene_from_file(&self.ui_path) {
            Ok(scene) => {
                self.scene_template = SceneTemplate::from_scene(&scene);
                self.last_mtime = Some(mtime);
                println!("✅ reloaded ui.toml");
            }
            Err(err) => {
                self.last_mtime = Some(mtime);
                eprintln!("❌ ui.toml error: {err}");
            }
        }
    }

    fn build_scene(&self, width: u32, height: u32) -> Scene {
        let mut acetates: Vec<Box<dyn Acetate>> = Vec::new();
        let mut has_bg = false;

        for layer in &self.scene_template.layers {
            let mut design = layer.design.clone();

            if layer.id == "bg" {
                design.area.x = 0;
                design.area.y = 0;
                design.area.width = width;
                design.area.height = height;
                has_bg = true;
            }

            if design.area.width == 0 {
                design.area.width = 1;
            }
            if design.area.height == 0 {
                design.area.height = 1;
            }

            acetates.push(Box::new(SimpleAcetate::new(
                &layer.id,
                &layer.name,
                if layer.id == "bg" {
                    BG_Z_INDEX
                } else {
                    layer.z_index
                },
                design,
            )));
        }

        if !has_bg {
            acetates.push(Box::new(SimpleAcetate::new(
                "bg",
                "Background",
                BG_Z_INDEX,
                AcetateDesign {
                    area: Rect {
                        x: 0,
                        y: 0,
                        width,
                        height,
                    },
                    background: default_background_color(),
                    border: Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    },
                    border_thickness: 0.0,
                    text: None,
                },
            )));
        }

        Scene::from(acetates)
    }
}

impl App for TomlApp {
    fn input_wants(&self) -> InputWants {
        InputWants {
            resize: true,
            ..InputWants::default()
        }
    }

    fn frame(&mut self, _events: &[Event], ctx: &FrameContext, _input: &InputState) -> Scene {
        self.maybe_hot_reload();
        self.build_scene(ctx.window_width, ctx.window_height)
    }
}

fn default_background_color() -> Color {
    Color {
        r: 0.08,
        g: 0.10,
        b: 0.14,
        a: 1.0,
    }
}

fn read_mtime(path: &str) -> Option<SystemTime> {
    std::fs::metadata(path).ok()?.modified().ok()
}
