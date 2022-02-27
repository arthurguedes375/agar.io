use super::Position;

pub struct Triangle {
    pub p1: Position,
    pub p2: Position,
    pub p3: Position,
}

impl Triangle {
    pub fn get_hypotenuses_size(a: u32, b: u32) -> u32 {
        return ((a.pow(2) + b.pow(2)) as f64).powf(0.5) as u32;
    }
}