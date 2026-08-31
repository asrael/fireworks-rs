use super::Ramp;

use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Spark {
    pub age: f32,
    pub drag: f32,
    pub gravity: f32,
    pub life: f32,
    pub pos: Vec3,
    pub ramp: Ramp,
    pub vel: Vec3,
}

impl Spark {
    pub fn t(&self) -> f32 {
        (self.age / self.life.max(1e-6)).min(1.0)
    }
}
