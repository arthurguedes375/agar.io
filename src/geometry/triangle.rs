use super::Position;

pub struct Triangle {
    pub p1: Position,
    pub p2: Position,
    pub p3: Position,
}

impl Triangle {
    pub fn get_hypotenuses_size(a: f32, b: f32) -> f32 {
        return (a.powf(2.0) + b.powf(2.0)).powf(0.5);
    }
}