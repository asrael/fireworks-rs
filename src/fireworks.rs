mod axis;
mod burst;
mod catalog;
mod effect;
mod motion;
mod pattern;
mod ramp;
mod range;
mod spark;
mod stage;
mod star;
mod strobe;
mod target;
mod trail;

use axis::Axis;
use burst::Burst;
pub use catalog::*;
pub use effect::{Effect, EffectId};
use motion::Motion;
use pattern::Pattern;
use ramp::Ramp;
pub use range::Range;
use spark::Spark;
use stage::Stage;
use star::Star;
use strobe::Strobe;
use target::Target;
use trail::Trail;

use crate::world::World;

use core::f32::consts::TAU;
use std::hash::BuildHasher;

use foldhash::fast::FixedState;
use glam::Vec3;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Fireworks {
    frame: u64,
    lib: &'static [Effect],
    pending: Vec<Star>,
    rng: SmallRng,
    sparks: Vec<Spark>,
    stars: Vec<Star>,
    pub world: World,
}

impl Fireworks {
    pub fn new(lib: &'static [Effect], world: World, seed: u64) -> Self {
        Self {
            frame: 0,
            lib,
            pending: Vec::with_capacity(512),
            rng: SmallRng::seed_from_u64(seed),
            sparks: Vec::with_capacity(world.spark_cap),
            stars: Vec::with_capacity(world.star_cap),
            world,
        }
    }

    pub fn burst(&mut self, effect: EffectId, pos: Vec3) {
        let lift = self.lib[effect as usize].lift_speed.sample(&mut self.rng);
        let parent = Star {
            age: 0.0,
            effect,
            life: 0.0,
            pos,
            seed: self.rng.random(),
            stage: 0,
            trail_acc: 0.0,
            vel: Vec3::new(0.0, lift * 0.3, 0.0),
        };

        for b in self.stage(effect, 0).terminal {
            self.fire_burst(b, &parent);
        }
    }

    pub fn launch(&mut self, effect: EffectId, pos: Vec3, vel: Vec3) {
        let stage = self.stage(effect, 0);
        let life = stage.life.sample(&mut self.rng);
        let seed = self.rng.random();

        self.push_star(Star {
            age: 0.0,
            effect,
            life,
            pos,
            seed,
            stage: 0,
            trail_acc: 0.0,
            vel,
        });
    }

    pub fn update(&mut self, dt: f32) {
        self.frame = self.frame.wrapping_add(1);
        self.update_sparks(dt);
        self.update_stars(dt);
        self.stars.append(&mut self.pending);
    }

    pub fn visit(&self, f: &mut impl FnMut(Vec3, u8, bool)) {
        for s in &self.sparks {
            f(s.pos, s.ramp.sample(s.t()), false);
        }

        for s in &self.stars {
            let stage = self.stage(s.effect, s.stage);

            if let Some(fl) = &stage.strobe {
                if !fl.lit(s.age, s.seed) {
                    continue;
                }
            }

            f(s.pos, stage.ramp.sample(s.t()), true);
        }
    }

    fn drag_and_advance(pos: &mut Vec3, vel: &mut Vec3, drag: f32, dt: f32) {
        *vel *= 1.0 / (1.0 + drag * dt);
        *pos += *vel * dt;
    }

    fn emit_trail(&mut self, trail: &Trail, prev: Vec3, s: &mut Star, dt: f32) {
        s.trail_acc += trail.rate * dt;
        let n = s.trail_acc as u32;

        if n == 0 {
            return;
        }

        s.trail_acc -= n as f32;

        let mut rng = SmallRng::seed_from_u64(Hash::mix(s.seed, self.frame));
        let step = s.pos - prev;

        for k in 0..n {
            if self.sparks.len() >= self.world.spark_cap {
                break;
            }

            let f = (k as f32 + 0.5) / n as f32;
            let life = trail.life.sample(&mut rng);

            self.sparks.push(Spark {
                age: 0.0,
                drag: trail.drag,
                gravity: trail.gravity,
                life,
                pos: prev + step * f,
                ramp: trail.ramp,
                vel: s.vel * trail.inherit + rng.unit_sphere() * trail.spread,
            });
        }
    }

    fn fire_burst(&mut self, b: &'static Burst, parent: &Star) {
        let mut rng = SmallRng::seed_from_u64(Hash::mix(parent.seed, 0x42));
        let axis = match b.axis {
            Axis::Velocity => parent.vel,
            Axis::World(v) => v,
        };
        let basis = Basis::from_axis(axis);
        let count = rng.random_range(b.count.0..=b.count.1);
        let child_stage = self.stage(b.child.effect, b.child.stage);

        for index in 0..count {
            let ctx = EmitCtx { count, index };
            let dir = b.pattern.direction(&mut rng, &ctx, &basis);
            let speed = b.speed.sample(&mut rng);
            let life = child_stage.life.sample(&mut rng);
            let seed = rng.random();

            self.push_star(Star {
                age: 0.0,
                effect: b.child.effect,
                life,
                pos: parent.pos + dir * b.offset,
                seed,
                stage: b.child.stage,
                trail_acc: rng.random(),
                vel: parent.vel * b.inherit + dir * speed,
            });
        }
    }

    fn push_star(&mut self, s: Star) {
        if self.stars.len() + self.pending.len() < self.world.star_cap {
            self.pending.push(s);
        }
    }

    fn stage(&self, effect: EffectId, stage: u8) -> &'static Stage {
        let lib: &'static [Effect] = self.lib;

        &lib[effect as usize].stages[stage as usize]
    }

    fn update_sparks(&mut self, dt: f32) {
        let g = self.world.gravity;
        let wind = self.world.wind;

        self.sparks.retain_mut(|s| {
            s.age += dt;
            if s.age >= s.life {
                return false;
            }
            s.vel += (g * s.gravity + wind) * dt;
            Self::drag_and_advance(&mut s.pos, &mut s.vel, s.drag, dt);
            true
        });
    }

    fn update_stars(&mut self, dt: f32) {
        let g = self.world.gravity;
        let wind = self.world.wind;
        let mut i = 0;

        while i < self.stars.len() {
            let mut s = self.stars[i];
            let stage = self.stage(s.effect, s.stage);
            let prev = s.pos;

            s.age += dt;
            s.vel += (g * stage.gravity + wind) * dt;
            s.vel += stage.motion.apply(s.vel, s.age, s.seed, dt);
            Self::drag_and_advance(&mut s.pos, &mut s.vel, stage.drag, dt);

            if let Some(trail) = &stage.trail {
                self.emit_trail(trail, prev, &mut s, dt);
            }

            if s.age >= s.life {
                for burst in stage.terminal {
                    self.fire_burst(burst, &s);
                }
                self.stars.swap_remove(i);
            } else {
                self.stars[i] = s;
                i += 1;
            }
        }
    }
}

pub struct Basis {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Basis {
    pub fn from_axis(axis: Vec3) -> Self {
        let w = axis.normalize_or(Vec3::Y);
        let (u, v) = w.any_orthonormal_pair();

        Self { u, v, w }
    }

    pub fn apply(&self, local: Vec3) -> Vec3 {
        self.u * local.x + self.v * local.y + self.w * local.z
    }

    pub fn azimuth(&self, angle: f32) -> Vec3 {
        self.u * angle.cos() + self.v * angle.sin()
    }
}

pub struct EmitCtx {
    pub count: u16,
    pub index: u16,
}

struct Hash;

impl Hash {
    fn dir(seed: u64, k: u64) -> Vec3 {
        SmallRng::seed_from_u64(Self::mix(seed, k)).unit_sphere()
    }

    fn mix(seed: u64, k: u64) -> u64 {
        FixedState::with_seed(seed).hash_one(k)
    }

    fn phase(seed: u64, k: u64) -> f32 {
        ((Self::mix(seed, k) >> 40) as f32) * (1.0 / 16_777_216.0)
    }
}

trait SampleDir: Rng + Sized {
    fn unit_cap(&mut self, cos_min: f32) -> Vec3 {
        let z = cos_min + (1.0 - cos_min) * self.random::<f32>();
        let phi = self.random::<f32>() * TAU;
        let r = (1.0 - z * z).max(0.0).sqrt();

        Vec3::new(r * phi.cos(), r * phi.sin(), z)
    }

    fn unit_sphere(&mut self) -> Vec3 {
        self.unit_cap(-1.0)
    }
}

impl<T: Rng> SampleDir for T {}
