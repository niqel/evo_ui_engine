// examples/toml_render.rs

use std::env;

use evo_ui_engine::actors::renderer::RendererVello;
use evo_ui_engine::actors::snapshot_builder::SnapshotBuilder;
use evo_ui_engine::ui_toml::load_scene_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).unwrap_or_else(|| "ui.toml".to_string());
    let scene = load_scene_from_file(&path)?;
    let (width, height) = (scene.width, scene.height);

    let snapshot = SnapshotBuilder::build(scene);
    let mut renderer = RendererVello::new_offscreen();
    let rgba = renderer
        .render_to_rgba8(snapshot, (width, height))
        .map_err(|e| format!("render error: {}", e))?;

    let out = "out.png";

    let file = std::fs::File::create(out)?;
    let w = &mut std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&rgba)?;

    println!("✅ PNG generado: {out}");
    Ok(())
}
