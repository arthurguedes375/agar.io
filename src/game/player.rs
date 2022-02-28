use rand::prelude::*;
use uuid::Uuid;

use crate::helper::U2GMessage;
use std::sync::mpsc::{Sender};

use crate::game::Game;
use crate::geometry::{Position, circle::Circle};
use crate::settings;

#[derive(Clone, Debug)]
pub struct Player {
    pub body_parts: Vec<Circle>,
    pub name: String,
    pub direction: Position,
    pub id: String,
}

impl Player {
    pub fn new(name: &str, rng: &mut rand::prelude::ThreadRng) -> Player {
        let id = Uuid::new_v4().to_string();
        let body_parts = vec![
            Circle {
                center: Position {
                    x: rng.gen_range(0..settings::MAP_WIDTH) as f32,
                    y: rng.gen_range(0..settings::MAP_HEIGHT) as f32,
                },
                radius: settings::INITIAL_PLAYER_SCORE,
            }
        ];

        let player = Player {
            id: id.clone(),
            body_parts,
            direction: Position {
                x: 0.0,
                y: 0.0,
            },
            name: name.to_string(),
        };
        
        return player;
    }

    pub fn connect(&self, tx: &Sender<U2GMessage>) {
        tx.send(U2GMessage::NewPlayer(self.clone())).unwrap();
    }

    pub fn get(id: Option<String>, game: &Game) -> Option<Player> {
        let player_id = match id.clone() {
            Some(id) => id,
            None => return None,
        };
        let player = match game.map.players.get(&player_id) {
            Some(player) => player,
            None => return None,
        };

        return Some(player.clone());
    }
}