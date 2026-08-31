use super::EffectId;

use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Star {
    pub age: f32,
    pub effect: EffectId,
    pub life: f32,
    pub pos: Vec3,
    pub seed: u64,
    pub stage: u8,
    pub trail_acc: f32,
    pub vel: Vec3,
}

impl Star {
    pub fn t(&self) -> f32 {
        (self.age / self.life.max(1e-6)).min(1.0)
    }
}
