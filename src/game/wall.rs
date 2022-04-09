use crate::{physics::{vector2d::{Vector2D, ExtendedVectorOperations}, matrix::RotationMatrix}, graphics::{draw::Draw, colors::WHITE}};

use super::{game_entity::GameEntity, line::Line};

pub struct Wall {
    pub line: Line,

    center: Vector2D,
    ref_unit: Vector2D,
    angle: f32,
}

impl Wall {
    pub fn new(start: Vector2D, end: Vector2D) -> Self {
        let l = Line::new(start, end);
        Self {
            line: l,
            center: l.start.add(l.end).multiply(0.5),
            ref_unit: l.end.subtract(l.start).unit(),
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
        self.line.end.subtract(self.line.start.clone()).unit()
    }
}

impl GameEntity for Wall {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let lines = [self.line.start.into_point(), self.line.end.into_point()];
        let result = canvas.draw_lines_with_color(&lines[..], WHITE);

        result
    }

    fn update(&mut self) -> Result<(), String> {
        self.line.rotate(self.angle);

        // let rotation_matrix = RotationMatrix::from_angle(self.angle);
        // let new_direction = rotation_matrix.multiply_vector(self.ref_unit);
        // self.line.start = self.center.add(new_direction.multiply(-self.length/2.0));
        // self.line.end = self.center.add(new_direction.multiply(self.length/2.0));

        Ok(())
    }
}