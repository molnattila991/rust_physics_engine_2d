use super::game_entity::GameEntity;
use crate::graphics::colors::{RED, WHITE};
use crate::graphics::draw::{Draw, DrawCircle};
use crate::physics::matrix::RotationMatrix;
use crate::physics::vector2d::{ExtendedVectorOperations, Vector2D};
use sdl2::rect::Point;
use sdl2::{pixels::Color, render::Canvas, video::Window};

struct CalculatedPosition {
    pos1: Vector2D,
    pos2: Vector2D,
    point1: Point,
    point2: Point,
    lines1: [Point; 2],
    lines2: [Point; 2]
}

impl CalculatedPosition {
    fn new(center: Vector2D, width: f32, radius: f32, angle: f32) -> Self {
        let ref_pos1 = center.subtract(Vector2D::new(width, 0.0));
        let ref_pos2 = center.add(Vector2D::new(width, 0.0));
        let ref_unit = ref_pos1.subtract(ref_pos2);

        let rotation_matrix = RotationMatrix::from_angle(angle);
        let new_direction = rotation_matrix.multiply_vector(ref_unit).unit();
        let pos1 = center.add(new_direction.multiply(-width/2.0));
        let pos2 = center.add(new_direction.multiply(width/2.0));

        let dir = pos1
            .subtract(pos2)
            .normal_unit();

        let lines_1 = [
            pos1
                .add(dir.multiply(radius))
                .into_point(),
                pos2
                .add(dir.multiply(radius))
                .into_point(),
        ];

        let lines_2 = [
            pos1
                .subtract(dir.multiply(radius))
                .into_point(),
                pos2
                .subtract(dir.multiply(radius))
                .into_point(),
        ];
        
        Self {
            pos1: pos1,
            pos2: pos2,
            point1: pos1.into_point(),
            point2: pos2.into_point(),
            lines1: lines_1,
            lines2: lines_2
        }
    }
}

pub struct Capsule {
    center: Vector2D,
    width: f32,
    radius: f32,
    rotation: f32,
    color: Color,
    ref_unit: Vector2D,
    positions: CalculatedPosition,
}

impl Capsule {
    pub fn new(center: Vector2D, width: f32, radius: f32, rotation: f32) -> Self {
        let calculated_position = CalculatedPosition::new(center, width, radius, rotation);
        let ref_unit = calculated_position.pos1.subtract(calculated_position.pos2);
        Self {
            center,
            width,
            radius,
            color: RED,
            rotation: rotation,
            ref_unit: ref_unit,
            positions: calculated_position,
        }
    }
}

impl GameEntity for Capsule {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        _ = canvas.draw_circle_with_color(
            self.positions.point1,
            self.radius,
            self.color,
        );
        _ = canvas.draw_circle_with_color(
            self.positions.point2,
            self.radius,
            self.color,
        );
        _ = canvas.draw_lines_with_color(&self.positions.lines1[..], RED);
        _ = canvas.draw_lines_with_color(&self.positions.lines2[..], RED);

        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        self.rotation += 0.01;

        self.positions = CalculatedPosition::new(self.center, self.width, self.radius, self.rotation);

        Ok(())
    }
}
