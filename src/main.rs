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

    let mut direction = Point::new(0,0);

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
        
        let (mut UP,mut DOWN,mut LEFT,mut RIGHT) = (false, false, false, false);
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {  repeat: false, keycode: Some(Keycode::Down), .. } => {
                    DOWN = true;
                },
                Event::KeyDown { repeat: false, keycode: Some(Keycode::Up), .. } => {
                    UP = true; 
                },
                Event::KeyDown { repeat: false, keycode: Some(Keycode::Left), .. } => {
                    LEFT = true;
                },
                Event::KeyDown { repeat: false, keycode: Some(Keycode::Right), .. } => {
                    RIGHT = true;
                }
                _ => {}
            }

            if UP {
                direction.y = -1;
            }
            if DOWN {
                direction.y = 1;
            }
            if LEFT {
                direction.x = -1;
            }
            if RIGHT {
                direction.x = 1;
            }
            if !UP && !DOWN {
                direction.y = 0;
            }
            if !LEFT && !RIGHT {
                direction.x = 0;
            }

            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        //Update

        ball.set_direction(Vector2D::new(direction.x as f32, direction.y as f32));
        ball.update().unwrap();

        //Draw

        ball.draw(&mut canvas).unwrap();    
        ball2.draw(&mut canvas).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}