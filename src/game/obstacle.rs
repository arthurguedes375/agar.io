use crate::geometry::circle::Circle;

pub type Obstacle = Circle;

impl Obstacle {
    pub fn init() -> Vec<Obstacle> {
        return vec![];
    }
}