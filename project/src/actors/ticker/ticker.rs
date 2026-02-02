use std::time::Instant;
use super::tick_domain::{TickDomain, tick};
use super::tick_codomain::TickCodomain;

/// Actor que emite ticks funcionales.
pub struct Ticker {
    last_tick: TickCodomain,
}

impl Ticker {
    /// Crea un nuevo ticker con un instante base.
    pub fn new(initial_instant: Instant) -> Self {
        let initial_tick = TickCodomain::new(0, initial_instant);
        Self { last_tick: initial_tick }
    }

    /// Acción principal: avanza el tiempo funcional.
    pub fn tick(&mut self) -> TickCodomain {
        let input = TickDomain {
            tick_number: self.last_tick.number,
            // si luego quieres usar un intervalo fijo, cámbialo aquí
            start_instant: Instant::now(),
        };

        let next = tick(input);
        self.last_tick = next.clone();
        next
    }
}
