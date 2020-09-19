use super::clock::Clock;
use super::cpu::Cpu;
use super::display::Display;
use super::keyboard::KeyboardState;
use super::memory::Memory;
use winit_input_helper::WinitInputHelper;

const FONT_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];
#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    keyboard: KeyboardState,
    cpu_clock: Clock,
    timer_clock: Clock,
}

impl Chip8 {
    pub const SCREEN_HEIGHT: u32 = 32;
    pub const SCREEN_WIDTH: u32 = 64;

    pub fn new(program: Box<[u8]>) -> Self {
        let font_addr = 0x0;
        let cpu = Cpu::new(font_addr);
        let mut memory = Memory::new();
        memory.write_chunk(font_addr, Box::from(FONT_SPRITES));
        memory.write_chunk(0x200, program);
        let mut display = Display::new();
        display.set_foreground_color(0x44FF55FF);
        display.set_background_color(0x333333FF);

        Chip8 {
            cpu: cpu,
            memory: memory,
            display: display,
            keyboard: KeyboardState::default(),
            cpu_clock: Clock::new(500),
            timer_clock: Clock::new(60),
        }
    }

    pub fn draw_to_frame(&self, frame: &mut [u8], scale_factor: usize) {
        self.display.draw_to_frame(frame, scale_factor);
    }

    pub fn handle_winit_input(&mut self, input: &WinitInputHelper) {
        self.keyboard.handle_winit_input(input);
    }

    pub fn update(&mut self) -> bool {
        let mut redraw = false;

        for _ in 0..self.cpu_clock.consume_ticks() {
            let program_change = self
                .cpu
                .step(&mut self.memory, &mut self.display, &self.keyboard);

            redraw |= program_change.redraw;
        }

        for _ in 0..self.timer_clock.consume_ticks() {
            self.cpu.tick_timers();
        }

        redraw
    }
}
