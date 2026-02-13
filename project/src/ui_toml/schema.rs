use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UiToml {
    pub scene: SceneToml,
    #[serde(default)]
    pub acetate: Vec<AcetateToml>,
}

#[derive(Debug, Deserialize)]
pub struct SceneToml {
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub includes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct AcetateToml {
    pub id: String,
    #[serde(default = "default_z")]
    pub z: i32,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub fill: String,
    #[serde(default = "default_border")]
    pub border: String,
    #[serde(default = "default_border_thickness")]
    pub border_thickness: f32,
    #[serde(default)]
    pub text: Option<String>,
}

fn default_z() -> i32 {
    0
}

fn default_border() -> String {
    "#00000000".to_string()
}

fn default_border_thickness() -> f32 {
    0.0
}
