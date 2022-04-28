extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use std::time::Duration;

mod chip8;
use chip8::cpu::Cpu;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let surface = Surface::new(64, 32, PixelFormatEnum::RGB888).unwrap();
    let mut texture = Texture::from_surface(&surface, &texture_creator).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cpu = Cpu::new();
    'running: loop {
        cpu.cycle();
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        //TODO Make the following block safe
        // By writing a conversion fn in cpu::draw
        unsafe {
            let _ = texture.update(
                Rect::new(0, 0, 64, 32),
                cpu.display.align_to::<u8>().1,
                64 * 4,
            );
        }

        let _ = canvas.copy(&texture, None, Rect::new(0, 0, 64, 32));
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
