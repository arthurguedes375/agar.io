use std::sync::mpsc::{Sender, Receiver};

use rand::prelude::thread_rng;

use crate::time;
use crate::settings;
use crate::helper::{G2UMessage, U2GMessage, PlayerEvent};
use crate::geometry::{Position, rectangle::{Rectangle, RectangleSize, Size}};

// Mods
pub mod map;
pub mod player;
pub mod fruit;
pub mod obstacle;


use map::Map;

#[derive(Clone)]
pub enum Status {
    Running,
    Paused,
    Closed,
}

#[derive(Clone)]
pub struct Game {
    pub map: Map,
    pub status: Status,
    pub last_frame_timestamp: u128,
    pub fps: u16,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            map: Map::new(),
            status: Status::Running,
            last_frame_timestamp: time::now(),
            fps: 0,
        };
    }

    fn update_fps(&mut self) {
        self.fps = Game::get_fps(self.last_frame_timestamp);
        self.last_frame_timestamp = time::now();
    }

    fn moving(&mut self, player_id: String, direction: Position) {
        let mut player = player::Player::get(Some(player_id.clone()), self).unwrap();

        player.direction = direction;

        *self.map.players.get_mut(&player_id).unwrap() = player;
    }

    fn move_players(&mut self) {
        let players = self.map.players.values_mut();

        for player in players {
            let player_score = player.get_score();
            for body_part in player.body_parts.iter_mut() {
                let delta = self.fps as f32 / (10.0 * player_score as f32);

                let future_position = Game::future_position(
                    body_part.center, Position {
                        x: player.direction.x * delta,
                        y: player.direction.y * delta,
                    });
                
                let map_rect = Rectangle {
                    position: Position {
                        x: settings::MAP_WIDTH as f32 / 2.0,
                        y: settings::MAP_HEIGHT as f32 / 2.0,
                    },
                    size: Size::Rectangle(RectangleSize {
                        width: settings::MAP_WIDTH,
                        height: settings::MAP_HEIGHT,
                    }),
                };

                if map_rect.contains_position(future_position) {
                    body_part.center = future_position;
                }
            }
        }
    }

    fn future_position(position: Position, speed: Position) -> Position {
        Position {
            x: position.x + speed.x,
            y: position.y + speed.y,
        }
    } 

    fn check_fruit_collision(&mut self) {
        let mut fruits = self.map.fruits.clone();
        let players = self.map.players.values_mut();

        for player in players {
            for body_part in player.body_parts.iter_mut() {
                fruits = fruits.iter()
                    .cloned()
                    .map(|fruit| {
                        if body_part.holds(fruit.center) {
                            body_part.radius += fruit.radius / 10;
                            let mut rng = thread_rng();
                            return fruit::Fruit::new(&mut rng, settings::MAP_WIDTH, settings::MAP_HEIGHT);
                        }

                        return fruit;
                    })
                    .collect();
            }
        }

        self.map.fruits = fruits;
    }
    fn check_player_collision(&mut self) {

    }

    fn check_object_collision(&mut self) {
        self.check_fruit_collision();
    }

    fn check_collision(&mut self) {
        self.check_object_collision();
    }

    fn update(&mut self) {
        self.move_players();
        self.check_collision();
    }

    pub fn get_fps(last_frame_timestamp: u128) -> u16 {
        let fps = (1_000_000_000 / (time::now() - last_frame_timestamp)) as u16;
        return fps;
    }

    fn get_inputs(&mut self, rx: &Receiver<U2GMessage>) {
        let rx_message = rx.try_iter();

        for message in rx_message {
            match message {
                U2GMessage::NewPlayer(player) => {
                    self.map.players.insert(player.id.clone(), player);
                }
                U2GMessage::PlayerEvent(player_id, event) => {
                    match event {
                        PlayerEvent::Moving(direction) => {
                            self.moving(player_id, direction);
                        }
                    }
                }
                U2GMessage::Quit => {
                    self.status = Status::Closed;
                }
            }
        }
    }

    pub fn delay_fps(last_frame_timestamp: u128, max_fps: u16) {
        let now = time::now();
        let last_frame = now - last_frame_timestamp;

        let min_fps_nano = 1_000_000_000 / max_fps as u128;

        if last_frame < min_fps_nano {
            let sleep_for =  min_fps_nano - last_frame;
            std::thread::sleep(std::time::Duration::from_nanos(sleep_for as u64));
        }
    }

    pub fn init(&mut self, tx: &Sender<G2UMessage>, rx: &Receiver<U2GMessage>) {
        'main_loop: loop {
            match self.status {
                Status::Running | Status::Paused => {
                    self.get_inputs(rx);
                    
                    if let Status::Running = &self.status {
                        self.update()
                    }
                    
                    tx.send(G2UMessage::StateUpdate(self.clone())).unwrap();

                    Game::delay_fps(self.last_frame_timestamp, settings::MAX_FPS + 1);
                    self.update_fps();
                }
                _ => {
                    break 'main_loop;
                }
            }
        }
    }
}