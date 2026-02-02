// examples/offscreen_frames.rs

use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

use evo_ui_engine::actors::renderer::RendererVello;
use evo_ui_engine::actors::snapshot_builder::SnapshotBuilder;
use evo_ui_engine::actors::ticker::Ticker;
use evo_ui_engine::contracts::event::Event;
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

    fn subscriptions(&self) -> Vec<Event> {
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
    let (width, height) = (800u32, 450u32);
    let frames = 60u32;
    let mut ticker = Ticker::new(Instant::now());
    let mut renderer = RendererVello::new_offscreen();

    for frame in 0..frames {
        let tick = ticker.tick();
        let x = 20 + ((tick.number as i32 * 5) % 600);

        let background = SimpleAcetate::new(
            "bg",
            "Background",
            0,
            AcetateDesign {
                area: Rect { x: 0, y: 0, width, height },
                background: Color { r: 0.08, g: 0.10, b: 0.14, a: 1.0 },
                border: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
                border_thickness: 0.0,
                text: None,
            },
        );

        let mover = SimpleAcetate::new(
            "mover",
            "Mover",
            10,
            AcetateDesign {
                area: Rect { x, y: 140, width: 160, height: 90 },
                background: Color { r: 0.20, g: 0.70, b: 0.35, a: 1.0 },
                border: Color { r: 0.95, g: 0.95, b: 0.95, a: 1.0 },
                border_thickness: 2.0,
                text: None,
            },
        );

        let scene = Scene::from(vec![
            Box::new(background) as Box<dyn Acetate>,
            Box::new(mover) as Box<dyn Acetate>,
        ]);
        let snapshot = SnapshotBuilder::build(scene);
        let rgba = renderer
            .render_to_rgba8(snapshot, (width, height))
            .map_err(|e| format!("render error: {}", e))?;

        let out = format!("out_{:03}.png", frame);
        save_png(&out, &rgba, width, height)?;
        println!("✅ frame {frame}: {out}");
    }

    Ok(())
}
