#[derive(Debug, Default)]
pub struct Display {
    pub framebuffer: Box<Vec<u32>>,
    foreground_color: u32,
    background_color: u32,
}

impl Display {
    pub fn new() -> Self {
        Display {
            framebuffer: Box::new(vec![0; 64 * 32]),
            foreground_color: 0xFFFFFFFF,
            background_color: 0x0,
        }
    }

    pub fn set_foreground_color(&mut self, foreground_color: u32) {
        self.foreground_color = foreground_color;
    }

    pub fn set_background_color(&mut self, background_color: u32) {
        self.background_color = background_color;
        self.clear();
    }

    pub fn clear(&mut self) {
        self.framebuffer = Box::new(vec![self.background_color; 64 * 32]);
    }

    pub fn draw(&mut self, x: u8, y: u8, bytes: Vec<u8>, v_flag: &mut bool) {
        for byte_pos in 0..bytes.len() {
            let y = (y as usize + byte_pos) % 32;
            for bit_pos in 0..8 {
                let x = (x as usize + bit_pos) % 64;
                let buffer_pos = (y * 64) + x as usize;
                let draw = (bytes[byte_pos] >> (7 - bit_pos)) & 1;
                let currently_active = self.framebuffer[buffer_pos] != self.background_color;

                if draw == 1 && currently_active {
                    *v_flag = true;
                }
                let new_state = (if currently_active { 1 } else { 0 }) ^ draw;

                self.framebuffer[buffer_pos] = if new_state == 1 {
                    self.foreground_color
                } else {
                    self.background_color
                };
            }
        }
    }

    pub fn draw_to_frame(&self, frame: &mut [u8], scale_factor: usize) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let y = i / (64 * scale_factor);
            let x = i % (64 * scale_factor);

            let downscaled_y = y / scale_factor;
            let downscaled_x = x / scale_factor;
            let source = self.framebuffer[downscaled_y * 64 + downscaled_x];
            let rgba = [
                ((source >> 24) & 0xFF) as u8, // R
                ((source >> 16) & 0xFF) as u8, // G
                ((source >> 8) & 0xFF) as u8,  // B
                (source & 0xFF) as u8,         // A
            ];

            pixel.copy_from_slice(&rgba);
        }
    }
}
