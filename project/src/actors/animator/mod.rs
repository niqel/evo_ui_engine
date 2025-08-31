// Alias monádicos locales (tu estilo Domain/Codomain)
pub type Domain   = self::animate_domain::AnimateDomain;
pub type Codomain = self::animate_codomain::AnimateCodomain;

pub mod animate_domain;
pub mod animate_codomain;
pub mod animator;

// Exports ergonómicos
pub use animator::Animator;
pub use animate_domain::AnimateDomain;
pub use animate_codomain::AnimateCodomain;
