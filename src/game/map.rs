use std::collections::HashMap;

use crate::settings;
use crate::{geometry, geometry::Position, geometry::circle::Circle};

use super::{fruit::Fruit, player::Player};

#[derive(Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub players: HashMap<String, Player>,
    pub fruits: Vec<Fruit>,
}

impl Map {
    pub fn new() -> Map {
        let width = settings::MAP_WIDTH;
        let height = settings::MAP_HEIGHT;
        let fruits = Fruit::generate_many(settings::FRUITS, width, height);
        let players = HashMap::new();

        return Map {
            fruits,
            players,
            width,
            height,
        }
    }
}

pub type MapView = geometry::rectangle::Rectangle;

impl MapView {

    pub fn is_visible(&self, circle: Circle) -> bool {
        self.contains_position(circle.center) || circle.holds(self.closest_position_within(circle.center))
    }

    pub fn map_position(&self, position: Position) -> Position {
        let top_left = self.get_corners().top_left;

        let mapped_pos = Position {
            x: position.x - top_left.x,
            y: position.y - top_left.y,
        };

        return mapped_pos;
    }

    pub fn get_visible_fruits(&self, map: &Map) -> Vec<Fruit> {
        map.fruits
            .iter()
            .cloned()
            .filter(|fruit| self.is_visible(fruit.clone()))
            .collect()
    }
    pub fn get_visible_players(&self, map: &Map) -> Vec<Player> {
        map.players
            .values()
            .cloned()
            .filter(|player| {
                let body_parts = player.body_parts.clone();
                for body_part in body_parts {
                    if self.is_visible(body_part) {
                        return true;
                    }
                }

                return false;
            })
            .map(|player| {
                let body_parts = player.body_parts.clone();
                let mapped_body_parts = body_parts
                    .iter()
                    .map(|body_part| Circle {
                        center: self.map_position(body_part.center),
                        ..*body_part
                    })
                    .collect();


                return Player {
                    body_parts: mapped_body_parts,
                    ..player
                };
            })
            .collect()
    }

}