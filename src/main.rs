mod graphics;

extern crate sdl2;
use graphics::draw::DrawCircle;
use rand::Rng;
use std::time::Duration;

use physics_engine_2d::{
    game::{
        ball::Ball,
        capsule::{self, Capsule},
        game_entity::{GameEntity, GameEntityMoving},
        line::Line,
        wall::{self, Wall},
    },
    graphics::{colors::*, draw::Draw},
    physics::vector2d::{ExtendedVectorOperations, Vector2D},
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rust Physics engine 2D", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(LIGHT_BLUE);
    canvas.clear();
    canvas.present();

    let mut direction = Point::new(0, 0);
    let mut ball_vector = Vec::new();
    let mut wall_vector: Vec<Wall> = Vec::new();
    let mut capsule_vector: Vec<Capsule> = Vec::new();
    let mut rng = rand::thread_rng();

    capsule_vector.push(Capsule::new(
        Vector2D::new(400.0, 100.0),
        100.0,
        30.0,
        30.0,
        10.0,
    ));
    capsule_vector.push(Capsule::new(
        Vector2D::new(500.0, 300.0),
        70.0,
        10.0,
        10.0,
        10.0,
    ));
    capsule_vector.push(Capsule::new(
        Vector2D::new(500.0, 350.0),
        70.0,
        10.0,
        330.0,
        10.0,
    ));

    for _ in 0..55 {
        ball_vector.push(Ball::new(
            Vector2D::new(rng.gen_range(0..400) as f32, rng.gen_range(0..400) as f32),
            Vector2D::new(0.0, 0.0),
            10.0,
            WHITE,
            rng.gen_range(0..400) as f32,
        ));
    }

    wall_vector.push(Wall::new(
        Vector2D::new(110.0, 110.0),
        Vector2D::new(310.0, 310.0),
    ));

    wall_vector.push(Wall::new(
        Vector2D::new(0.0, 300.0),
        Vector2D::new(700.0, 300.0),
    ));

    wall_vector.push(Wall::new(
        Vector2D::new(10.0, 10.0),
        Vector2D::new(700.0, 10.0),
    ));
    wall_vector.push(Wall::new(
        Vector2D::new(700.0, 10.0),
        Vector2D::new(500.0, 700.0),
    ));
    wall_vector.push(Wall::new(
        Vector2D::new(500.0, 700.0),
        Vector2D::new(10.0, 700.0),
    ));
    wall_vector.push(Wall::new(
        Vector2D::new(10.0, 700.0),
        Vector2D::new(10.0, 10.0),
    ));

    ball_vector[0].is_player = true;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(LIGHT_BLUE);
        canvas.clear();

        let (mut UP, mut DOWN, mut LEFT, mut RIGHT) = (false, false, false, false);
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    DOWN = true;
                }
                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    UP = true;
                }
                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    LEFT = true;
                }
                Event::KeyDown {
                    repeat: false,
                    keycode: Some(Keycode::Right),
                    ..
                } => {
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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        //Update

        // ball_vector[0].update().unwrap();
        // ball_vector[0].set_direction(Vector2D::new(direction.x as f32, direction.y as f32));

        for index1 in 0..ball_vector.len() {
            if ball_vector[index1].is_player == true {
                ball_vector[index1]
                    .set_direction(Vector2D::new(direction.x as f32, direction.y as f32));
            }
            ball_vector[index1].update().unwrap();

            for index_wall in 0..wall_vector.len() {
                if index_wall == 0 {
                    wall_vector[index_wall].rotate(0.0002);
                }
                wall_vector[index_wall].update().unwrap();

                if collision_detection_ball_wall(&ball_vector[index1], &wall_vector[index_wall]) {
                    //println!("Collide {}", rng.gen_range(0, 400));
                    let new_position = penetration_resolution_ball_wall(
                        &ball_vector[index1],
                        &wall_vector[index_wall],
                    );
                    ball_vector[index1].set_position(new_position);
                    let v = collision_resolution_ball_wall(
                        &ball_vector[index1],
                        &wall_vector[index_wall],
                    );
                    ball_vector[index1].set_velocity(v);
                }
            }

            for index2 in index1 + 1..ball_vector.len() {
                if index1 != index2 {
                    let is_collide =
                        collision_ball_ball(&ball_vector[index1], &ball_vector[index2]).unwrap();
                    if is_collide {
                        let res = penetration_resolution_ball_ball(
                            &ball_vector[index1],
                            &ball_vector[index2],
                        );
                        ball_vector[index1].set_position(res.0);
                        ball_vector[index2].set_position(res.1);

                        let res = collision_resolution_ball_ball(
                            &ball_vector[index1],
                            &ball_vector[index2],
                        );

                        let new_vel1 = ball_vector[index1].get_velocity().add(res.0);
                        let new_vel2 = ball_vector[index2].get_velocity().add(res.1);

                        ball_vector[index1].set_velocity(new_vel1);
                        ball_vector[index2].set_velocity(new_vel2);
                    }
                }
            }
        }

        for index_1 in 0..capsule_vector.len() {
            for index_2 in index_1 + 1..capsule_vector.len() {
                let res = closest_point_between_line_segments(
                    &capsule_vector[index_1].get_line(),
                    &capsule_vector[index_2].get_line(),
                );
                let line = [res[0].into_point(), res[1].into_point()];
                _ = canvas.draw_lines_with_color(&line[..], PINK);
                _ = canvas.draw_circle_with_color(
                    res[0].into_point(),
                    capsule_vector[index_1].radius,
                    PINK,
                );
                _ = canvas.draw_circle_with_color(
                    res[1].into_point(),
                    capsule_vector[index_2].radius,
                    PINK,
                );

                if collision_detection_capsule_capsule(
                    &capsule_vector[index_1],
                    &capsule_vector[index_2],
                ) {
                    let res = penetration_resolution_capsule_capsule(
                        &capsule_vector[index_1],
                        &capsule_vector[index_2],
                    );

                    capsule_vector[index_1].set_position(res.0);
                    capsule_vector[index_2].set_position(res.1);

                    let res = collision_resolution_capsule_capsule(
                        &capsule_vector[index_1],
                        &capsule_vector[index_2],
                    );

                    let new_vel1 = ball_vector[index_1].get_velocity().add(res.0);
                    let new_vel2 = ball_vector[index_2].get_velocity().add(res.1);

                    ball_vector[index_1].set_velocity(new_vel1);
                    ball_vector[index_2].set_velocity(new_vel2);
                }
            }

            // let r = capsule_vector[index_1].get_position().add_number(1.0, 0.0);
            // capsule_vector[index_1].set_position(r);

            capsule_vector[index_1].rotate(0.01);
            _ = capsule_vector[index_1].update();
        }

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

            let point = closest_point_on_line_segment(
                ball_vector[0].get_position(),
                &wall_vector[index_wall].line,
            );
            let points = [
                ball_vector[0].get_position().into_point(),
                point.into_point(),
            ];
            let _ = canvas.draw_lines_with_color(&points[..], RED);
        }

        for index1 in 0..ball_vector.len() {
            ball_vector[index1].draw(&mut canvas).unwrap();
        }

        for index_capsule in 0..capsule_vector.len() {
            _ = capsule_vector[index_capsule].draw(&mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}

fn collision_ball_ball(b1: &Ball, b2: &Ball) -> Result<bool, String> {
    if b1.radius + b2.radius
        >= b1
            .get_position()
            .subtract(b2.get_position().clone())
            .magnitude()
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn penetration_resolution_ball_ball(b1: &Ball, b2: &Ball) -> (Vector2D, Vector2D) {
    let distance = b1.get_position().subtract(b2.get_position().clone());
    let pen_depth = b1.radius + b2.radius - distance.magnitude();
    let pen_res = distance
        .unit()
        .multiply(pen_depth / (b1.inverse_mass + b2.inverse_mass));

    (
        b1.get_position()
            .add(pen_res.multiply(b1.inverse_mass).clone()),
        b2.get_position().add(pen_res.multiply(-b2.inverse_mass)),
    )
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

    (
        impulse_vector.multiply(b1.inverse_mass).clone(),
        impulse_vector.multiply(-b2.inverse_mass),
    )
}

fn closest_point_on_line_segment(point: Vector2D, line: &Line) -> Vector2D {
    let point_to_line_start = line.start.subtract(point);
    if Vector2D::dot_product(line.direction, point_to_line_start.clone()) > 0.0 {
        return line.start.clone();
    }

    let line_end_to_point = point.subtract(line.end.clone());
    if Vector2D::dot_product(line.direction, line_end_to_point) > 0.0 {
        return line.end.clone();
    }

    let closest_dist = Vector2D::dot_product(line.direction, point_to_line_start);
    let closest_vect = line.direction.multiply(closest_dist);
    line.start.subtract(closest_vect)
}

fn closest_point_wall_ball(ball: &Ball, wall: &Wall) -> Vector2D {
    let ball_to_wall_start = wall.line.start.subtract(ball.get_position());
    if Vector2D::dot_product(wall.wall_unit(), ball_to_wall_start.clone()) > 0.0 {
        return wall.line.start.clone();
    }

    let wall_end_to_ball = ball.get_position().subtract(wall.line.end.clone());
    if Vector2D::dot_product(wall.wall_unit(), wall_end_to_ball) > 0.0 {
        return wall.line.end.clone();
    }

    let closest_dist = Vector2D::dot_product(wall.wall_unit(), ball_to_wall_start);
    let closest_vect = wall.wall_unit().multiply(closest_dist);
    wall.line.start.subtract(closest_vect)
}

fn collision_detection_ball_wall(ball: &Ball, wall: &Wall) -> bool {
    let ball_to_closest = closest_point_on_line_segment(ball.get_position(), &wall.line)
        .subtract(ball.get_position());
    if ball_to_closest.magnitude() <= ball.radius {
        return true;
    }

    false
}

fn penetration_resolution_ball_wall(ball: &Ball, wall: &Wall) -> Vector2D {
    let penetration_vector = ball.get_position().subtract(closest_point_on_line_segment(
        ball.get_position(),
        &wall.line,
    ));
    ball.get_position().add(
        penetration_vector
            .unit()
            .multiply(ball.radius - penetration_vector.magnitude()),
    )
}

fn collision_resolution_ball_wall(ball: &Ball, wall: &Wall) -> Vector2D {
    let normal = ball
        .get_position()
        .subtract(closest_point_on_line_segment(
            ball.get_position(),
            &wall.line,
        ))
        .unit();
    let sep_velocity = Vector2D::dot_product(ball.get_velocity(), normal);
    let new_sep_velocity = -sep_velocity * 0.0; // ball.elasticity;
    let vsep_diff = sep_velocity - new_sep_velocity;
    ball.get_velocity().add(normal.multiply(-vsep_diff))
}

fn closest_point_between_line_segments(l1: &Line, l2: &Line) -> [Vector2D; 2] {
    let temp = closest_point_on_line_segment(l1.start, l2);

    let mut shortest_distance = temp.subtract(l1.start).magnitude();
    let mut closest_points = [l1.start, temp];

    let temp = closest_point_on_line_segment(l1.end, l2);
    let temp_mag = temp.subtract(l1.end).magnitude();

    if temp_mag < shortest_distance {
        shortest_distance = temp_mag;
        closest_points = [l1.end, temp];
    }

    let temp = closest_point_on_line_segment(l2.start, l1);
    let temp_mag = temp.subtract(l2.start).magnitude();

    if temp_mag < shortest_distance {
        shortest_distance = temp_mag;
        closest_points = [temp, l2.start];
    }

    let temp = closest_point_on_line_segment(l2.end, l1);
    let temp_mag = temp.subtract(l2.end).magnitude();

    if temp_mag < shortest_distance {
        shortest_distance = temp_mag;
        closest_points = [temp, l2.end];
    }

    closest_points
}

fn collision_detection_capsule_capsule(c1: &Capsule, c2: &Capsule) -> bool {
    let points = closest_point_between_line_segments(&c1.get_line(), &c2.get_line());

    if c1.radius + c2.radius >= points[0].subtract(points[1]).magnitude() {
        return true;
    }

    false
}

fn penetration_resolution_capsule_capsule(c1: &Capsule, c2: &Capsule) -> (Vector2D, Vector2D) {
    let points = closest_point_between_line_segments(&c1.get_line(), &c2.get_line());

    let distance = points[0].subtract(points[1]);
    let penetration_depth = c1.radius + c2.radius - distance.magnitude();
    let penetration_resolution = distance
        .unit()
        .multiply(penetration_depth / (c1.inverse_mass + c2.inverse_mass));

    (
        c1.get_position()
            .add(penetration_resolution.multiply(c1.inverse_mass)),
        c2.get_position()
            .add(penetration_resolution.multiply(-c2.inverse_mass)),
    )
}

fn collision_resolution_capsule_capsule(c1: &Capsule, c2: &Capsule) -> (Vector2D, Vector2D) {
    let points = closest_point_between_line_segments(&c1.get_line(), &c2.get_line());

    let normal = points[0].subtract(points[1]).unit();
    let relative_velocity = c1.get_velocity().subtract(c2.get_velocity());
    let sep_velocity = Vector2D::dot_product(relative_velocity, normal.clone());
    let new_sep_velocity = -sep_velocity * c1.elasticity.max(c2.elasticity);
    //let sep_velocity_vector = normal.multiply(new_sep_velocity);

    let vsep_diff = new_sep_velocity - sep_velocity;
    let impulse = vsep_diff / (c1.inverse_mass + c2.inverse_mass);
    let impulse_vector = normal.multiply(impulse);

    (
        impulse_vector.multiply(c1.inverse_mass).clone(),
        impulse_vector.multiply(-c2.inverse_mass),
    )
}
