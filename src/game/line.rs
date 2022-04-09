use sdl2::rect::Point;

use crate::physics::{vector2d::{Vector2D, ExtendedVectorOperations}, matrix::RotationMatrix};

#[derive(Clone, Copy)]
pub struct Line {
    pub start: Vector2D,
    pub end: Vector2D,
    pub direction: Vector2D,
    ref_unit: Vector2D,
    center: Vector2D,
    length: f32,
}

impl Line {
    pub fn new(start: Vector2D, end: Vector2D) -> Self {
        let dir = end.subtract(start.clone());
        Self {
            start,
            end,
            direction: dir.unit(),
            ref_unit: dir.unit(),
            center: start.add(end).multiply(0.5),
            length: dir.magnitude()
        }
    }

    pub fn rotate(&mut self, value: f32) {
        let rotation_matrix = RotationMatrix::from_angle(value);
        let new_direction = rotation_matrix.multiply_vector(self.ref_unit);
        self.direction = new_direction.unit();
        self.start = self.center.add(new_direction.multiply(-self.length/2.0));
        self.end = self.center.add(new_direction.multiply(self.length/2.0));
    }

    pub fn into_points(&self) -> [Point; 2] {
        [self.start.into_point(), self.end.into_point()]
    }
}