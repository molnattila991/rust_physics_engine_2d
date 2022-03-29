use sdl2::rect::Point;

#[derive(Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0}
    }
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
        Vector2D::new(
            self.x - other.x,
            self.y - other.y
        )
    }
}

impl ExtendedVectorOperations for Vector2D {
    fn magnitude(&self) -> f32 {
        if self.x == 0.0 && self.y == 0.0 {
            0.0
        } else {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    fn unit (&self) -> Vector2D {
        if self.x == 0.0 && self.y == 0.0 {
            Vector2D::new(0.0, 0.0)
        } else {
            let mag = self.magnitude();
            Vector2D::new(self.x / mag, self.y / mag)
        }
    }

    fn normal (&self) -> Vector2D {
        Vector2D::new(-self.y, self.x)
    }

    fn normal_unit(&self) -> Vector2D {
        if self.x == 0.0 && self.y == 0.0 {
            Vector2D::new(0.0, 0.0)
        } else {
            let mag = self.magnitude();
            Vector2D::new(-self.y / mag, self.x / mag)
        }
    }
}

pub trait ExtendedVectorOperations {
    fn magnitude(&self) -> f32;
    fn unit (&self) -> Vector2D;
    fn normal (&self) -> Vector2D;
    fn normal_unit(&self) -> Vector2D;

    fn dot_product(v1: Vector2D, v2: Vector2D) -> f32{
        v1.x * v2.x + v1.y * v2.y
    }
}