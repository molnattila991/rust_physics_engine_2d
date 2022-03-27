use sdl2::rect::Point;
use sdl2::{render::Canvas, video::Window, pixels::Color};

use crate::graphics::colors::{WHITE, RED};
use crate::graphics::draw::Draw;
use crate::physics::vector2d::{ExtendedVectorOperations, Vector2D};

use super::super::graphics::draw::DrawCircle;
use super::game_entity::{GameEntity, GameEntityMoving};

pub struct Ball {
    pub radius: f32,
    pub is_player: bool,
    position: Vector2D,         //For calculation purposes
    pub mass: f32,
    pub inverse_mass: f32,
    pub elasticity: f32,
    position_point: Point,          //For drawing
    velocity: Vector2D, 
    color: Color,

    direction: Vector2D,
    acceleration: f32,
    friction: f32

}

impl Ball {
    pub fn new(position: Vector2D, velocity: Vector2D, radius: f32, color: Color, mass: f32) -> Self {
        let m;
        if mass == 0.0 { 
            m = 0.0; 
        }else { 
            m = 1.0/mass; 
        }
        Self {
            position: position.clone(),
            velocity,
            position_point: position.into_point(),
            radius,
            color,
            mass,
            inverse_mass: m,
            direction: Vector2D::new(0.0,0.0),
            acceleration: 0.1,
            friction: 0.01,
            is_player: false,
            elasticity: 1.0
        }
    }

    pub fn get_position(&self) -> Vector2D { 
        self.position.clone()
    }

    pub fn set_position(&mut self, position: Vector2D) {
        self.position = position;
        self.position_point = self.position.clone().into_point();
    }
}


impl GameEntity for Ball {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        
        let result = canvas.draw_circle_with_color(self.position_point, self.radius, self.color);

        // let radius = self.radius.clone() as i32;
        // let boundary : [Point; 5] = [
        //     Point::new(self.position_point.x + radius, self.position_point.y + radius),
        //     Point::new(self.position_point.x - radius, self.position_point.y + radius),
        //     Point::new(self.position_point.x - radius, self.position_point.y - radius),
        //     Point::new(self.position_point.x + radius, self.position_point.y - radius),
        //     Point::new(self.position_point.x + radius, self.position_point.y + radius),
        // ];
        // let result = canvas.draw_lines_with_color(&boundary[..], WHITE);


        if self.is_player {
            let points: [Point; 2] = [self.position_point, self.position.add(self.velocity.multiply(10.0)).into_point()];
            let _ = canvas.draw_lines_with_color(&points[..], WHITE).unwrap();
            let points: [Point; 2] = [self.position_point, self.position.add(self.velocity.normal_unit().multiply(100.0)).into_point()];
            let _ = canvas.draw_lines_with_color(&points[..], WHITE).unwrap();
            let points: [Point; 2] = [self.position_point, self.position.add(self.direction.multiply(100.0)).into_point()];
            let _ = canvas.draw_lines_with_color(&points[..], RED).unwrap();
            let points: [Point; 2] = [self.position_point, self.position.add(self.direction.normal_unit().multiply(100.0)).into_point()];
            let _ = canvas.draw_lines_with_color(&points[..], RED).unwrap();
        }
        
        result
    }

    fn update(&mut self) -> Result<(), String> {

        self.direction = self.direction.unit().multiply(self.acceleration);
        self.velocity = self.velocity.add(self.direction.clone()).multiply(1.0 - self.friction);
        self.position = self.position.add(self.velocity.clone());
        
        //self.velocity = self.velocity.add(self.direction.multiply(self.acceleration)).multiply(1.0 - self.friction);
        //self.position = self.position.add(self.velocity.clone());
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

    fn get_velocity(&self) -> Vector2D {
        self.velocity.clone()
    }
}