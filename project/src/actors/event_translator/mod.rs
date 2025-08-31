// Alias monádicos locales
pub type Domain   = self::translate_domain::TranslateDomain; // = InternalEvent
pub type Codomain = self::translate_codomain::Event;

pub mod translate_domain;
pub mod translate_codomain;
pub mod event_translator;

// Exports públicos ergonómicos
pub use event_translator::EventTranslator;
pub use translate_codomain::Event;
