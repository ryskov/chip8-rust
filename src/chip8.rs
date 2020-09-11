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
        loop {
            let program_change = self.cpu.step(&mut self.memory, &mut self.display);
            if program_change.redraw {
                self.window
                    .update_with_buffer(&self.display.framebuffer, 64, 32)
                    .unwrap();
            }
        }
    }
}
