use super::{Range, Stage};

pub type EffectId = u16;

pub struct Effect {
    pub lift_speed: Range,
    pub stages: &'static [Stage],
}
