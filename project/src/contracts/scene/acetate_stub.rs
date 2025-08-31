// acetate_stub.rs for scene module

use super::design::AcetateDesign;
use super::rect::Rect;

/// Proyección funcional de un Acetate.
/// Resume lo necesario para construir la representación visual de la Scene.
#[derive(Debug, Clone)]
pub struct AcetateStub {
    pub id: String,
    pub z_index: i32,
    pub area: Rect,
    pub design: Option<AcetateDesign>,
}
