use std::fmt::Debug;

use super::rect::Rect;
use super::acetate_io::AcetateIO;
use super::scene_info::SceneInfo;
use super::design::AcetateDesign;

use crate::contracts::event::Event;

pub trait Acetate: Send + Sync + Debug {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn z_index(&self) -> i32;
    fn area(&self) -> Rect;
    fn subscriptions(&self) -> Vec<Event>;

    fn react(&self, event: &Event, scene: &SceneInfo) -> Option<Box<dyn Acetate>>;
    fn perceive(&self, scene: &SceneInfo) -> SceneInfo;
    fn output(&self) -> AcetateIO;
    fn design(&self) -> AcetateDesign;
    fn clone_box(&self) -> Box<dyn Acetate>;
}

impl Clone for Box<dyn Acetate> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
