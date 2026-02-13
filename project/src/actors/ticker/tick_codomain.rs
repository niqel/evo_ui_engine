use std::time::Instant;
use crate::contracts::event::Event;

/// Representa un instante funcional en el tiempo
#[derive(Debug, Clone, PartialEq)]
pub struct TickCodomain {
    pub number: u64,
    pub timestamp: Instant,
    pub events: Vec<Event>,
}

impl TickCodomain {
    /// Crea un nuevo TickCodomain vacío (con número y timestamp dados)
    pub fn new(number: u64, timestamp: Instant) -> Self {
        Self {
            number,
            timestamp,
            events: vec![],
        }
    }

    /// Devuelve una copia del TickCodomain con nuevos eventos
    pub fn with_events(mut self, events: Vec<Event>) -> Self {
        self.events = events;
        self
    }

    /// Devuelve los eventos asociados a este TickCodomain
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}
