use super::cpu::Cpu;
use super::memory::Memory;
use super::display::Display;

#[derive(Default,Debug)]
pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    display: Display
}

impl Chip8 {
    pub fn new(program: Box<[u8]>) -> Self {
        let cpu = Cpu::new();
        let mut memory = Memory::new();
        memory.write_chunk(0x200, program);

        Chip8 {
            cpu: cpu,
            memory: memory,
            display: Display::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.memory, &mut self.display);
            pause();
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