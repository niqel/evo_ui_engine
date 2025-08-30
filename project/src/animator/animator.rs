//! animator module for animator

// TODO: implement
use crate::animator::animate_domain::AnimateDomain;
use crate::animator::animate_codomain::AnimateCodomain;

/// Actor funcional que genera nuevos eventos a partir de un evento dado.
/// Su responsabilidad es gobernar la animación (reacciones temporales o progresivas).
pub struct Animator;

impl Animator {
    /// Acción principal: analiza el evento de entrada y decide qué nuevos eventos generar.
    pub fn animate(_input: AnimateDomain) -> AnimateCodomain {
        vec![]
    }
}
