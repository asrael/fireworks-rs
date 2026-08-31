use super::{Axis, Burst, Effect, EffectId, Motion, Pattern, Ramp, Range, Stage, Target, Trail};

use glam::Vec3;

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
        Stage::star(GOLD_RAMP)
            .burn(3.4, 4.4)
            .drag(0.3)
            .trail(Trail {
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
        Stage::star(PURPLE_RAMP)
            .burn(1.4, 1.9)
            .drag(1.1)
            .trail(Trail {
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
        Stage::mine(&[Burst {
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
        Stage::star(ORANGE_RAMP)
            .burn(1.5, 2.1)
            .drag(0.55)
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
        Stage::star(RED_RAMP)
            .burn(0.7, 0.85)
            .drag(0.5)
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
        Stage::star(WHITE_RAMP)
            .burn(0.5, 0.7)
            .drag(1.0)
            .terminal(&[Burst {
                axis: Axis::Velocity,
                child: Target::new(FX_CROSSETTE, 3),
                count: (6, 10),
                inherit: 0.4,
                offset: 0.0,
                pattern: Pattern::Sphere,
                speed: Range::new(1.0, 2.2),
            }]),
        Stage::star(WHITE_RAMP)
            .burn(0.15, 0.3)
            .drag(3.0)
            .gravity(0.8),
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
        Stage::star(GOLD_RAMP)
            .burn(0.7, 2.0)
            .drag(1.5)
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
        Stage::star(WHITE_RAMP)
            .burn(0.06, 0.16)
            .drag(4.0)
            .gravity(0.9),
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
        Stage::star(PURPLE_RAMP)
            .burn(1.1, 1.7)
            .drag(0.9)
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
        Stage::star(GOLD_RAMP)
            .burn(1.7, 2.2)
            .drag(0.55)
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
        Stage::star(GREEN_RAMP).burn(1.2, 1.7).drag(1.4),
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
        Stage::star(SILVER_RAMP).burn(1.3, 1.8).drag(1.3),
        Stage::star(RED_RAMP).burn(1.5, 2.0).drag(1.3),
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
        Stage::star(CYAN_RAMP).burn(1.4, 1.8).drag(1.3),
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
        Stage::star(WHITE_RAMP)
            .burn(2.6, 3.6)
            .drag(2.0)
            .gravity(0.55)
            .strobe(11.0, 0.3),
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
        Stage::star(SILVER_RAMP)
            .burn(1.3, 1.8)
            .drag(0.7)
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
        Stage::lift(&[Burst::sphere(
            (70, 90),
            Range::new(6.0, 8.0),
            Target::new(FX_WILLOW, 1),
        )])
        .fuse(1.7, 1.9),
        Stage::star(GOLD_RAMP)
            .burn(2.6, 3.4)
            .drag(0.45)
            .trail(Trail {
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

pub const CATALOG: &[Effect] = &[
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

const ORANGE_RAMP: Ramp = Ramp::new(1, 8);
const RED_RAMP: Ramp = Ramp::new(2, 8);
const GOLD_RAMP: Ramp = Ramp::new(3, 8);
const PURPLE_RAMP: Ramp = Ramp::new(4, 8);
const CYAN_RAMP: Ramp = Ramp::new(5, 8);
const GREEN_RAMP: Ramp = Ramp::new(6, 8);
const SILVER_RAMP: Ramp = Ramp::new(7, 6);
const WHITE_RAMP: Ramp = Ramp::new(7, 8);
pub const LIFT_RAMP: Ramp = GOLD_RAMP;

const SHELL_LIFT: Range = Range::new(40.0, 44.0);

pub const LIFT_TRAIL: Trail = Trail {
    drag: 2.0,
    gravity: 0.3,
    inherit: 0.1,
    life: Range::new(0.15, 0.4),
    ramp: LIFT_RAMP,
    rate: 120.0,
    spread: 0.7,
};
