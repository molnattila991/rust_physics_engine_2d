use sdl2::rect::Point;

pub struct Vector2D {
    x: f32,
    y: f32
}

impl Vector2D {
    pub fn new(x: f32, y:f32) -> Self {
        Self {x, y}
    }

    pub fn into_point(&self) -> Point {
        Point::new(self.x.round() as i32, self.y.round() as i32)
    }

    pub fn add(&self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}