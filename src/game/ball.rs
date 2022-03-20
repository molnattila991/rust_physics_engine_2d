use sdl2::rect::Point;
use sdl2::{render::Canvas, video::Window, pixels::Color};

use super::super::physics::vector2d::Vector2D;
use super::super::graphics::draw::DrawCircle;
use super::game_entity::{GameEntity, GameEntityMoving};

pub struct Ball {
    center: Vector2D,       //For calculation purposes
    velocity: Vector2D, 
    center_point: Point,    //For drawing
    radius: f32,
    color: Color
}

impl Ball {
    pub fn new(center: Vector2D, velocity: Vector2D, radius: f32, color: Color) -> Self {
        Self {
            center: center.clone(),
            velocity,
            center_point: center.into_point(),
            radius,
            color
        }
    }
}


impl GameEntity for Ball {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.draw_circle_with_color_filled(self.center_point, self.radius, self.color)
    }

    fn update(&mut self) -> Result<(), String> {
        self.center = self.center.add(self.velocity.clone());
        self.center_point = self.center.clone().into_point();

        Ok(())
    }
}

impl GameEntityMoving for Ball {
    fn set_velocity(&mut self, value: Vector2D) {
        self.velocity = value;
    }
}