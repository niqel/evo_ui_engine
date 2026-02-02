/// Información temporal para animación por tick.
/// Útil para `ticker` y `animator` sin acoplar a un reloj específico.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimationFrame {
    /// Número de tick/frames transcurridos (monótono).
    pub tick: u64,
    /// Delta time en segundos desde el último tick (p.ej. 1.0/60.0).
    pub dt: f32,
}
