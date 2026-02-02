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

impl From<Vec<Box<dyn Acetate>>> for Scene {
    fn from(acetates: Vec<Box<dyn Acetate>>) -> Self {
        let mut max_x = 0u32;
        let mut max_y = 0u32;

        for acetate in &acetates {
            let area = acetate.area();
            max_x = max_x.max((area.x.max(0) as u32).saturating_add(area.width));
            max_y = max_y.max((area.y.max(0) as u32).saturating_add(area.height));
        }

        Scene {
            width: max_x,
            height: max_y,
            metrics: Metrics {
                scale: 1.0,
                margin: 0,
                spacing: 0,
            },
            acetates,
        }
    }
}
