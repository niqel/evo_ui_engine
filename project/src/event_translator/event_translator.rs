use crate::event_translator::translate_domain::TranslateDomain;
use crate::event_translator::translate_codomain::Event;
use crate::event_interpreter::interpret_codomain::{InternalEvent, InputKind};

/// Actor funcional que traduce eventos internos a eventos funcionales del sistema.
/// Esto desacopla las capas internas de los actores que operan sobre eventos puros.
pub struct EventTranslator;

impl EventTranslator {
    pub fn translate(input: TranslateDomain) -> Event {
        match input {
            InternalEvent::Tick => Event::Tick,

            InternalEvent::SystemExit => Event::Exit,

            InternalEvent::Input { kind } => match kind {
                InputKind::KeyPressed(k) => Event::KeyPressed(k),
                InputKind::KeyReleased(k) => Event::KeyReleased(k),
                InputKind::MouseMoved(x, y) => Event::MouseMoved(x, y),
                InputKind::MouseClicked => Event::MouseMoved(0, 0), // 🔧 temporal si no tienes MouseClicked
            },
        }
    }
}
