use super::tick_codomain::TickCodomain;
use std::time::Instant;

/// Dominio funcional que describe cómo se debe generar el siguiente tick.
pub struct TickDomain {
    pub tick_number: u64,
    pub start_instant: Instant,
}

/// Función pura que genera un nuevo TickCodomain a partir del dominio declarado.
pub fn tick(input: TickDomain) -> TickCodomain {
    TickCodomain {
        number: input.tick_number + 1,
        timestamp: Instant::now(), // También puedes usar input.start_instant si prefieres control total
        events: vec![],
    }
}
