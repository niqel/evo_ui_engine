use std::fmt;
use std::fs;
use std::path::Path;

use crate::contracts::event::{Event, EventKind};
use crate::contracts::scene::{
    Acetate, AcetateDesign, AcetateIO, AcetateStatus, Metrics, Rect, Scene, SceneInfo,
};
use crate::core::Color;

use serde::de::Error as _;
use serde::Deserialize;

use super::schema::AcetateToml;

#[derive(Debug)]
pub enum UiTomlError {
    Io(std::io::Error),
    ParseToml(toml::de::Error),
    InvalidColor {
        value: String,
        acetate_index: Option<usize>,
        field: &'static str,
    },
    InvalidDimensions {
        value: String,
        acetate_index: Option<usize>,
        field: &'static str,
    },
    MissingField {
        acetate_index: Option<usize>,
        field: &'static str,
    },
}

impl fmt::Display for UiTomlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiTomlError::Io(err) => write!(f, "io error: {}", err),
            UiTomlError::ParseToml(err) => write!(f, "toml parse error: {}", err),
            UiTomlError::InvalidColor {
                value,
                acetate_index,
                field,
            } => {
                if let Some(index) = acetate_index {
                    write!(
                        f,
                        "invalid color for acetate[{}].{}: {}",
                        index, field, value
                    )
                } else {
                    write!(f, "invalid color for {}: {}", field, value)
                }
            }
            UiTomlError::InvalidDimensions {
                value,
                acetate_index,
                field,
            } => {
                if let Some(index) = acetate_index {
                    write!(
                        f,
                        "invalid dimensions for acetate[{}].{}: {}",
                        index, field, value
                    )
                } else {
                    write!(f, "invalid dimensions for {}: {}", field, value)
                }
            }
            UiTomlError::MissingField {
                acetate_index,
                field,
            } => {
                if let Some(index) = acetate_index {
                    write!(f, "missing field acetate[{}].{}", index, field)
                } else {
                    write!(f, "missing field {}", field)
                }
            }
        }
    }
}

impl std::error::Error for UiTomlError {}

impl From<std::io::Error> for UiTomlError {
    fn from(err: std::io::Error) -> Self {
        UiTomlError::Io(err)
    }
}

impl From<toml::de::Error> for UiTomlError {
    fn from(err: toml::de::Error) -> Self {
        UiTomlError::ParseToml(err)
    }
}

#[derive(Debug, Deserialize)]
struct RawUiToml {
    scene: Option<RawSceneToml>,
    #[serde(default)]
    acetate: Vec<RawAcetateToml>,
}

#[derive(Debug, Deserialize)]
struct RawSceneToml {
    width: Option<i64>,
    height: Option<i64>,
    includes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct RawAcetateToml {
    id: Option<String>,
    #[serde(default = "default_z_i64")]
    z: i64,
    x: Option<i64>,
    y: Option<i64>,
    w: Option<i64>,
    h: Option<i64>,
    fill: Option<String>,
    #[serde(default = "default_border")]
    border: String,
    #[serde(default = "default_border_thickness")]
    border_thickness: f32,
    #[serde(default)]
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawIncludeToml {
    #[serde(default)]
    acetate: Vec<RawAcetateToml>,
}

fn default_z_i64() -> i64 {
    0
}

fn default_border() -> String {
    "#00000000".to_string()
}

fn default_border_thickness() -> f32 {
    0.0
}

#[derive(Debug, Clone)]
struct TomlAcetate {
    id: String,
    name: String,
    z_index: i32,
    area: Rect,
    design: AcetateDesign,
}

impl Acetate for TomlAcetate {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn area(&self) -> Rect {
        self.area.clone()
    }

    fn subscriptions(&self) -> Vec<EventKind> {
        vec![]
    }

    fn react(&self, _event: &Event, _scene: &SceneInfo) -> Option<Box<dyn Acetate>> {
        None
    }

    fn perceive(&self, scene: &SceneInfo) -> SceneInfo {
        scene.clone()
    }

    fn output(&self) -> AcetateIO {
        AcetateIO {
            content: None,
            focus: false,
            status: AcetateStatus::Ready,
        }
    }

    fn design(&self) -> AcetateDesign {
        self.design.clone()
    }

    fn clone_box(&self) -> Box<dyn Acetate> {
        Box::new(self.clone())
    }
}

fn parse_color(
    value: &str,
    acetate_index: Option<usize>,
    field: &'static str,
) -> Result<Color, UiTomlError> {
    let value = value.trim();
    let hex = value.strip_prefix('#').unwrap_or(value);
    let bytes = match hex.len() {
        6 => u32::from_str_radix(hex, 16).ok().map(|rgb| (rgb << 8) | 0xFF),
        8 => u32::from_str_radix(hex, 16).ok(),
        _ => None,
    }
    .ok_or_else(|| UiTomlError::InvalidColor {
        value: value.to_string(),
        acetate_index,
        field,
    })?;

    let r = ((bytes >> 24) & 0xFF) as f32 / 255.0;
    let g = ((bytes >> 16) & 0xFF) as f32 / 255.0;
    let b = ((bytes >> 8) & 0xFF) as f32 / 255.0;
    let a = (bytes & 0xFF) as f32 / 255.0;

    Ok(Color { r, g, b, a })
}

fn acetate_from_toml(
    input: &AcetateToml,
    acetate_index: usize,
) -> Result<TomlAcetate, UiTomlError> {
    let background = parse_color(&input.fill, Some(acetate_index), "fill")?;
    let border = parse_color(&input.border, Some(acetate_index), "border")?;
    let area = Rect {
        x: input.x,
        y: input.y,
        width: input.w,
        height: input.h,
    };

    Ok(TomlAcetate {
        id: input.id.clone(),
        name: input.id.clone(),
        z_index: input.z,
        area: area.clone(),
        design: AcetateDesign {
            area,
            background,
            border,
            border_thickness: input.border_thickness,
            text: input.text.clone(),
        },
    })
}

fn require_field<T>(
    value: Option<T>,
    field: &'static str,
    acetate_index: Option<usize>,
) -> Result<T, UiTomlError> {
    value.ok_or(UiTomlError::MissingField {
        acetate_index,
        field,
    })
}

fn parse_u32_dimensions(
    value: i64,
    field: &'static str,
    acetate_index: Option<usize>,
) -> Result<u32, UiTomlError> {
    if value < 0 || value > i64::from(u32::MAX) {
        return Err(UiTomlError::InvalidDimensions {
            value: value.to_string(),
            acetate_index,
            field,
        });
    }
    Ok(value as u32)
}

fn parse_i32(
    value: i64,
    field: &'static str,
    acetate_index: Option<usize>,
) -> Result<i32, UiTomlError> {
    if value < i64::from(i32::MIN) || value > i64::from(i32::MAX) {
        let prefix = match acetate_index {
            Some(index) => format!("acetate[{}].", index),
            None => String::new(),
        };
        let err = toml::de::Error::custom(format!(
            "value out of range for {}{}",
            prefix, field
        ));
        return Err(UiTomlError::ParseToml(err));
    }
    Ok(value as i32)
}

fn build_scene_from_raw(
    raw: RawUiToml,
    include_acetates: Vec<RawAcetateToml>,
) -> Result<Scene, UiTomlError> {
    let scene = require_field(raw.scene, "scene", None)?;
    let scene_width = require_field(scene.width, "scene.width", None)?;
    let scene_height = require_field(scene.height, "scene.height", None)?;
    let width = parse_u32_dimensions(scene_width, "scene.width", None)?;
    let height = parse_u32_dimensions(scene_height, "scene.height", None)?;

    let mut raw_acetates = raw.acetate;
    raw_acetates.extend(include_acetates);

    let mut acetates: Vec<Box<dyn Acetate>> = Vec::with_capacity(raw_acetates.len());
    let mut parsed_acetate = Vec::with_capacity(raw_acetates.len());

    for (index, acetate) in raw_acetates.into_iter().enumerate() {
        let id = require_field(acetate.id, "id", Some(index))?;
        let z = parse_i32(acetate.z, "z", Some(index))?;
        let x = parse_i32(require_field(acetate.x, "x", Some(index))?, "x", Some(index))?;
        let y = parse_i32(require_field(acetate.y, "y", Some(index))?, "y", Some(index))?;
        let w =
            parse_u32_dimensions(require_field(acetate.w, "w", Some(index))?, "w", Some(index))?;
        let h =
            parse_u32_dimensions(require_field(acetate.h, "h", Some(index))?, "h", Some(index))?;
        let fill = require_field(acetate.fill, "fill", Some(index))?;
        let border = acetate.border;
        let border_thickness = acetate.border_thickness;

        parsed_acetate.push(AcetateToml {
            id,
            z,
            x,
            y,
            w,
            h,
            fill,
            border,
            border_thickness,
            text: acetate.text,
        });
    }

    for (index, acetate) in parsed_acetate.iter().enumerate() {
        let instance = acetate_from_toml(acetate, index)?;
        acetates.push(Box::new(instance) as Box<dyn Acetate>);
    }

    Ok(Scene {
        width,
        height,
        metrics: Metrics {
            scale: 1.0,
            margin: 0,
            spacing: 0,
        },
        acetates,
    })
}

pub fn load_scene_from_str(toml_str: &str) -> Result<Scene, UiTomlError> {
    let raw: RawUiToml = toml::from_str(toml_str)?;
    build_scene_from_raw(raw, vec![])
}

pub fn load_scene_from_file(path: impl AsRef<Path>) -> Result<Scene, UiTomlError> {
    let path = path.as_ref();
    let toml_str = fs::read_to_string(path)?;
    let raw: RawUiToml = toml::from_str(&toml_str)?;

    let mut include_acetates = Vec::new();
    let includes = raw
        .scene
        .as_ref()
        .and_then(|scene| scene.includes.clone())
        .unwrap_or_default();
    let base_dir = path.parent().unwrap_or(Path::new("."));

    for include in includes {
        let include_path = base_dir.join(&include);
        let include_str = fs::read_to_string(&include_path).map_err(|err| {
            UiTomlError::Io(std::io::Error::new(
                err.kind(),
                format!("failed to read include '{}': {}", include_path.display(), err),
            ))
        })?;
        let include_raw: RawIncludeToml = toml::from_str(&include_str).map_err(|err| {
            UiTomlError::ParseToml(toml::de::Error::custom(format!(
                "in include '{}': {}",
                include_path.display(),
                err
            )))
        })?;
        include_acetates.extend(include_raw.acetate);
    }

    build_scene_from_raw(raw, include_acetates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn load_scene_from_file_merges_includes_relative_to_root() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("evo_ui_engine_ui_toml_{unique}"));
        let includes_dir = root.join("src/acetates");
        fs::create_dir_all(&includes_dir).expect("create include dir");

        let include_path = includes_dir.join("space.toml");
        fs::write(
            &include_path,
            r##"
[[acetate]]
id = "from_include"
x = 4
y = 5
w = 60
h = 30
fill = "#445566"
"##,
        )
        .expect("write include");

        let root_path = root.join("ui.toml");
        fs::write(
            &root_path,
            r##"
[scene]
width = 800
height = 450
includes = ["src/acetates/space.toml"]

[[acetate]]
id = "root"
x = 1
y = 2
w = 10
h = 20
fill = "#112233"
"##,
        )
        .expect("write root ui");

        let scene = load_scene_from_file(&root_path).expect("load scene");
        assert_eq!(scene.width, 800);
        assert_eq!(scene.height, 450);
        assert_eq!(scene.acetates.len(), 2);

        let _ = fs::remove_file(&root_path);
        let _ = fs::remove_file(&include_path);
        let _ = fs::remove_dir_all(&root);
    }
}
