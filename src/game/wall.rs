use crate::{physics::{vector2d::{Vector2D, ExtendedVectorOperations}, matrix::RotationMatrix}, graphics::{draw::Draw, colors::WHITE}};

use super::game_entity::GameEntity;

pub struct Wall {
    pub start: Vector2D,
    pub end: Vector2D,

    center: Vector2D,
    length: f32,
    ref_start: Vector2D,
    ref_end: Vector2D,
    ref_unit: Vector2D,
    angle: f32,
}

impl Wall {
    pub fn new(start: Vector2D, end: Vector2D) -> Self {
        Self {
            start,
            end,
            center: start.add(end).multiply(0.5),
            length: end.subtract(start).magnitude(), 
            ref_start: start,
            ref_end: end,
            ref_unit: end.subtract(start).unit(),
            angle: 0.0
        }
    }

    pub fn set_angle(&mut self, value: f32) {
        self.angle = value;
    }

    pub fn rotate(&mut self, value: f32) {
        self.angle += value;
    }

    pub fn wall_unit(&self) -> Vector2D {
        self.end.subtract(self.start.clone()).unit()
    }
}

impl GameEntity for Wall {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let lines = [self.start.clone().into_point(), self.end.clone().into_point()];
        let result = canvas.draw_lines_with_color(&lines[..], WHITE);

        result
    }

    fn update(&mut self) -> Result<(), String> {
        let rotation_matrix = RotationMatrix::from_angle(self.angle);
        let new_direction = rotation_matrix.multiply_vector(self.ref_unit);
        self.start = self.center.add(new_direction.multiply(-self.length/2.0));
        self.end = self.center.add(new_direction.multiply(self.length/2.0));

        Ok(())
    }
}