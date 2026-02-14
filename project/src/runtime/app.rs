use std::collections::HashSet;
use std::time::{Duration, Instant};

use crate::contracts::event::{Event, MouseButton};
use crate::contracts::scene::Scene;

#[derive(Debug, Clone)]
pub struct InputState {
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_buttons_down: HashSet<MouseButton>,
    pub keys_down: HashSet<String>,
    pub text_buffer: Option<String>,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_x: 0,
            mouse_y: 0,
            mouse_buttons_down: HashSet::new(),
            keys_down: HashSet::new(),
            text_buffer: None,
            window_width: 1,
            window_height: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FrameContext {
    pub tick_number: u64,
    pub dt: Duration,
    pub timestamp: Instant,
    pub window_width: u32,
    pub window_height: u32,
    pub fps: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct InputWants {
    pub mouse_move: bool,
    pub mouse_buttons: bool,
    pub keyboard: bool,
    pub text_input: bool,
    pub tick: bool,
    pub resize: bool,
}

impl Default for InputWants {
    fn default() -> Self {
        Self {
            mouse_move: false,
            mouse_buttons: false,
            keyboard: false,
            text_input: false,
            tick: false,
            resize: true,
        }
    }
}

pub trait App {
    fn input_wants(&self) -> InputWants {
        InputWants::default()
    }

    fn frame(&mut self, events: &[Event], ctx: &FrameContext, input: &InputState) -> Scene;
}
