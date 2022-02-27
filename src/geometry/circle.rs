use super::Position;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Position,
    pub radius: u32,
}

impl Circle {
    pub fn holds(&self, target: Position) -> bool {
        let distance = self.center.get_distance(target);
        return distance < self.radius;
    }
}