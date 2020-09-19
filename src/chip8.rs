use super::clock::Clock;
use super::cpu::Cpu;
use super::display::Display;
use super::keyboard::KeyboardState;
use super::memory::Memory;
use winit_input_helper::WinitInputHelper;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    keyboard: KeyboardState,
}
const CpuClock: Clock = Clock::new(500);
const TimerClock: Clock = Clock::new(60);

impl Chip8 {
    pub const SCREEN_HEIGHT: u32 = 32;
    pub const SCREEN_WIDTH: u32 = 64;

    pub fn new(program: Box<[u8]>) -> Self {
        let cpu = Cpu::new();
        let mut memory = Memory::new();
        memory.write_chunk(0x200, program);
        let mut display = Display::new();
        display.set_foreground_color(0x44FF55FF);
        display.set_background_color(0x333333FF);

        CpuClock.start_clock();
        TimerClock.start_clock();
        
        Chip8 {
            cpu: cpu,
            memory: memory,
            display: display,
            keyboard: KeyboardState::default(),
        }
    }

    pub fn draw_to_frame(&mut self, frame: &mut [u8], scale_factor: usize) {
        self.display.draw_to_frame(frame, scale_factor);
    }

    pub fn handle_winit_input(&mut self, input: &WinitInputHelper) {
        self.keyboard.handle_winit_input(input);
    }

    pub fn update(&mut self) -> bool {
        let mut redraw = false;

        CpuClock.run(|| {
            let program_change = self.cpu.step(&mut self.memory, &mut self.display, &self.keyboard);

            redraw |= program_change.redraw;
        });
        TimerClock.run(|| {
            self.cpu.tick_timers();
        });

        redraw
    }
}
