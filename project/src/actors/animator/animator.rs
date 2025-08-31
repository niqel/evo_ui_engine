use super::animate_domain::AnimateDomain;
use super::animate_codomain::AnimateCodomain;
use crate::contracts::event::Event;

/// Actor de animación: dado el estado/escena y un evento (p. ej. Tick),
/// devuelve una nueva escena (inmutable).
pub struct Animator;

impl Animator {
    /// Paso de animación: hoy es passthrough; aquí pondrás keyframes/tweens.
    pub fn step(scene: AnimateDomain, _event: &Event) -> AnimateCodomain {
        // TODO: aplicar reglas de animación (keyframes, tween, easing) por acetato
        scene
    }
}
