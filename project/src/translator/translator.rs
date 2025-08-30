//! translator module for translator

// TODO: implement
use crate::translator::translate_domain::TranslateDomain;
use crate::translator::translate_codomain::{Snapshot, SnapshotLayer, SnapshotStyle, Color, Rect};

/// Actor funcional que traduce una Scene inmutable en una proyección visual lista para renderizado.
pub struct Translator;

impl Translator {
    pub fn translate(scene: TranslateDomain) -> Snapshot {
        // 🔧 Por ahora generamos una snapshot vacía, o con un ejemplo de capa genérica.

        Snapshot {
            layers: vec![
                SnapshotLayer {
                    z_index: 0,
                    area: Rect {
                        x: 10,
                        y: 10,
                        width: 100,
                        height: 50,
                    },
                    style: SnapshotStyle {
                        fill_color: Color { r: 0.1, g: 0.6, b: 0.9, a: 1.0 },
                        border_color: Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
                        border_thickness: 2.0,
                        text: Some("Ejemplo".into()),
                    },
                }
            ]
        }
    }
}
