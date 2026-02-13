use super::translate_domain::TranslateDomain;   // alias al InternalEvent
use crate::contracts::event::Event;             // evento funcional
use crate::actors::event_router::interpret_codomain::{InternalEvent, InputKind};
use crate::actors::ticker::TickCodomain;
use std::time::Instant;

/// Actor funcional que traduce eventos internos a eventos funcionales del sistema.
/// Esto desacopla las capas internas de los actores que operan sobre eventos puros.
pub struct InputMapper;

impl InputMapper {
    pub fn translate(input: TranslateDomain) -> Event {
        match input {
            InternalEvent::Tick => {
                let tick = TickCodomain::new(0, Instant::now());
                Event::Tick(tick)
            }
            InternalEvent::SystemExit => Event::Exit,

            InternalEvent::Input { kind } => match kind {
                InputKind::KeyPressed(k)   => Event::KeyPressed(k),
                InputKind::KeyReleased(k)  => Event::KeyReleased(k),
                InputKind::MouseMoved(x,y) => Event::MouseMoved(x, y),
                InputKind::MouseClicked    => Event::MouseClicked,
            },
        }
    }
}
