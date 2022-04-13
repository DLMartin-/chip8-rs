// mod chip8;

// fn main() {
// println!("Hello World");
// }

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
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

    let mut surface = Surface::new(64, 32, PixelFormatEnum::RGB332).unwrap();
    let mut texture = Texture::from_surface(&surface, &texture_creator).unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut cpu = Cpu::new();
    'running: loop {
        cpu.cycle();
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(8, 64, 255 - i));
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

        //texture.update(None, &cpu.display, 1);
        //canvas.copy(&texture, None, None);
        // The rest of the game loop goes here...

        //texture.update(None, &cpu.display, 32);
        //canvas.copy(&texture, None, None);
        /*
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(color(col));
                let _ = self.canvas.fill_rect(Rect::new(
                    x as i32,
                    y as i32,
                    SCALE_FACTOR,
                    SCALE_FACTOR,
                ));
            }
        }
        */
        // for y in 0..32 {
        // for x in 0..64 {
        // let pixel = &cpu.display
        // canvas.fill_rect(Rect::new(x, y, 1, 1));
        // }
        // }

        // let mut disp: [u8; 64 * 32] = [0; 64 * 32];
        // let mut r = 0;
        // for outer in &cpu.display {
        //     &disp[r..r + 64].copy_from_slice(outer);
        //     r += 64;
        // }
        // texture.update(None, &disp, 32);
        // canvas.copy(&texture, None, None);

        //pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        //    for (y, row) in pixels.iter().enumerate() {
        //        for (x, &col) in row.iter().enumerate() {
        //            let x = (x as u32) * SCALE_FACTOR;
        //            let y = (y as u32) * SCALE_FACTOR;

        //            self.canvas.set_draw_color(color(col));
        //            let _ = self.canvas
        //                .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
        //        }
        //    }
        //    self.canvas.present();
        //}

        /*
        for (y, row) in cpu.display.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                canvas.set_draw_color(color(col));
                let _ = canvas.fill_rect(Rect::new(x as i32 + 100, y as i32 + 100, 1, 1));
            }
        }
        */

        let mut count = 0;
        for (index, pixel) in cpu.display.iter().enumerate() {
            let y = (index / 64);
            let x = (index % 64);

            count += 1;
            canvas.set_draw_color(color(*pixel));
            let _ = canvas.fill_rect(Rect::new(x as i32 + 100, y as i32 + 100, 1, 1));
        }

        println!("{}", count);

        for y in 0..32 {
            for x in 0..64 {
                let pixel = cpu.display[(64 * y) + x];
                canvas.set_draw_color(color(pixel));
                let _ = canvas.fill_rect(Rect::new(x as i32 + 200, y as i32 + 100, 1, 1));
            }
        }

        /*for x in 0..64 {
            for y in 0..32 {
                let pixel = cpu.display[(x * 32) + y];

                canvas.set_draw_color(color(pixel));
                let _ = canvas.fill_rect(Rect::new(x as i32 + 100, y as i32 + 100, 1, 1));
            }
        }*/

        canvas.present();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}
