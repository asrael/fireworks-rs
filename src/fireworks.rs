mod burst;
mod effect;
mod motion;
mod pattern;
mod spark;
mod stage;
mod star;
mod target;

use burst::Burst;
pub use effect::{Effect, EffectId};
use motion::Motion;
use pattern::Pattern;
use spark::Spark;
use stage::Stage;
use star::Star;
use target::Target;

use crate::world::World;

use core::f32::consts::TAU;
use std::hash::BuildHasher;

use foldhash::fast::FixedState;
use glam::Vec3;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub const FX_BROCADE: EffectId = 0;
pub const FX_CHRYSANTHEMUM: EffectId = 1;
pub const FX_COMET: EffectId = 2;
pub const FX_CROSSETTE: EffectId = 3;
pub const FX_DRAGONS_EGGS: EffectId = 4;
pub const FX_FISH: EffectId = 5;
pub const FX_PALM: EffectId = 6;
pub const FX_PEONY: EffectId = 7;
pub const FX_PISTIL: EffectId = 8;
pub const FX_STROBE: EffectId = 9;
pub const FX_TOURBILLION: EffectId = 10;
pub const FX_WILLOW: EffectId = 11;
pub const FX_RING: EffectId = 12;

pub const BROCADE: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst::sphere(
            (85, 105),
            Range::new(7.0, 9.0),
            Target::new(FX_BROCADE, 1),
        )]),
        Stage::new(Range::new(3.4, 4.4), 0.3, GOLD_RAMP).trail(Trail {
            drag: 1.0,
            gravity: 0.28,
            inherit: 0.04,
            life: Range::new(1.3, 2.1),
            ramp: GOLD_RAMP,
            rate: 150.0,
            spread: 0.2,
        }),
    ],
};

pub const CHRYSANTHEMUM: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst::sphere(
            (170, 210),
            Range::new(9.5, 12.5),
            Target::new(FX_CHRYSANTHEMUM, 1),
        )]),
        Stage::new(Range::new(1.4, 1.9), 1.1, PURPLE_RAMP).trail(Trail {
            drag: 1.4,
            gravity: 0.6,
            inherit: 0.06,
            life: Range::new(0.3, 0.55),
            ramp: PURPLE_RAMP,
            rate: 80.0,
            spread: 0.25,
        }),
    ],
};

pub const COMET: Effect = Effect {
    lift_speed: Range::at(4.0),
    stages: &[
        Stage::new(Range::new(0.04, 0.06), 0.0, LIFT_RAMP).terminal(&[Burst {
            axis: Axis::Velocity,
            child: Target::new(FX_COMET, 1),
            count: (5, 8),
            inherit: 0.25,
            offset: 0.0,
            pattern: Pattern::Cone {
                angle: 0.30,
                jitter: 0.02,
            },
            speed: Range::new(20.0, 27.0),
        }]),
        Stage::new(Range::new(1.5, 2.1), 0.55, ORANGE_RAMP)
            .motion(Motion::Thrust { accel: 3.5 })
            .trail(Trail {
                drag: 1.6,
                gravity: 0.45,
                inherit: 0.05,
                life: Range::new(0.5, 0.9),
                ramp: ORANGE_RAMP,
                rate: 220.0,
                spread: 0.5,
            }),
    ],
};

pub const CROSSETTE: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst::sphere(
            (14, 18),
            Range::new(8.0, 9.5),
            Target::new(FX_CROSSETTE, 1),
        )]),
        Stage::new(Range::new(0.7, 0.85), 0.5, RED_RAMP)
            .trail(Trail {
                drag: 1.5,
                gravity: 0.6,
                inherit: 0.08,
                life: Range::new(0.25, 0.5),
                ramp: RED_RAMP,
                rate: 90.0,
                spread: 0.3,
            })
            .terminal(&[Burst {
                axis: Axis::Velocity,
                child: Target::new(FX_CROSSETTE, 2),
                count: (4, 4),
                inherit: 0.35,
                offset: 0.0,
                pattern: Pattern::Crossette {
                    arms: 4,
                    forward: 0.15,
                    jitter: 0.05,
                },
                speed: Range::new(4.5, 5.5),
            }]),
        Stage::new(Range::new(0.5, 0.7), 1.0, WHITE_RAMP).terminal(&[Burst {
            axis: Axis::Velocity,
            child: Target::new(FX_CROSSETTE, 3),
            count: (6, 10),
            inherit: 0.4,
            offset: 0.0,
            pattern: Pattern::Sphere,
            speed: Range::new(1.0, 2.2),
        }]),
        Stage::new(Range::new(0.15, 0.3), 3.0, WHITE_RAMP).gravity(0.8),
    ],
};

pub const DRAGONS_EGGS: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst::sphere(
            (110, 140),
            Range::new(6.5, 9.5),
            Target::new(FX_DRAGONS_EGGS, 1),
        )]),
        Stage::new(Range::new(0.7, 2.0), 1.5, GOLD_RAMP)
            .trail(Trail {
                drag: 2.0,
                gravity: 0.7,
                inherit: 0.05,
                life: Range::new(0.15, 0.3),
                ramp: GOLD_RAMP,
                rate: 30.0,
                spread: 0.15,
            })
            .terminal(&[Burst {
                axis: Axis::Velocity,
                child: Target::new(FX_DRAGONS_EGGS, 2),
                count: (10, 16),
                inherit: 0.3,
                offset: 0.0,
                pattern: Pattern::Sphere,
                speed: Range::new(1.4, 3.2),
            }]),
        Stage::new(Range::new(0.06, 0.16), 4.0, WHITE_RAMP).gravity(0.9),
    ],
};

pub const FISH: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst::sphere(
            (45, 65),
            Range::new(3.0, 5.0),
            Target::new(FX_FISH, 1),
        )]),
        Stage::new(Range::new(1.1, 1.7), 0.9, PURPLE_RAMP)
            .gravity(0.5)
            .motion(Motion::Wander {
                accel: 34.0,
                hz: 7.0,
            })
            .trail(Trail {
                drag: 2.5,
                gravity: 0.5,
                inherit: 0.05,
                life: Range::new(0.12, 0.28),
                ramp: PURPLE_RAMP,
                rate: 110.0,
                spread: 0.2,
            }),
    ],
};

pub const PALM: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst {
            axis: Axis::Velocity,
            child: Target::new(FX_PALM, 1),
            count: (56, 56),
            inherit: 0.2,
            offset: 0.3,
            pattern: Pattern::Spokes {
                cone: 1.05,
                spokes: 8,
                spread: 0.055,
            },
            speed: Range::new(9.0, 13.0),
        }]),
        Stage::new(Range::new(1.7, 2.2), 0.55, GOLD_RAMP)
            .motion(Motion::Thrust { accel: 2.0 })
            .trail(Trail {
                drag: 1.3,
                gravity: 0.4,
                inherit: 0.05,
                life: Range::new(0.55, 1.0),
                ramp: GOLD_RAMP,
                rate: 190.0,
                spread: 0.3,
            }),
    ],
};

pub const PEONY: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst::sphere(
            (150, 190),
            Range::new(9.0, 12.5),
            Target::new(FX_PEONY, 1),
        )]),
        Stage::new(Range::new(1.2, 1.7), 1.4, GREEN_RAMP),
    ],
};

pub const PISTIL: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[
            Burst::sphere(
                (150, 180),
                Range::new(10.0, 13.0),
                Target::new(FX_PISTIL, 1),
            ),
            Burst::sphere((55, 70), Range::new(3.0, 4.6), Target::new(FX_PISTIL, 2)),
        ]),
        Stage::new(Range::new(1.3, 1.8), 1.3, SILVER_RAMP),
        Stage::new(Range::new(1.5, 2.0), 1.3, RED_RAMP),
    ],
};

pub const RING: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[
            Burst {
                axis: Axis::World(Vec3::Z),
                child: Target::new(FX_RING, 1),
                count: (64, 64),
                inherit: 0.1,
                offset: 0.1,
                pattern: Pattern::Ring { jitter: 0.03 },
                speed: Range::new(10.0, 10.4),
            },
            Burst::sphere((30, 40), Range::new(1.5, 3.0), Target::new(FX_RING, 1)),
        ]),
        Stage::new(Range::new(1.4, 1.8), 1.3, CYAN_RAMP),
    ],
};

pub const STROBE: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst::sphere(
            (90, 120),
            Range::new(5.0, 8.0),
            Target::new(FX_STROBE, 1),
        )]),
        Stage::new(Range::new(2.6, 3.6), 2.0, WHITE_RAMP)
            .gravity(0.55)
            .flicker(11.0, 0.3),
    ],
};

pub const TOURBILLION: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::lift(&[Burst::sphere(
            (9, 14),
            Range::new(7.0, 9.0),
            Target::new(FX_TOURBILLION, 1),
        )]),
        Stage::new(Range::new(1.3, 1.8), 0.7, SILVER_RAMP)
            .motion(Motion::Helix {
                accel: 120.0,
                hz: 5.5,
            })
            .trail(Trail {
                drag: 1.5,
                gravity: 0.5,
                inherit: 0.03,
                life: Range::new(0.35, 0.7),
                ramp: SILVER_RAMP,
                rate: 260.0,
                spread: 0.1,
            }),
    ],
};

pub const WILLOW: Effect = Effect {
    lift_speed: SHELL_LIFT,
    stages: &[
        Stage::new(Range::new(1.7, 1.9), 0.35, LIFT_RAMP)
            .trail(LIFT_TRAIL)
            .terminal(&[Burst::sphere(
                (70, 90),
                Range::new(6.0, 8.0),
                Target::new(FX_WILLOW, 1),
            )]),
        Stage::new(Range::new(2.6, 3.4), 0.45, GOLD_RAMP).trail(Trail {
            drag: 1.2,
            gravity: 0.55,
            inherit: 0.05,
            life: Range::new(0.8, 1.4),
            ramp: GOLD_RAMP,
            rate: 60.0,
            spread: 0.25,
        }),
    ],
};

pub const LIBRARY: &[Effect] = &[
    BROCADE,
    CHRYSANTHEMUM,
    COMET,
    CROSSETTE,
    DRAGONS_EGGS,
    FISH,
    PALM,
    PEONY,
    PISTIL,
    STROBE,
    TOURBILLION,
    WILLOW,
    RING,
];

const ORANGE_RAMP: Ramp = Ramp::new(0x10, 8);
const RED_RAMP: Ramp = Ramp::new(0x20, 8);
const GOLD_RAMP: Ramp = Ramp::new(0x30, 8);
const PURPLE_RAMP: Ramp = Ramp::new(0x40, 8);
const CYAN_RAMP: Ramp = Ramp::new(0x50, 8);
const GREEN_RAMP: Ramp = Ramp::new(0x60, 8);
const SILVER_RAMP: Ramp = Ramp::new(0x70, 6);
const WHITE_RAMP: Ramp = Ramp::new(0x70, 8);
const LIFT_RAMP: Ramp = GOLD_RAMP;

const SHELL_LIFT: Range = Range::new(40.0, 44.0);

const LIFT_TRAIL: Trail = Trail {
    drag: 2.0,
    gravity: 0.3,
    inherit: 0.1,
    life: Range::new(0.15, 0.4),
    ramp: LIFT_RAMP,
    rate: 120.0,
    spread: 0.7,
};

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

            if let Some(fl) = &stage.flicker {
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

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    Velocity,
    World(Vec3),
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

#[derive(Clone, Copy)]
pub struct Flicker {
    pub hz: f32,
    pub lit_frac: f32,
}

impl Flicker {
    fn lit(&self, age: f32, seed: u64) -> bool {
        (Hash::phase(seed, 0x5B) + age * self.hz).fract() < self.lit_frac
    }
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

#[derive(Clone, Copy, Debug)]
pub struct Ramp {
    pub base: u8,
    pub len: u8,
}

impl Ramp {
    pub const fn new(base: u8, len: u8) -> Self {
        Self { base, len }
    }

    pub fn sample(&self, t: f32) -> u8 {
        if self.len == 0 {
            return self.base;
        }

        let i = (t * self.len as f32) as u16;

        self.base.wrapping_add(i.min(self.len as u16 - 1) as u8)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

impl Range {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub const fn at(v: f32) -> Self {
        Self { min: v, max: v }
    }

    pub fn sample(&self, rng: &mut SmallRng) -> f32 {
        self.min + (self.max - self.min) * rng.random::<f32>()
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

#[derive(Clone, Copy)]
pub struct Trail {
    pub drag: f32,
    pub gravity: f32,
    pub inherit: f32,
    pub life: Range,
    pub ramp: Ramp,
    pub rate: f32,
    pub spread: f32,
}
