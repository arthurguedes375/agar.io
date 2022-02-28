use crate::geometry::{circle::Circle, Position};

use crate::settings;

use rand::prelude::*;

pub type Fruit = Circle;

impl Fruit {

    pub fn new(rng: &mut rand::prelude::ThreadRng, width: u32, height: u32) -> Fruit {
        Fruit {
            center: Position {
                x: rng.gen_range(0..width) as f32,
                y: rng.gen_range(0..height) as f32,
            },
            radius: settings::FRUIT_RADIUS,
        }
    }

    pub fn generate_many(amount: u16, width: u32, height: u32) -> Vec<Fruit> {
        let mut rng = rand::thread_rng();
        
        let mut fruits: Vec<Fruit> = vec![];

        for _ in 0..amount {
            fruits.push(Fruit::new(&mut rng, width, height));
        }

        return fruits;
    }
}