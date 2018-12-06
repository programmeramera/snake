extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::video::{Window};
use sdl2::render::{Canvas};
use sdl2::rect::{Rect};

const FRAMES_PER_SECOND: u32 = 15;
const SPRITE_SIZE: u32 = 20;
const SCREEN_SIZE: u32 = 20;
const _INITIAL_TAIL: u32 = 5;

fn handle_input(event_pump: &mut sdl2::EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return true;
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                return true;
            },
            _ => { }
        }
    }
    false
}

fn update() {

}

fn draw(i: u8, canvas: &mut Canvas<Window>) -> u8 {
    let nextIndex = (i + 1) % 255;
    canvas.set_draw_color(Color::RGB(nextIndex, 64, 255 - nextIndex));
    canvas.fill_rect(Rect::new(100, 100, SPRITE_SIZE, SPRITE_SIZE));
    nextIndex
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("snake", SPRITE_SIZE * SCREEN_SIZE, SPRITE_SIZE * SCREEN_SIZE)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut index = 0;
        
    'running: loop {
        if handle_input(&mut event_pump) {
            break 'running;
        }
        // The rest of the game loop goes here...
        update();
        index = draw(index, &mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMES_PER_SECOND));
    }
}