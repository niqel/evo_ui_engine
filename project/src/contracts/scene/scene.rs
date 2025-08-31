// scene.rs for scene module

use super::metrics::Metrics;
use super::scene_info::SceneInfo;
use super::acetate::Acetate;
use super::acetate_stub::AcetateStub;

#[derive(Debug, Clone)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub metrics: Metrics,
    pub acetates: Vec<Box<dyn Acetate>>,
}

impl Scene {
    /// Proyección funcional de la escena para su percepción.
    pub fn info(&self) -> SceneInfo {
        let stubs: Vec<AcetateStub> = self
            .acetates
            .iter()
            .map(|a| AcetateStub {
                id: a.id(),
                z_index: a.z_index(),
                area: a.area(),
                design: Some(a.design()),
            })
            .collect();

        SceneInfo {
            width: self.width,
            height: self.height,
            metrics: self.metrics.clone(),
            stubs,
        }
    }
}
