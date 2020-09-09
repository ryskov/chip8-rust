use super::cpu::Cpu;
use super::display::Display;
use super::memory::Memory;
use minifb::Window;

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    window: Window,
}

impl Chip8 {
    pub fn new(program: Box<[u8]>, window: Window) -> Self {
        let cpu = Cpu::new();
        let mut memory = Memory::new();
        memory.write_chunk(0x200, program);

        Chip8 {
            cpu: cpu,
            memory: memory,
            display: Display::new(),
            window: window,
        }
    }

    pub fn run(&mut self) {
        let mut vbuffer = vec![0; 64 * 32];
        loop {
            let mut y = 0;
            let mut x = 0;
            self.cpu.step(&mut self.memory, &mut self.display);
            for pixel_row in self.display.image.iter() {
                for pixel_column in pixel_row.iter() {
                    let draw_pixel = *pixel_column == 1;
                    println!("{}", (y * 64) + x);
                    if draw_pixel {
                        vbuffer[(y * 64) + x] = 0xFFFFFFFF;
                    } else {
                        vbuffer[(y * 64) + x] = 0x0;
                    }
                    x += 1;
                }
                y += 1;
            }
            self.window.update_with_buffer(&vbuffer, 64, 32).unwrap();
        }
    }
}

use std::io::{stdin, stdout, Read, Write};
fn pause() {
    let mut stdout = stdout();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    print!("{}[2J", 27 as char);
}
