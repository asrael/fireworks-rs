use super::{Basis, Hash};

use core::f32::consts::TAU;

use glam::Vec3;

#[derive(Clone, Copy)]
pub enum Motion {
    Ballistic,
    Helix { accel: f32, hz: f32 },
    Thrust { accel: f32 },
    Wander { accel: f32, hz: f32 },
}

impl Motion {
    pub fn apply(&self, vel: Vec3, age: f32, seed: u64, dt: f32) -> Vec3 {
        match *self {
            Motion::Ballistic => Vec3::ZERO,

            Motion::Helix { accel, hz } => {
                let basis = Basis::from_axis(vel);
                let ph = (Hash::phase(seed, 0x51) + age * hz) * TAU;

                basis.azimuth(ph) * (accel * dt)
            }

            Motion::Thrust { accel } => vel.normalize_or(Vec3::Y) * (accel * dt),

            Motion::Wander { accel, hz } => {
                let t = age * hz;
                let k = t as u64;
                let f = t - k as f32;
                let f = f * f * (3.0 - 2.0 * f);
                let a = Hash::dir(seed, k);
                let b = Hash::dir(seed, k + 1);

                a.lerp(b, f).normalize_or(a) * (accel * dt)
            }
        }
    }
}
