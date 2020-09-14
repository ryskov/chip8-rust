use super::clock::Clock;
use super::cpu::Cpu;
use super::display::Display;
use super::keyboard::KeyboardState;
use super::memory::Memory;

use minifb::Window;
use minifb::{Key, KeyRepeat};


use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;
const SCALE_FACTOR: u32 = 10;

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

    fn draw_to_frame(&mut self, frame: &mut [u8]) {

    }

    pub fn run(&mut self) {
        // let event_loop = EventLoop::new();
        // let mut input = WinitInputHelper::new();
        // let window = {
        //     let size = LogicalSize::new(CHIP8_WIDTH * SCALE_FACTOR, CHIP8_HEIGHT * SCALE_FACTOR);
        //     WindowBuilder::new()
        //         .with_title("CHIP8")
        //         .with_inner_size(size)
        //         .with_min_inner_size(size)
        //         .build(&event_loop)
        //         .unwrap()
        // };
        // let mut pixels = {
        //     let window_size = window.inner_size();
        //     let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        //     Pixels::new(CHIP8_WIDTH * SCALE_FACTOR, CHIP8_HEIGHT * SCALE_FACTOR, surface_texture).unwrap()
        // };
        // self.draw_to_frame(pixels.get_frame());
        
        // event_loop.run(move |event, _, control_flow| {
        //     if let Event::RedrawRequested(_) = event {
        //         self.display.draw_to_frame(pixels.get_frame(), SCALE_FACTOR as usize);

        //         // self.display.draw_to_frame(&'a mut [0, 1], SCALE_FACTOR as usize);
        //         // self.display.draw_to_frame(pixels.get_frame(), SCALE_FACTOR as usize);
        //     //     pixels.render().unwrap();
        //     //     // if pixels.render().is_err() {
        //     //     //     *control_flow = ControlFlow::Exit;
        //     //     //     return;
        //         // }
        //     }
        // });

        
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
