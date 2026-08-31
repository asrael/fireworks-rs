const HUES: [u32; 8] = [
    0xFFD296, 0xFF9632, 0xFF463C, 0xFFC85A, 0xC378FF, 0x5ADCFF, 0x82FF96, 0xE6EEFF,
];

pub const SHADES_PER_RAMP: u8 = 8;

pub struct Palette([[u8; 3]; 256]);

impl Palette {
    pub fn new() -> Self {
        let mut table = [[0u8; 3]; 256];

        for (hue, &color) in HUES.iter().enumerate() {
            let [_, r, g, b] = color.to_be_bytes();
            let color = [r, g, b];

            for shade in 0..16usize {
                let mut entry = [0u8; 3];
                let falloff = (1.0 - shade as f32 / SHADES_PER_RAMP as f32)
                    .max(0.0)
                    .powf(1.1);
                let white_mix = match shade {
                    0 => 0.80,
                    1 => 0.40,
                    _ => 0.0,
                };

                for c in 0..3 {
                    let mixed = color[c] as f32 * (1.0 - white_mix) + 255.0 * white_mix;
                    entry[c] = (mixed * falloff).min(255.0) as u8;
                }

                table[hue * 16 + shade] = entry;
            }
        }

        Palette(table)
    }

    pub fn lookup(&self, index: u8) -> [u8; 3] {
        self.0[index as usize]
    }
}
