mod graphics;

extern  crate sdl2;
use rand::Rng;
use std::{time::Duration};

use physics_engine_2d::{graphics::{colors::*, draw::Draw}, game::{ball::Ball, game_entity::{GameEntity, GameEntityMoving}, wall::{Wall, self}, capsule::Capsule}, physics::vector2d::{Vector2D, ExtendedVectorOperations}};
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
    let mut ball_vector = Vec::new();
    let mut wall_vector: Vec<Wall> = Vec::new();
    let mut rng = rand::thread_rng();

    let mut capsule = Capsule::new(Vector2D::new(400.0, 100.0), 100.0, 30.0, 30.0);

    for _ in 0..55 {
        ball_vector.push(Ball::new(
            Vector2D::new(rng.gen_range(0..400) as f32, rng.gen_range(0..400) as f32), 
            Vector2D::new(0.0,0.0),
            10.0, 
            WHITE,
            rng.gen_range(0..400) as f32
        ));
    }
    
    wall_vector.push(Wall::new(Vector2D::new(110.0, 110.0), Vector2D::new(310.0, 310.0)));
    

    wall_vector.push(Wall::new(Vector2D::new(0.0, 300.0), Vector2D::new(700.0, 300.0)));

    wall_vector.push(Wall::new(Vector2D::new(10.0, 10.0), Vector2D::new(700.0, 10.0)));
    wall_vector.push(Wall::new(Vector2D::new(700.0, 10.0), Vector2D::new(500.0, 700.0)));
    wall_vector.push(Wall::new(Vector2D::new(500.0, 700.0), Vector2D::new(10.0, 700.0)));
    wall_vector.push(Wall::new(Vector2D::new(10.0, 700.0), Vector2D::new(10.0, 10.0)));


    ball_vector[0].is_player = true;
    
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

        // ball_vector[0].update().unwrap();
        // ball_vector[0].set_direction(Vector2D::new(direction.x as f32, direction.y as f32));

        for index1 in 0..ball_vector.len()  {
            if ball_vector[index1].is_player == true{
                ball_vector[index1].set_direction(Vector2D::new(direction.x as f32, direction.y as f32));
            }
            ball_vector[index1].update().unwrap();

            for index_wall in 0..wall_vector.len() {
                if index_wall == 0 {
                    wall_vector[index_wall].rotate(0.0002);
                }
                wall_vector[index_wall].update().unwrap();

                if collision_detection_ball_wall(&ball_vector[index1], &wall_vector[index_wall]) {
                    //println!("Collide {}", rng.gen_range(0, 400));
                    let new_position = penetration_resolution_ball_wall(&ball_vector[index1], &wall_vector[index_wall]);
                    ball_vector[index1].set_position(new_position);
                    let v = collision_resolution_ball_wall(&ball_vector[index1], &wall_vector[index_wall]);
                    ball_vector[index1].set_velocity(v);
                }
            }

            for index2 in index1 + 1..ball_vector.len()  {
                if index1 != index2 {
                    let is_collide = collision_ball_ball(&ball_vector[index1], &ball_vector[index2]).unwrap();
                    if is_collide {
                        let res = penetration_resolution_ball_ball(&ball_vector[index1], &ball_vector[index2]);
                        ball_vector[index1].set_position(res.0);
                        ball_vector[index2].set_position(res.1);
                        
                        let res = collision_resolution_ball_ball(&ball_vector[index1], &ball_vector[index2]);
                        
                        let new_vel1 = ball_vector[index1].get_velocity().add(res.0);
                        let new_vel2 = ball_vector[index2].get_velocity().add(res.1);

                        ball_vector[index1].set_velocity(new_vel1);
                        ball_vector[index2].set_velocity(new_vel2);
                    }
                }
            }
        }

        _ = capsule.update();
        
        //Draw
        //ball_vector[0].draw(&mut canvas).unwrap();

        // if collision_detection_ball_wall(&ball_vector[0], &wall_vector[0]) {
        //     println!("Collide {}", rng.gen_range(0, 400));
        //     let new_position = penetration_resolution_ball_wall(&ball_vector[0], &wall_vector[0]);
        //     ball_vector[0].set_position(new_position);
        //     let v = collision_resolution_ball_wall(&ball_vector[0], &wall_vector[0]);
        //     ball_vector[0].set_velocity(v);
        // }

        for index_wall in 0..wall_vector.len() {
            wall_vector[index_wall].draw(&mut canvas).unwrap();

            let point = closest_point_wall_ball(&ball_vector[0], &wall_vector[index_wall]);
            let points = [ball_vector[0].get_position().into_point(), point.into_point()];
            let _ = canvas.draw_lines_with_color(&points[..], RED);
        }

        for index1 in 0..ball_vector.len()  {
            ball_vector[index1].draw(&mut canvas).unwrap();    
        }

        _ = capsule.draw(&mut canvas);
        

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}

fn collision_ball_ball(b1: &Ball, b2: &Ball) -> Result<bool, String> {
    if b1.radius + b2.radius >= b1.get_position().subtract(b2.get_position().clone()).magnitude() {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn penetration_resolution_ball_ball(b1: &Ball, b2: &Ball) -> (Vector2D, Vector2D) {
    let distance = b1.get_position().subtract(b2.get_position().clone());
    let pen_depth = b1.radius + b2.radius - distance.magnitude();
    let pen_res = distance.unit().multiply(pen_depth/(b1.inverse_mass + b2.inverse_mass));

    (b1.get_position().add(pen_res.multiply(b1.inverse_mass).clone()), b2.get_position().add(pen_res.multiply(-b2.inverse_mass)))
}

fn collision_resolution_ball_ball(b1: &Ball, b2: &Ball) -> (Vector2D, Vector2D) {
    let normal = b1.get_position().subtract(b2.get_position()).unit();
    let relative_velocity = b1.get_velocity().subtract(b2.get_velocity());
    let sep_velocity = Vector2D::dot_product(relative_velocity, normal.clone());
    let new_sep_velocity = -sep_velocity * b1.elasticity.max(b2.elasticity);
    //let sep_velocity_vector = normal.multiply(new_sep_velocity);
    
    let vsep_diff = new_sep_velocity - sep_velocity;
    let impulse = vsep_diff / (b1.inverse_mass + b2.inverse_mass);
    let impulse_vector = normal.multiply(impulse);

    (impulse_vector.multiply(b1.inverse_mass).clone(), impulse_vector.multiply(-b2.inverse_mass))
}

fn closest_point_wall_ball(ball: &Ball, wall: &Wall) -> Vector2D {
    let ball_to_wall_start = wall.start.subtract(ball.get_position());
    if Vector2D::dot_product(wall.wall_unit(), ball_to_wall_start.clone()) > 0.0 {
        return wall.start.clone();
    }

    let wall_end_to_ball = ball.get_position().subtract(wall.end.clone());
    if Vector2D::dot_product(wall.wall_unit(), wall_end_to_ball) > 0.0 {
        return wall.end.clone();
    }

    let closest_dist = Vector2D::dot_product(wall.wall_unit(), ball_to_wall_start);
    let closest_vect = wall.wall_unit().multiply(closest_dist);
    wall.start.subtract(closest_vect)
}

fn collision_detection_ball_wall(ball: &Ball, wall: &Wall) -> bool {
    let ball_to_closest = closest_point_wall_ball(ball, wall).subtract(ball.get_position());
    if ball_to_closest.magnitude() <= ball.radius {
        return true;
    }

    false
}

fn penetration_resolution_ball_wall(ball: &Ball, wall: &Wall) -> Vector2D {
    let penetration_vector = ball.get_position().subtract(closest_point_wall_ball(ball, wall));
    ball.get_position().add(penetration_vector.unit().multiply(ball.radius - penetration_vector.magnitude()))
}

fn collision_resolution_ball_wall(ball: &Ball, wall: &Wall) -> Vector2D {
    let normal = ball.get_position().subtract(closest_point_wall_ball(ball, wall)).unit();
    let sep_velocity = Vector2D::dot_product(ball.get_velocity(), normal);
    let new_sep_velocity = -sep_velocity * 0.0;// ball.elasticity;
    let vsep_diff = sep_velocity - new_sep_velocity;
    ball.get_velocity().add(normal.multiply(-vsep_diff))
}