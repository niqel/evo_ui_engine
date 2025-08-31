mod acetate;
mod acetate_io;
mod acetate_stub;
mod design;
mod metrics;
mod rect;
mod scene;
mod scene_info;
mod acetate_init;

pub use acetate::Acetate;
pub use acetate_io::{AcetateIO, AcetateStatus};
pub use acetate_stub::AcetateStub;
pub use design::AcetateDesign;
pub use metrics::Metrics;
pub use rect::Rect;
pub use scene::Scene;
pub use scene_info::SceneInfo;
pub use acetate_init::AcetateInit;

// Re-export conveniente
