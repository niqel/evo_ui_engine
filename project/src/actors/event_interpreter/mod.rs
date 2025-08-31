// Alias monádicos locales (opcional, por estilo)
pub type Domain   = self::interpret_domain::SystemEvent;
pub type Codomain = self::interpret_codomain::InternalEvent;

pub mod interpret_domain;
pub mod interpret_codomain;
pub mod event_interpreter;

// Exports ergonómicos
pub use event_interpreter::EventInterpreter;
pub use interpret_domain::SystemEvent;
pub use interpret_codomain::{InternalEvent, InputKind};
