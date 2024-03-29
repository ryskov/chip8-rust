mod chip8;
mod clock;
mod cpu;
mod display;
mod keyboard;
mod memory;
mod opcode;

use chip8::Chip8;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use minifb::{Window, WindowOptions};
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let program_file = std::env::args().nth(1).unwrap();
    let program = read_bin(program_file);
    let mut window_options = WindowOptions::default();
    window_options.scale = minifb::Scale::X16;
    let mut window = Window::new("CHIP8", WIDTH, HEIGHT, window_options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(None);
    let mut chip8 = Chip8::new(program, window);
    chip8.run();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();

    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
