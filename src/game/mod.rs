use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::mpsc::{Sender, Receiver};

use crate::time;
use crate::settings;
use crate::helper::{G2UMessage, U2GMessage};


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

    fn update(&mut self) {
        
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
                U2GMessage::Event(event) => {
                    match event {
        
                        Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            match self.status {
                                Status::Running => self.status = Status::Paused,
                                Status::Paused => self.status = Status::Running,
        
                                _ => {}
                            }
                        }

                        Event::Quit {..} => {
                            self.status = Status::Closed;
                        }
                        _ => {}
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