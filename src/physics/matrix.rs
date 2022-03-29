use super::vector2d::Vector2D;

pub struct RotationMatrix {
    data: [[f32; 2]; 2]
}

impl RotationMatrix {
    pub fn new() -> Self {
        let data: [[f32; 2]; 2]= [[0.0, 0.0], [0.0, 0.0]];

        Self { 
            data 
        }
    }

    pub fn from_angle(angle: f32) -> Self {
        let data: [[f32; 2]; 2]= [[angle.cos(), -angle.sin()], [angle.sin(), angle.cos()]];

        Self { 
            data 
        }
    }

    pub fn multiply_vector(&self, vector: Vector2D) -> Vector2D {
        let result = Vector2D::new(
            self.data[0][0] * vector.x + self.data[0][1] * vector.y,
            self.data[1][0] * vector.x + self.data[1][1] * vector.y
        );
        
        result
    }
}