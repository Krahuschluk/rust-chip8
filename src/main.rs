extern crate sdl2;
mod cpu;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::Read;

pub fn main() {
    println!("Hello, world!");
    let mut memory = [0u8; 4096];
    load_fontset(&mut memory);
    load_rom(&mut memory);

    println!("Current memory snapshot: {:?}", &memory[..4096]);

    let mut cpu = cpu::CPU {
        memory: memory,
        program_counter: 0x200,
        stack: [0; 16],
        stack_pointer: 0,
        cpu_register: [0; 16],
        opcode: 0,
        index: 0,
        pixels: [0; 2048],
        key: [false; 16],
        draw_flag: false,
        delay_timer: 0,
        sound_timer: 0,
    };


    let black = Color::RGB(0, 0, 0);
    let white = Color::RGB(255, 255, 255);

    // Initialize the graphics. Clean this up when I understand wth is going on
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 3*640, 3*320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Clear screen
    canvas.set_draw_color(black);
    canvas.clear();

    // Example of white pixels
    canvas.set_draw_color(white);
    canvas.present();

    // Event loop using SDL
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    cpu.key[0] = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                    cpu.key[0] = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    cpu.key[1] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                    cpu.key[2] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                    cpu.key[3] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    cpu.key[4] = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                    cpu.key[4] = false;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    cpu.key[5] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    cpu.key[6] = true;
                },
                Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                    cpu.key[6] = false;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    cpu.key[7] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    cpu.key[8] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    cpu.key[9] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    cpu.key[10] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    cpu.key[11] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    cpu.key[12] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    cpu.key[13] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    cpu.key[14] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    cpu.key[15] = true;
                },


                _ => {}
            }
        }

        cpu.emulate_cycle();
        let pixels = cpu.pixels;

        if cpu.draw_flag {
//            println!("Drawing!");
//            println!("{:?}", &pixels[..2048]);

            canvas.set_draw_color(black);
            canvas.clear();

            // Determine which pixels to fill in
            canvas.set_draw_color(white);

            let pixel_width : u32 = 30;
            for i in 0..pixels.len() {
                // 64 x 32 pixels. Probably 2:1 and not 1:2
                if pixels[i] == 1 {
                    let x_pos = (i % 64) as i32;
                    let y_pos = (i / 64) as i32;
//                    println!("i {}", i);
//                    println!("x pos {}", x_pos);
//                    println!("y pos {}", y_pos);
                    canvas.fill_rect(Rect::new(x_pos * 30, y_pos * 30, pixel_width, pixel_width));
                }
            }

            canvas.present();
        }

        // Clean up the key inputs after each cycle
        //cpu.clear_keys();

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        ::std::thread::sleep(Duration::new(0, 1_000u32 / 60));
    }
}


pub fn draw_something() {
    println!("Hello SDL!");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 3*640, 3*320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let mut rects: [Rect; 2] = [
        Rect::new(30, 30, 30, 150),
        Rect::new(30, 300, 120, 150)
    ];
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
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}


fn load_fontset(memory: &mut [u8; 4096]) {
    // Copied from the tutorial
    let font_set: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    for i in 0..font_set.len() {
        memory[i] = font_set[i];
    }

}



fn load_rom(memory: &mut [u8; 4096]) {
    // memory: [u8; 4096]
    let rom_start_index = 0x200;

    println!("Attempting to read ROM");

    let mut f = File::open("roms/BRIX").unwrap();

    //let mut buffer =  [0u8; 4096];

    // Read the entire rom
    //let n = f.read(&mut memory[200..]).unwrap();
    f.read(&mut memory[0x200..]).unwrap();

    //println!("Current memory snapshot: {:?}", &memory[..4096]);

    println!("Still alive");
}