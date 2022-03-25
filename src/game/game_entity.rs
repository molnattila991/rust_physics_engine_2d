use sdl2::{render::Canvas, video::Window};

use crate::physics::vector2d::Vector2D;

pub trait GameEntity {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn update(&mut self) -> Result<(), String>;
}

pub trait GameEntityMoving {
    fn set_velocity(&mut self, value: Vector2D);
    fn set_direction(&mut self, value: Vector2D);
    fn get_velocity(&self) -> Vector2D;
}