#[derive(Clone, Copy, Debug)]
pub struct Ramp {
    pub base: u8,
    pub len: u8,
}

impl Ramp {
    pub const fn new(hue: u8, len: u8) -> Self {
        Self {
            base: hue << 4,
            len,
        }
    }

    pub fn sample(&self, t: f32) -> u8 {
        if self.len == 0 {
            return self.base;
        }

        let i = (t * self.len as f32) as u16;

        self.base.wrapping_add(i.min(self.len as u16 - 1) as u8)
    }
}
