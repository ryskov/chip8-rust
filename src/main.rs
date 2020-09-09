mod chip8;
mod cpu;
mod memory;
mod display;

use chip8::Chip8;
use std::path::Path;
use std::fs::File;
use std::io::Read;

fn main() {
    let program_file = std::env::args().nth(1).unwrap();
    let program = read_bin(program_file);

    let mut chip8 = Chip8::new(program);
    chip8.run();
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();

    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
