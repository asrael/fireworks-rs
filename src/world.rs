use glam::Vec3;

pub struct World {
    pub gravity: Vec3,
    pub spark_cap: usize,
    pub star_cap: usize,
    pub wind: Vec3,
}

impl Default for World {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            spark_cap: 32768,
            star_cap: 4096,
            wind: Vec3::ZERO,
        }
    }
}
