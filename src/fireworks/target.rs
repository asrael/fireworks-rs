use super::EffectId;

#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub effect: EffectId,
    pub stage: u8,
}

impl Target {
    pub const fn new(effect: EffectId, stage: u8) -> Self {
        Self { effect, stage }
    }
}
