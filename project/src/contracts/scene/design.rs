// design.rs for scene module

use super::rect::Rect;
use crate::core::Color;

/// Describe visualmente cómo debe representarse un acetate.
#[derive(Debug, Clone)]
pub struct AcetateDesign {
    pub area: Rect,
    pub background: Color,
    pub border: Color,
    pub border_thickness: f32,
    pub text: Option<String>,
}
