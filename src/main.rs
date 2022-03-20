mod graphics;

extern  crate sdl2;

use std::time::Duration;

use physics_engine_2d::{graphics::colors::*, game::{ball::Ball, game_entity::{GameEntity, GameEntityMoving}}, physics::vector2d::Vector2D};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

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

    let mut velocity = Point::new(0,0);

    let mut ball = Ball::new(
        Vector2D::new(100.0, 100.0), 
        Vector2D::new(0.0,0.0),
        10.0, 
        LIGHT_GREEN
    );

    let ball2 = Ball::new(
        Vector2D::new(150.0, 100.0), 
        Vector2D::new(0.0,0.0),
        10.0, 
        RED
    );


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(LIGHT_BLUE);
        canvas.clear();

        for event in event_pump.poll_iter() {
            set_velocity(&event, &mut velocity);

            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        //Update

        ball.set_velocity(Vector2D::new(velocity.x as f32, velocity.y as f32));
        ball.update().unwrap();

        //Draw

        ball.draw(&mut canvas).unwrap();    
        ball2.draw(&mut canvas).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}

fn set_velocity(event: &Event, velocity: &mut Point) {
    match event {
        Event::KeyDown {  repeat: false, keycode: Some(Keycode::Down), .. } => {
            velocity.y = velocity.y + 1;
        },
        Event::KeyDown { repeat: false, keycode: Some(Keycode::Up), .. } => {
            velocity.y = velocity.y - 1;
        },
        Event::KeyDown { repeat: false, keycode: Some(Keycode::Left), .. } => {
            velocity.x = velocity.x - 1;
        },
        Event::KeyDown { repeat: false, keycode: Some(Keycode::Right), .. } => {
            velocity.x = velocity.x + 1;
        }
        _ => {}
    }
}