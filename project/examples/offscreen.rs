// examples/offscreen.rs

use std::fs::File;
use std::io::BufWriter;

use evo_ui_engine::actors::renderer::RendererVello;
use evo_ui_engine::core::{Snapshot, SnapshotLayer, SnapshotStyle, Color, Rect};

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

    // Capa 0: fondo (Canvas) #161923 -> rgb(22,25,35)
    let background = SnapshotLayer {
        z_index: 0,
        area: Rect { x: 0, y: 0, width, height },
        style: SnapshotStyle {
            fill_color: Color { r: 22.0/255.0, g: 25.0/255.0, b: 35.0/255.0, a: 1.0 },
            border_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            border_thickness: 0.0,
            text: None,
        },
    };

    // Capa 1: rectángulo de prueba para ver superposición
    let test_rect = SnapshotLayer {
        z_index: 10,
        area: Rect { x: 40, y: 40, width: 160, height: 100 },
        style: SnapshotStyle {
            fill_color: Color { r: 0.10, g: 0.60, b: 0.90, a: 1.0 },
            border_color: Color { r: 0.95, g: 0.95, b: 0.95, a: 1.0 },
            border_thickness: 2.0,
            text: None,
        },
    };

    // Snapshot final
    let snapshot = Snapshot { layers: vec![background, test_rect] };

    // Render off-screen
    let mut renderer = RendererVello::new_offscreen();
    let rgba = renderer
        .render_to_rgba8(snapshot, (width, height))
        .map_err(|e| format!("render error: {}", e))?;

    // Guardar PNG
    let out = "target/offscreen.png";
    save_png(out, &rgba, width, height)?;
    println!("✅ PNG generado: {out}");

    Ok(())
}
