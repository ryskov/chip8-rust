use super::clock::Clock;
use super::cpu::Cpu;
use super::display::Display;
use super::keyboard::KeyboardState;
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
            cpu,
            memory,
            display: Display::new(),
            window,
        }
    }

    pub fn run(&mut self) {
        let mut cpu_clock = Clock::new(500);
        let mut timer_clock = Clock::new(60);
        let mut keyboard_poll_clock = Clock::new(10);
        let mut keyboard_state = KeyboardState::get_keyoard_state(&mut self.window);

        loop {
            if keyboard_poll_clock.tick() {
                keyboard_state = KeyboardState::get_keyoard_state(&mut self.window);
            }

            if cpu_clock.tick() {
                let program_change =
                    self.cpu
                        .step(&mut self.memory, &mut self.display, &keyboard_state);
                if program_change.redraw == true {
                    self.window
                        .update_with_buffer(&self.display.framebuffer, 64, 32)
                        .unwrap();
                }
            }

            if timer_clock.tick() {
                self.cpu.tick_timers();
            }

            Clock::sleep_until_next_tick(vec![&keyboard_poll_clock, &cpu_clock, &timer_clock]);
        }
    }
}
