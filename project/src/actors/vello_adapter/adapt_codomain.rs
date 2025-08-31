//! Codominio del actor VelloAdapter: construcción final para ejecutar en Renderer.
#[cfg(feature = "vello")]
#[derive(Default)]
pub struct VelloScene {
    scene: vello::Scene,
}

#[cfg(not(feature = "vello"))]
#[derive(Default)]
pub struct VelloScene {
    // En modo sin Vello, dejamos un placeholder para no romper la API.
}

#[cfg(feature = "vello")]
impl VelloScene {
    pub fn new() -> Self { Self { scene: vello::Scene::new() } }
    pub fn scene(&self) -> &vello::Scene { &self.scene }
    pub fn scene_mut(&mut self) -> &mut vello::Scene { &mut self.scene }
}

#[cfg(not(feature = "vello"))]
impl VelloScene {
    pub fn new() -> Self { Self {} }
}
