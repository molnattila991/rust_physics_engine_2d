use std::ops::Add;

use sdl2::rect::Point;
use sdl2::{render::Canvas, video::Window, pixels::Color};

use crate::graphics::colors::WHITE;
use crate::graphics::draw::Draw;

use super::super::physics::vector2d::Vector2D;
use super::super::graphics::draw::DrawCircle;
use super::game_entity::{GameEntity, GameEntityMoving};

pub struct Ball {
    position: Vector2D,       //For calculation purposes
    position_point: Point,    //For drawing
    velocity: Vector2D, 
    radius: f32,
    color: Color,

    direction: Vector2D,
    acceleration: f32,
    friction: f32
}

impl Ball {
    pub fn new(position: Vector2D, velocity: Vector2D, radius: f32, color: Color) -> Self {
        Self {
            position: position.clone(),
            velocity,
            position_point: position.into_point(),
            radius,
            color,
            direction: Vector2D::new(0.0,0.0),
            acceleration: 0.1,
            friction: 0.01,
        }
    }
}


impl GameEntity for Ball {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let points: [Point; 2] = [self.position_point, self.position.add(self.velocity.multiply(10.0)).into_point()];

        let result = canvas.draw_circle_with_color_filled(self.position_point, self.radius, self.color);
        
        canvas.draw_lines_with_color(&points[..], WHITE).unwrap();

        result
    }

    fn update(&mut self) -> Result<(), String> {
        self.velocity = self.velocity.add(self.direction.multiply(self.acceleration)).multiply(1.0 - self.friction);
        

        self.position = self.position.add(self.velocity.clone());
        self.position_point = self.position.clone().into_point();

        Ok(())
    }
}

impl GameEntityMoving for Ball {
    fn set_velocity(&mut self, value: Vector2D) {
        self.velocity = value;
    }

    fn set_direction(&mut self, value: Vector2D) {
        self.direction = value;
    }
}