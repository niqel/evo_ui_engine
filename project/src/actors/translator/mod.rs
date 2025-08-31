// Alias monádicos locales
pub type Domain   = crate::core::TranslateDomain;
pub type Codomain = crate::core::Snapshot;

pub mod translator;
pub use translator::Translator;
