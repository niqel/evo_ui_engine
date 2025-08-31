// metrics.rs for scene module

// TODO: implement
/// Representa las métricas base que rigen la escena.
/// Escala, margen, espaciado, etc.
#[derive(Debug, Clone)]
pub struct Metrics {
    pub scale: f32,
    pub margin: u32,
    pub spacing: u32,
}
