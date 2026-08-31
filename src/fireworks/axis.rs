use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    Velocity,
    World(Vec3),
}
