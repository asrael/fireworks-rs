use super::{Burst, Motion, Ramp, Range, Strobe, Trail, LIFT_RAMP, LIFT_TRAIL};

#[derive(Clone, Copy)]
pub struct Stage {
    pub drag: f32,
    pub gravity: f32,
    pub life: Range,
    pub motion: Motion,
    pub ramp: Ramp,
    pub strobe: Option<Strobe>,
    pub terminal: &'static [Burst],
    pub trail: Option<Trail>,
}

impl Stage {
    pub const fn lift(terminal: &'static [Burst]) -> Self {
        Stage::star(LIFT_RAMP)
            .burn(1.5, 1.7)
            .drag(0.35)
            .trail(LIFT_TRAIL)
            .terminal(terminal)
    }

    pub const fn mine(terminal: &'static [Burst]) -> Self {
        Stage::star(LIFT_RAMP).burn(0.04, 0.06).terminal(terminal)
    }

    pub const fn star(ramp: Ramp) -> Self {
        Self {
            drag: 0.0,
            gravity: 1.0,
            life: Range::at(1.0),
            motion: Motion::Ballistic,
            ramp,
            strobe: None,
            terminal: &[],
            trail: None,
        }
    }

    pub const fn burn(mut self, min: f32, max: f32) -> Self {
        self.life = Range::new(min, max);
        self
    }

    pub const fn drag(mut self, drag: f32) -> Self {
        self.drag = drag;
        self
    }

    pub const fn fuse(self, min: f32, max: f32) -> Self {
        self.burn(min, max)
    }

    pub const fn gravity(mut self, gravity: f32) -> Self {
        self.gravity = gravity;
        self
    }

    pub const fn motion(mut self, motion: Motion) -> Self {
        self.motion = motion;
        self
    }

    pub const fn strobe(mut self, hz: f32, lit_frac: f32) -> Self {
        self.strobe = Some(Strobe { hz, lit_frac });
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
}
