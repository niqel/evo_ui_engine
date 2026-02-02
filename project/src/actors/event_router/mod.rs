// Alias monádicos locales (opcional, por estilo)
pub type Domain   = self::interpret_domain::SystemEvent;
pub type Codomain = self::interpret_codomain::InternalEvent;

pub mod interpret_domain;
pub mod interpret_codomain;
pub mod event_router;

// Exports ergonómicos
pub use event_router::EventRouter;
pub use interpret_domain::SystemEvent;
pub use interpret_codomain::{InternalEvent, InputKind};
