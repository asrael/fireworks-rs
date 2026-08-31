mod palette;

use palette::Palette;

use glam::Vec3;

use crate::fireworks::Fireworks;

pub const WIDTH: u32 = 960;
pub const HEIGHT: u32 = 640;

const FOCAL_LENGTH: f32 = 430.0;
const HORIZON_Y: f32 = HEIGHT as f32 - 20.0;
const PERSISTENCE: u16 = 205;
const PIXELS_PER_METER: f32 = 9.0;
const SKYLINE_H: usize = 18;

pub struct Renderer {
    palette: Palette,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            palette: Palette::new(),
        }
    }

    pub fn draw(&self, fireworks: &Fireworks, frame: &mut [u8]) {
        let sky_bytes = (HEIGHT as usize - SKYLINE_H) * WIDTH as usize * 4;
        self.fade(&mut frame[..sky_bytes]);

        fireworks.visit(&mut |pos, index, is_star| {
            self.plot(frame, pos, index, is_star);
        });

        self.draw_skyline(frame);
    }

    pub fn unproject(sx: f32, sy: f32) -> Vec3 {
        Vec3::new(
            (sx - WIDTH as f32 * 0.5) / PIXELS_PER_METER,
            (HORIZON_Y - sy) / PIXELS_PER_METER,
            0.0,
        )
    }

    fn add_rgb(frame: &mut [u8], offset: usize, r: u8, g: u8, b: u8) {
        frame[offset] = frame[offset].saturating_add(r);
        frame[offset + 1] = frame[offset + 1].saturating_add(g);
        frame[offset + 2] = frame[offset + 2].saturating_add(b);
    }

    fn draw_skyline(&self, frame: &mut [u8]) {
        let ground = HEIGHT as usize - SKYLINE_H;

        for y in ground..HEIGHT as usize {
            for x in 0..WIDTH as usize {
                Self::set_rgb(frame, (y * WIDTH as usize + x) * 4, 10, 12, 26);
            }
        }

        for i in 0..36usize {
            let bx = i * 27 + (i % 3) * 4;
            let bh = 10 + (i * 37) % 26;

            for y in ground.saturating_sub(bh)..ground {
                for x in bx..(bx + 21).min(WIDTH as usize) {
                    Self::set_rgb(frame, (y * WIDTH as usize + x) * 4, 17, 20, 42);
                }
            }
        }
    }

    fn fade(&self, frame: &mut [u8]) {
        for pixel in frame.as_chunks_mut::<4>().0 {
            for c in &mut pixel[..3] {
                let faded = (*c as u16 * PERSISTENCE) >> 8;
                *c = (faded as u8).saturating_sub(1);
            }

            pixel[3] = 0xff;
        }
    }

    fn plot(&self, frame: &mut [u8], pos: Vec3, index: u8, bright: bool) {
        let scale = FOCAL_LENGTH / (FOCAL_LENGTH + pos.z * PIXELS_PER_METER);
        let sx = (WIDTH as f32 * 0.5 + pos.x * PIXELS_PER_METER * scale) as i32;
        let sy = (HORIZON_Y - pos.y * PIXELS_PER_METER * scale) as i32;

        if sx < 1 || sy < 1 || sx >= WIDTH as i32 - 1 || sy >= HEIGHT as i32 - 1 {
            return;
        }

        let [r, g, b] = self.palette.lookup(index);
        let offset = (sy as usize * WIDTH as usize + sx as usize) * 4;
        Self::add_rgb(frame, offset, r, g, b);

        if bright {
            let row = WIDTH as usize * 4;
            for neighbour in [offset - 4, offset + 4, offset - row, offset + row] {
                Self::add_rgb(frame, neighbour, r >> 1, g >> 1, b >> 1);
            }
        }
    }

    fn set_rgb(frame: &mut [u8], offset: usize, r: u8, g: u8, b: u8) {
        frame[offset] = r;
        frame[offset + 1] = g;
        frame[offset + 2] = b;
        frame[offset + 3] = 0xff;
    }
}
