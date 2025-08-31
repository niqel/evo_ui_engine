pub type Domain   = crate::core::RenderDomain;     // hoy = Snapshot
pub type Codomain = crate::core::RenderedFrame;    // hoy = Vec<u8>

pub mod renderer;
pub use renderer::RendererVello;
