#[derive(Debug, Default)]
pub struct Display {
    pub framebuffer: Box<Vec<u32>>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            framebuffer: Box::new(vec![0; 64 * 32]),
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer = Box::new(vec![0; 64 * 32]);
    }

    pub fn draw(&mut self, x: u8, y: u8, bytes: Vec<u8>, v_flag: &mut bool) {
        for byte_pos in 0..bytes.len() {
            let y = (y as usize + byte_pos) % 32;
            for bit_pos in 0..8 {
                let x = (x as usize + bit_pos) % 64;
                let buffer_pos = (y * 64) + x as usize;
                let draw = (bytes[byte_pos] >> (7 - bit_pos)) & 1;

                if draw == 1 && self.framebuffer[buffer_pos] != 0 {
                    *v_flag = true;
                }
                let currently_active = if self.framebuffer[buffer_pos] != 0 {
                    1
                } else {
                    0
                } as u8;
                let new_state = currently_active ^ draw;

                self.framebuffer[buffer_pos] = if new_state == 1 { 0x44FFFF00 } else { 0 };
            }
        }
    }

    pub fn draw_to_frame(&self, frame: &mut [u8], scale_factor: usize) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % 64;
            let y = i / 64;

            let downscaled_y = y / scale_factor;
            let downscaled_x = x / scale_factor;
            let source = self.framebuffer[downscaled_y * 64 + downscaled_x];
            
            let rgba = [
                (source >> 24 & 0xF) as u8, // R
                (source >> 16 & 0xF) as u8, // G
                (source >> 8 & 0xF) as u8,  // B
                (source & 0xF) as u8,       // A
            ];

            pixel.copy_from_slice(&rgba);
        }
    }
}
