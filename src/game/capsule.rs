use super::game_entity::{GameEntity, GameEntityMoving};
use super::line::Line;
use crate::graphics::colors::{LIGHT_GREEN, RED};
use crate::graphics::draw::{Draw, DrawCircle};
use crate::physics::matrix::RotationMatrix;
use crate::physics::vector2d::{ExtendedVectorOperations, Vector2D};
use sdl2::pixels::Color;
use sdl2::rect::Point;

struct CalculatedPosition {
    point1: Point,
    point2: Point,
    line: Line,
    lines1: [Point; 2],
    lines2: [Point; 2],
}

impl CalculatedPosition {
    fn new(center: Vector2D, width: f32, radius: f32, angle: f32) -> Self {
        let ref_pos1 = center.subtract(Vector2D::new(width, 0.0));
        let ref_pos2 = center.add(Vector2D::new(width, 0.0));

        let ref_unit = ref_pos1.subtract(ref_pos2);

        let rotation_matrix = RotationMatrix::from_angle(angle);
        let new_direction = rotation_matrix.multiply_vector(ref_unit).unit();
        let pos1 = center.add(new_direction.multiply(-width / 2.0));
        let pos2 = center.add(new_direction.multiply(width / 2.0));

        let center_line = Line::new(pos1, pos2);

        let dir = center_line.start.subtract(center_line.end).normal_unit();

        let lines_1 = [
            center_line.start.add(dir.multiply(radius)).into_point(),
            center_line.end.add(dir.multiply(radius)).into_point(),
        ];

        let lines_2 = [
            center_line
                .start
                .subtract(dir.multiply(radius))
                .into_point(),
            center_line.end.subtract(dir.multiply(radius)).into_point(),
        ];

        Self {
            point1: center_line.start.into_point(),
            point2: center_line.end.into_point(),
            line: center_line,
            lines1: lines_1,
            lines2: lines_2,
        }
    }
}

pub struct Capsule {
    center: Vector2D,
    width: f32,
    pub radius: f32,
    rotation: f32,
    color: Color,
    positions: CalculatedPosition,
    changed: bool,
    pub mass: f32,
    pub inverse_mass: f32,
    pub elasticity: f32,
    velocity: Vector2D,
    acceleration: f32,
    friction: f32,
    direction: Vector2D,
}

impl Capsule {
    pub fn new(center: Vector2D, width: f32, radius: f32, rotation: f32, mass: f32) -> Self {
        let calculated_position = CalculatedPosition::new(center, width, radius, rotation);
        let m;
        if mass == 0.0 {
            m = 0.0;
        } else {
            m = 1.0 / mass;
        }

        Self {
            center,
            width,
            radius,
            color: RED,
            rotation: rotation,
            positions: calculated_position,
            changed: false,
            mass,
            inverse_mass: m,
            elasticity: 1.0,
            velocity: Vector2D::zero(),
            acceleration: 0.1,
            friction: 0.0,
            direction: Vector2D::new(0.0, 0.0),
        }
    }

    pub fn get_line(&self) -> Line {
        self.positions.line
    }

    pub fn get_position(&self) -> Vector2D {
        self.center
    }

    pub fn set_position(&mut self, value: Vector2D) {
        self.center = value;

        self.changed = true;
    }

    fn set_direction(&mut self, value: Vector2D) {
        self.direction = value;
    }

    pub fn rotate(&mut self, value: f32) {
        self.rotation += value;

        self.changed = true;
    }
}

impl GameEntity for Capsule {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        _ = canvas.draw_circle_with_color(self.positions.point1, self.radius, self.color);
        _ = canvas.draw_circle_with_color(self.positions.point2, self.radius, self.color);

        _ = canvas.draw_lines_with_color(&self.positions.line.into_points()[..], LIGHT_GREEN);

        _ = canvas.draw_lines_with_color(&self.positions.lines1[..], RED);
        _ = canvas.draw_lines_with_color(&self.positions.lines2[..], RED);

        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        self.velocity = self
            .velocity
            .add(self.direction.unit().multiply(self.acceleration))
            .multiply(1.0 - self.friction);
        self.center = self.center.add(self.velocity.clone());

        //if self.changed {
        self.positions =
            CalculatedPosition::new(self.center, self.width, self.radius, self.rotation);
        //}

        Ok(())
    }
}

impl GameEntityMoving for Capsule {
    fn set_velocity(&mut self, value: Vector2D) {
        self.velocity = value;
    }

    fn set_direction(&mut self, value: Vector2D) {
        self.direction = value;
    }

    fn get_velocity(&self) -> Vector2D {
        self.velocity.clone()
    }
}