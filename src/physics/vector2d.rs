use sdl2::rect::Point;

#[derive(Clone)]
pub struct Vector2D {
    x: f32,
    y: f32
}

impl Vector2D {
    pub fn new(x: f32, y:f32) -> Self {
        Self {x, y}
    }

    pub fn into_point(self) -> Point {
        Point::new(self.x.round() as i32, self.y.round() as i32)
    }

    pub fn add(&self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }

    pub fn multiply(&self, value: f32) -> Vector2D {
        Vector2D::new(self.x * value, self.y * value)
    }

    pub fn subtract(&self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}