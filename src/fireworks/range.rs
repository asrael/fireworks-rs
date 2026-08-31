use rand::rngs::SmallRng;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

impl Range {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub const fn at(v: f32) -> Self {
        Self { min: v, max: v }
    }

    pub fn sample(&self, rng: &mut SmallRng) -> f32 {
        self.min + (self.max - self.min) * rng.random::<f32>()
    }
}
