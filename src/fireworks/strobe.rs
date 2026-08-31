use super::Hash;

#[derive(Clone, Copy)]
pub struct Strobe {
    pub hz: f32,
    pub lit_frac: f32,
}

impl Strobe {
    pub fn lit(&self, age: f32, seed: u64) -> bool {
        (Hash::phase(seed, 0x5B) + age * self.hz).fract() < self.lit_frac
    }
}
