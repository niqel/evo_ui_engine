//! Snapshot -> vello::Scene (adapter)
//! Convierte el codominio del SnapshotBuilder (Snapshot) en una vello::Scene.

use crate::core::{Snapshot, SnapshotLayer, Color, Rect as SnapRect};

use vello::Scene;
use vello::kurbo::{Rect, Affine, Stroke};
use vello::peniko::{Brush, Color as PColor, Fill};

pub struct VelloAdapter;

impl VelloAdapter {
    /// Construye y devuelve una `vello::Scene` nueva a partir del `Snapshot`.
    pub fn adapt(snapshot: Snapshot) -> Scene {
        let mut scene = Scene::new();
        build_scene(&mut scene, &snapshot);
        scene
    }

    /// Versión in-place: reutiliza una `Scene` externa (útil si cacheas la escena por frame).
    pub fn adapt_into(scene: &mut Scene, snapshot: Snapshot) {
        scene.reset();
        build_scene(scene, &snapshot);
    }
}

// --- helpers internos ---

fn to_u8(x: f32) -> u8 {
    let v = (x * 255.0).round();
    if v.is_nan() { return 0; }
    v.clamp(0.0, 255.0) as u8
}

fn to_pcolor(c: &Color) -> PColor {
    PColor::from_rgba8(to_u8(c.r), to_u8(c.g), to_u8(c.b), to_u8(c.a))
}

fn to_kurbo_rect(r: &SnapRect) -> Rect {
    Rect::new(
        r.x as f64,
        r.y as f64,
        (r.x + r.width as i32) as f64,
        (r.y + r.height as i32) as f64,
    )
}

fn build_scene(scene: &mut Scene, snapshot: &Snapshot) {
    // Dibuja capas por z ascendente (fondo primero)
    let mut layers = snapshot.layers.clone();
    layers.sort_by_key(|l| l.z_index);

    for layer in layers {
        draw_layer(scene, &layer);
    }
}

fn draw_layer(scene: &mut Scene, layer: &SnapshotLayer) {
    let rect = to_kurbo_rect(&layer.area);

    // Relleno
    let fill_brush = Brush::Solid(to_pcolor(&layer.style.fill_color));
    scene.fill(Fill::NonZero, Affine::IDENTITY, &fill_brush, None, &rect);

    // Borde
    if layer.style.border_thickness > 0.0 {
        let stroke_brush = Brush::Solid(to_pcolor(&layer.style.border_color));
        let stroke = Stroke::new(layer.style.border_thickness as f64);
        scene.stroke(&stroke, Affine::IDENTITY, &stroke_brush, None, &rect);
    }

    // TODO: texto (vello_glyph) cuando lo quieras
}
