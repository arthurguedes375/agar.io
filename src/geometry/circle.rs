use super::triangle::Triangle;
use super::Position;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Position,
    pub radius: u32,
}

impl Circle {
    pub fn angle_to(&self, target: Position) -> f32 {
        let triangle_to = Triangle {
            p1: self.center,
            p2: target,
            p3: Position {
                x: target.x,
                y: self.center.y,
            },
        };

        let a = triangle_to.p3.y - triangle_to.p2.y;
        let b = triangle_to.p3.x - triangle_to.p1.x;

        let mut angle = (a / b).atan() * (180.0 / 3.14);

        if target.y > self.center.y && target.x > self.center.x {
            angle = 360.0 + angle;
        } else if target.x < self.center.x  {
            angle = 180.0 + angle;
        }

        return angle / (180.0 / 3.14);
    }

    pub fn angle_to_coordinates(angle: f32) -> Position {
        let x = angle.cos();
        let y = -(angle.sin());

        return Position {
            x,
            y,
        };
    }
    
    pub fn holds(&self, target: Position) -> bool {
        let distance = self.center.get_distance(target);
        return distance < self.radius as f32;
    }
}