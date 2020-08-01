use std::cell::Cell;
use std::env;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use chip8_core::cart::Cartridge;
use chip8_core::cpu::{MAX_MEMORY_SIZE, ProcState};
use chip8_core::font::FONT_SPRITES;
use std::time::Duration;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use chip8_core::{SCREEN_WIDTH, SCREEN_HEIGHT};

const SCALING_FACTOR: u32 = 12;

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let delay = args[2].parse::<u64>().unwrap();
    let io = Rc::new(Cell::new(Option::None));
    let mut state = start_emu(&filename, io.clone());

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let title = Path::new(&filename).file_name().and_then(|f| f.to_str()).unwrap_or("Unknown");
    let width = (SCREEN_WIDTH as u32) * SCALING_FACTOR;
    let height = (SCREEN_HEIGHT as u32) * SCALING_FACTOR;
    let mut window = video_subsystem.window(title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;

    'main: loop {
        poll_keys(&mut events, io.clone());

        // Audio

        let opcode = state.fetch_and_decode_opcode();
        state.execute_opcode(opcode);

        state.clock_tick(1);

        draw_screen(&mut window, &mut events, &state.video_buffer)?;

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("Quitting...");
                    break 'main;
                }
                _ => ()
            }
        }

        println!("ProcState: {}, Instruction: {}, KeyPress: {:?}", &state, &opcode, io.clone().get());
        std::thread::sleep(Duration::from_millis(delay));
    }

    Ok(())
}

fn draw_screen(window: &mut Window, events: &mut EventPump, video_buffer: &[u64]) -> Result<(), String> {
    let mut surface = window.surface(events)?;
    for row in 0 .. SCREEN_HEIGHT {
        for column in 0 .. SCREEN_WIDTH {
            let xpos = (column as i32) * (SCALING_FACTOR as i32);
            let ypos = (row as i32) * (SCALING_FACTOR as i32);

            let pixel = Rect::new(xpos, ypos, SCALING_FACTOR, SCALING_FACTOR);
            let pixel_on = (video_buffer[row] >> ((SCREEN_WIDTH as u64) - 1 - (column as u64))) & 0x1 == 0x1;
            let color = if pixel_on {
                Color::RGB(255, 255, 255)
            } else {
                Color::RGB(0, 0,0)
            };

            surface.fill_rect(pixel, color)?
        }
    }

    surface.finish()?;
    Ok(())
}

fn poll_keys(events: &mut EventPump, io: Rc<Cell<Option<u8>>>) {
    let key = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).last();

    match key {
        None => io.set(Option::None),
        Some(Keycode::Num1) => io.set(Option::Some(0x1)),
        Some(Keycode::Num2) => io.set(Option::Some(0x2)),
        Some(Keycode::Num3) => io.set(Option::Some(0x3)),
        Some(Keycode::Num4) => io.set(Option::Some(0xC)),
        Some(Keycode::Q) => io.set(Option::Some(0x4)),
        Some(Keycode::W) => io.set(Option::Some(0x5)),
        Some(Keycode::E) => io.set(Option::Some(0x6)),
        Some(Keycode::R) => io.set(Option::Some(0xD)),
        Some(Keycode::A) => io.set(Option::Some(0x7)),
        Some(Keycode::S) => io.set(Option::Some(0x8)),
        Some(Keycode::D) => io.set(Option::Some(0x9)),
        Some(Keycode::F) => io.set(Option::Some(0xE)),
        Some(Keycode::Z) => io.set(Option::Some(0xA)),
        Some(Keycode::X) => io.set(Option::Some(0x0)),
        Some(Keycode::C) => io.set(Option::Some(0xB)),
        Some(Keycode::V) => io.set(Option::Some(0xF)),
        Some(_) => io.set(Option::None)
    }
}

fn start_emu(filename: &String, io: Rc<Cell<Option<u8>>>) -> ProcState {
    let mut f = File::open(&Path::new(filename)).expect(&format!("File not found: {}", filename));

    let cart = Cartridge::load(&mut f);
    println!("Cart Loaded. Size={} bytes", cart.size);

    let mut mem = [0x0; MAX_MEMORY_SIZE];

    // Load cartridge into memory
    for i in 0 .. cart.size {
        mem[i + 0x200] = cart.buffer[i];
    }

    // Load the font sprites into memory
    for i in 0 .. FONT_SPRITES.len() {
        mem[i] = FONT_SPRITES[i];
    }

    ProcState::new(mem, io)
}