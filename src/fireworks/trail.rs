use super::{Ramp, Range};

#[derive(Clone, Copy)]
pub struct Trail {
    pub drag: f32,
    pub gravity: f32,
    pub inherit: f32,
    pub life: Range,
    pub ramp: Ramp,
    pub rate: f32,
    pub spread: f32,
}
