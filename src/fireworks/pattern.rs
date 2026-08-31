use super::{Basis, EmitCtx, SampleDir};

use core::f32::consts::TAU;

use glam::Vec3;
use rand::rngs::SmallRng;

#[derive(Clone, Copy)]
pub enum Pattern {
    Cone {
        angle: f32,
        jitter: f32,
    },
    Crossette {
        arms: u16,
        forward: f32,
        jitter: f32,
    },
    Ring {
        jitter: f32,
    },
    Sphere,
    Spokes {
        cone: f32,
        spokes: u16,
        spread: f32,
    },
}

impl Pattern {
    pub fn direction(&self, rng: &mut SmallRng, ctx: &EmitCtx, basis: &Basis) -> Vec3 {
        match *self {
            Pattern::Cone { angle, jitter: j } => {
                let d = basis.apply(rng.unit_cap(angle.cos()));

                Self::jitter(d, j, rng)
            }

            Pattern::Crossette {
                arms,
                forward,
                jitter: j,
            } => {
                let a = ctx.index as f32 / arms.max(1) as f32 * TAU;
                let flat = basis.azimuth(a);
                let d = flat.lerp(basis.w, forward).normalize_or(flat);

                Self::jitter(d, j, rng)
            }

            Pattern::Sphere => rng.unit_sphere(),

            Pattern::Spokes {
                cone,
                spokes,
                spread,
            } => {
                let n = spokes.max(1);
                let k = ctx.index % n;
                let az = k as f32 / n as f32 * TAU;
                let (sc, cc) = (cone.sin(), cone.cos());
                let d = basis.apply(Vec3::new(sc * az.cos(), sc * az.sin(), cc));

                Self::jitter(d, spread, rng)
            }

            Pattern::Ring { jitter: j } => {
                let a = ctx.index as f32 / ctx.count.max(1) as f32 * TAU;

                Self::jitter(basis.azimuth(a), j, rng)
            }
        }
    }

    fn jitter(dir: Vec3, amount: f32, rng: &mut SmallRng) -> Vec3 {
        if amount <= 0.0 {
            dir
        } else {
            (dir + rng.unit_sphere() * amount).normalize_or(dir)
        }
    }
}
