// examples/offscreen.rs

use std::fs::File;
use std::io::BufWriter;

use evo_ui_engine::actors::renderer::RendererVello;
use evo_ui_engine::actors::snapshot_builder::SnapshotBuilder;
use evo_ui_engine::contracts::event::{Event, EventKind};
use evo_ui_engine::contracts::scene::{
    Acetate, AcetateDesign, AcetateIO, AcetateStatus, Rect, Scene, SceneInfo,
};
use evo_ui_engine::core::Color;

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

    fn subscriptions(&self) -> Vec<EventKind> {
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


fn save_png(path: &str, rgba: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(rgba)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tamaño del frame off-screen
    let (width, height) = (800u32, 500u32);

    // Acetate 0: fondo (Canvas) #161923 -> rgb(22,25,35)
    let background = SimpleAcetate::new(
        "bg",
        "Background",
        0,
        AcetateDesign {
            area: Rect { x: 0, y: 0, width, height },
            background: Color { r: 22.0 / 255.0, g: 25.0 / 255.0, b: 35.0 / 255.0, a: 1.0 },
            border: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            border_thickness: 0.0,
            text: None,
        },
    );

    // Acetate 1: rectángulo de prueba con borde
    let test_rect = SimpleAcetate::new(
        "card",
        "Card",
        10,
        AcetateDesign {
            area: Rect { x: 40, y: 40, width: 160, height: 100 },
            background: Color { r: 0.10, g: 0.60, b: 0.90, a: 1.0 },
            border: Color { r: 0.95, g: 0.95, b: 0.95, a: 1.0 },
            border_thickness: 2.0,
            text: None,
        },
    );

    let scene = Scene::from(vec![
        Box::new(background) as Box<dyn Acetate>,
        Box::new(test_rect) as Box<dyn Acetate>,
    ]);
    let snapshot = SnapshotBuilder::build(scene);

    // Render off-screen
    let mut renderer = RendererVello::new_offscreen();
    let rgba = renderer
        .render_to_rgba8(snapshot, (width, height))
        .map_err(|e| format!("render error: {}", e))?;

    // Guardar PNG
    let out = "out.png";
    save_png(out, &rgba, width, height)?;
    println!("✅ PNG generado: {out}");

    Ok(())
}
