pub mod rectangle;
pub mod triangle;
pub mod circle;

use triangle::Triangle;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn get_distance(&self, target: Position) -> u32 {
        let triangle = Triangle {
            p1: *self,
            p2: target,
            p3: Position {
                x: target.x,
                y: self.y,
            },
        };

        Triangle::get_hypotenuses_size(
            (triangle.p1.x as i32 - triangle.p3.x as i32).abs() as u32,
            (triangle.p3.y as i32 - triangle.p2.y as i32).abs() as u32
        )
    }
}