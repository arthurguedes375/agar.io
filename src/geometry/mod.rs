pub mod rectangle;
pub mod triangle;
pub mod circle;

use triangle::Triangle;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn get_triangle_to(&self, target: Position) -> Triangle {
        Triangle {
            p1: *self,
            p2: target,
            p3: Position {
                x: target.x,
                y: self.y,
            },
        }
    }
    pub fn get_distance(&self, target: Position) -> f32 {
        let triangle = self.get_triangle_to(target);

        Triangle::get_hypotenuses_size(
            (triangle.p1.x - triangle.p3.x).abs(),
            (triangle.p3.y - triangle.p2.y).abs(),
        )
    }
}