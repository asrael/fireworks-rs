use super::{Burst, Flicker, Motion, Ramp, Range, Trail, LIFT_RAMP, LIFT_TRAIL};

#[derive(Clone, Copy)]
pub struct Stage {
    pub drag: f32,
    pub flicker: Option<Flicker>,
    pub gravity: f32,
    pub life: Range,
    pub motion: Motion,
    pub ramp: Ramp,
    pub terminal: &'static [Burst],
    pub trail: Option<Trail>,
}

impl Stage {
    pub const fn new(life: Range, drag: f32, ramp: Ramp) -> Self {
        Self {
            drag,
            flicker: None,
            gravity: 1.0,
            life,
            motion: Motion::Ballistic,
            ramp,
            terminal: &[],
            trail: None,
        }
    }

    pub const fn flicker(mut self, hz: f32, lit_frac: f32) -> Self {
        self.flicker = Some(Flicker { hz, lit_frac });
        self
    }

    pub const fn gravity(mut self, gravity: f32) -> Self {
        self.gravity = gravity;
        self
    }

    pub const fn motion(mut self, motion: Motion) -> Self {
        self.motion = motion;
        self
    }

    pub const fn terminal(mut self, terminal: &'static [Burst]) -> Self {
        self.terminal = terminal;
        self
    }

    pub const fn trail(mut self, trail: Trail) -> Self {
        self.trail = Some(trail);
        self
    }

    pub const fn lift(terminal: &'static [Burst]) -> Self {
        Stage::new(Range::new(1.5, 1.7), 0.35, LIFT_RAMP)
            .trail(LIFT_TRAIL)
            .terminal(terminal)
    }
}
