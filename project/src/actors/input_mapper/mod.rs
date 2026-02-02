// Alias monádicos locales
pub type Domain   = self::translate_domain::TranslateDomain; // = InternalEvent
pub type Codomain = crate::contracts::event::Event;

pub mod translate_domain;
pub mod input_mapper;

// Exports públicos ergonómicos
pub use input_mapper::InputMapper;
pub use crate::contracts::event::Event;
