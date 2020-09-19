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

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use std::time::{Duration, Instant};
use std::ops::Add;

const SCALE_FACTOR: u32 = 10;

fn main() {
    let program_file = std::env::args().nth(1).unwrap();
    let program = read_bin(program_file);
    let mut chip8 = Chip8::new(program);

    let event_loop = EventLoop::new();
    
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(
            Chip8::SCREEN_WIDTH * SCALE_FACTOR,
            Chip8::SCREEN_HEIGHT * SCALE_FACTOR,
        );
        WindowBuilder::new()
            .with_title("CHIP8")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(Chip8::SCREEN_WIDTH, Chip8::SCREEN_HEIGHT, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::RedrawRequested(_) => {
                chip8.draw_to_frame(pixels.get_frame(), 1 as usize);
                pixels.render().unwrap();
            }
            _ => (),
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            chip8.handle_winit_input(&input);

            if chip8.update() {
                window.request_redraw();
            }
            // *control_flow = ControlFlow::WaitUntil(std::time::Instant::now().add(sleep_duration));
        }

    });
}

fn read_bin<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();

    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
