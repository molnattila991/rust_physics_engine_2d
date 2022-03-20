
use std::{f32::consts::PI, ops::Add};

use sdl2::{render::Canvas, pixels::Color, rect::Point, video::Window};

pub trait Draw {
    fn draw_lines_with_color<'a, P>(&mut self, points: P, color: Color) -> Result<(), String> where P: Into<&'a [Point]>;
    fn draw_rectangle_with_color(&mut self, point: Point, width: i32, height: i32,  color: Color) -> Result<(), String>;
}

pub trait DrawCircle {
    fn draw_circle_with_color(&mut self, center: Point, radius: f32, color: Color) -> Result<(), String>;
    fn draw_circle_with_color_thick(&mut self, center: Point, radius: f32, thickness: i8, color: Color) -> Result<(), String>;
    fn draw_circle_with_color_filled(&mut self, center: Point, radius: f32, color: Color) -> Result<(), String>;
}

impl Draw for Canvas<Window> {
    fn draw_lines_with_color<'a, P>(&mut self, points: P, color: Color) -> Result<(), String> where P: Into<&'a [Point]> {
        self.set_draw_color(color);

        self.draw_lines(&points.into()[..]).unwrap();

        Ok(())
    }

    fn draw_rectangle_with_color(&mut self, point: Point, width: i32, height: i32, color: Color) -> Result<(), String> {
        self.set_draw_color(color);

        let points: [Point; 5] = [
            Point::new(point.x, point.y), 
            Point::new(point.x + width, point.y), 
            Point::new(point.x + width, point.y + height), 
            Point::new(point.x, point.y + height),
            Point::new(point.x, point.y)
            ];
        
        self.draw_lines(&points[..]).unwrap();

        Ok(())
    }
}

impl DrawCircle for Canvas<Window> {
    fn draw_circle_with_color(&mut self, center: Point, radius: f32, color: Color) -> Result<(), String> {
        self.set_draw_color(color);
        let quantity = radius * 20.0;
        
        let mut points: Vec<Point> = vec![];
        
        for i in 0..quantity as i32 {
            let rr = (2 * i) as f32;
            let point = Point::new(
                (radius * (rr * PI / quantity).cos()).round() as i32,
                (radius * (rr * PI / quantity).sin()).round() as i32,
            );


            let point = point.add(Point::new(center.x, center.y));
            points.push(point);
        }
        
        self.draw_points(&points[..]).unwrap();

        Ok(())
    }

    fn draw_circle_with_color_thick(&mut self, center: Point, radius: f32, thickness: i8, color: Color) -> Result<(), String> {
        self.set_draw_color(color);
        
        let mut points: Vec<Point> = vec![];
        
        for thickness_index in 0..thickness {
            let radius = radius + thickness_index as f32;
            let quantity = radius * 20.0;
            for i in 0..quantity as i32 {
                let rr = (2 * i) as f32;
                let point = Point::new(
                    (radius * (rr * PI / quantity).cos()).round() as i32,
                    (radius * (rr * PI / quantity).sin()).round() as i32,
                );
                
                
                let point = point.add(Point::new(center.x, center.y));
                points.push(point);
            }
        }
        
        self.draw_points(&points[..]).unwrap();

        Ok(())
    }

    fn draw_circle_with_color_filled(&mut self, center: Point, radius: f32, color: Color) -> Result<(), String> {
        self.set_draw_color(color);
        
        let mut points: Vec<Point> = vec![];

        points.push(Point::new(center.x, center.y));
        
        for thickness_index in 0..radius as i32 {
            let radius = thickness_index as f32;
            let quantity = radius * 20.0;
            for i in 0..quantity as i32 {
                let rr = (2 * i) as f32;
                let point = Point::new(
                    (radius * (rr * PI / quantity).cos()).round() as i32,
                    (radius * (rr * PI / quantity).sin()).round() as i32,
                );
                
                
                let point = point.add(Point::new(center.x, center.y));
                points.push(point);
            }
        }
        
        self.draw_points(&points[..]).unwrap();

        Ok(())
    }
}