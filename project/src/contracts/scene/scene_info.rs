// scene_info.rs for scene module

use super::metrics::Metrics;
use super::acetate_stub::AcetateStub;

/// Proyección funcional e inmutable de la escena.
/// Usada por acetatos para percibir el universo visual.
#[derive(Debug, Clone)]
pub struct SceneInfo {
    pub width: u32,
    pub height: u32,
    pub metrics: Metrics,
    pub stubs: Vec<AcetateStub>,
}
