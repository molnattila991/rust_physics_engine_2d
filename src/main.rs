mod graphics;

extern  crate sdl2;

use std::time::Duration;

use physics_engine_2d::graphics::colors::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use physics_engine_2d::graphics::draw::{Draw, DrawCircle};

pub fn main () {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust Physics engine 2D", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(LIGHT_BLUE);
    canvas.clear();
    canvas.present();

    
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(LIGHT_BLUE);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        // canvas.set_draw_color(LIGHT_BLUE);
        // canvas.clear();
        let result = canvas.draw_circle_with_color(Point::new(100,100), 510.0, WHITE);
        let result = canvas.draw_circle_with_color_thick(Point::new(11,11), 10.0, 3, RED);
        let result = canvas.draw_circle_with_color_filled(Point::new(44,44), 20.0, LIGHT_GREEN);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}