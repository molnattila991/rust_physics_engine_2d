use crate::{physics::vector2d::{Vector2D, ExtendedVectorOperations}, graphics::{draw::Draw, colors::WHITE}};

use super::game_entity::GameEntity;

pub struct Wall {
    pub start: Vector2D,
    pub end: Vector2D,
}

impl Wall {
    pub fn new(start: Vector2D, end: Vector2D) -> Self {
        Self {
            start: start,
            end: end
        }
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
        Ok(())
    }
}