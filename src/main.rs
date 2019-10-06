extern crate sdl2;
mod cpu;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

pub fn main() {
    println!("Hello, world!");
    let cpu = cpu::CPU {
        memory: [123; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
        cpu_register: [0; 16],
        opcode: 0,
        index: 0,
    };
    draw_something();
}

pub fn draw_something() {
    println!("Hello SDL!");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let mut rects: [Rect; 2] = [Rect::new(10, 10, 100, 150), Rect::new(10, 300, 100, 150)];
    canvas.fill_rects(&rects);

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
//    let mut i = 0;
    'running: loop {
//        i = (i + 1) % 255;
//        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
//        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

//        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
