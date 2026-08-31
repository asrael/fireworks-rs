use super::{Axis, Pattern, Range, Target};

#[derive(Clone, Copy)]
pub struct Burst {
    pub axis: Axis,
    pub child: Target,
    pub count: (u16, u16),
    pub inherit: f32,
    pub offset: f32,
    pub pattern: Pattern,
    pub speed: Range,
}

impl Burst {
    pub const fn sphere(count: (u16, u16), speed: Range, child: Target) -> Self {
        Self {
            axis: Axis::Velocity,
            child,
            count,
            inherit: 0.15,
            offset: 0.2,
            pattern: Pattern::Sphere,
            speed,
        }
    }
}
