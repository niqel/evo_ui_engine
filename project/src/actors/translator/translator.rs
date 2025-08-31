use crate::core::{Snapshot, SnapshotLayer, SnapshotStyle, TranslateDomain, Rect as SnapRect};

pub struct Translator;

impl Translator {
    pub fn translate(scene: TranslateDomain) -> Snapshot {
        let mut layers: Vec<SnapshotLayer> = scene
            .acetates
            .iter()
            .map(|a| {
                let design = a.design();
                let area = SnapRect {
                    x: design.area.x,
                    y: design.area.y,
                    width: design.area.width,
                    height: design.area.height,
                };
                SnapshotLayer {
                    z_index: a.z_index(),
                    area,
                    style: SnapshotStyle {
                        fill_color: design.background,
                        border_color: design.border,
                        border_thickness: design.border_thickness,
                        text: design.text,
                    },
                }
            })
            .collect();

        layers.sort_by_key(|l| l.z_index);
        Snapshot { layers }
    }
}
