use std::ops::{Add, Sub};
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub trait AdditionalDrawMethods {
    fn draw_circle(&mut self, p: Point, r: i32) -> Result<(), String>;
}

impl AdditionalDrawMethods for WindowCanvas {
    fn draw_circle(&mut self, p: Point, r: i32) -> Result<(), String> {
        let d = r*2;
        let mut x = r - 1;
        let mut y = 0;
        let mut tx = 1;
        let mut ty = 1;
        let mut error = tx - d;

        while x >= y {
            self.draw_line(Point::new(p.x.add(x), p.y.sub(y)), p)?;
            self.draw_line(Point::new(p.x.add(x), p.y.add(y)), p)?;
            self.draw_line(Point::new(p.x.sub(x), p.y.sub(y)), p)?;
            self.draw_line(Point::new(p.x.sub(x), p.y.add(y)), p)?;
            self.draw_line(Point::new(p.x.add(y), p.y.sub(x)), p)?;
            self.draw_line(Point::new(p.x.add(y), p.y.add(x)), p)?;
            self.draw_line(Point::new(p.x.sub(y), p.y.sub(x)), p)?;
            self.draw_line(Point::new(p.x.sub(y), p.y.add(x)), p)?;

            if error <= 0 {
                y += 1;
                error += ty;
                ty += 2;
            }
            if error > 0 {
                x -= 1;
                tx += 2;
                error += tx - d;
            }
        }
        Ok(())
    }
}

pub trait AdditionalPointMethods {
    fn p(x: u32, y: u32) -> Self;
}

impl AdditionalPointMethods for Point {
    fn p(x: u32, y: u32) -> Self {
        Point::new((x * 100 + 100) as i32, (y * 100 + 100) as i32)
    }
}