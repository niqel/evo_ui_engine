use std::time::Instant;
use crate::time_ticker::tick_domain::{TickDomain, tick};
use crate::time_ticker::tick_codomain::TickCodomain;

/// Actor que emite ticks funcionales.
pub struct TimeTicker {
    last_tick: TickCodomain,
}

impl TimeTicker {
    /// Crea un nuevo ticker con un instante base.
    pub fn new(initial_instant: Instant) -> Self {
        let initial_tick = TickCodomain::new(0, initial_instant);
        Self {
            last_tick: initial_tick,
        }
    }

    /// Acción principal: avanza el tiempo funcional.
    pub fn tick(&mut self) -> TickCodomain {
        let input = TickDomain {
            tick_number: self.last_tick.number,
            start_instant: Instant::now(), // o self.last_tick.timestamp + intervalo si decides usarlo después
        };

        let next = tick(input);
        self.last_tick = next.clone();
        next
    }
}
