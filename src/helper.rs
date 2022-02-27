use sdl2::event::Event;

use crate::game::{player::Player, Game};

pub enum G2UMessage {
    StateUpdate(Game)
}

pub enum U2GMessage {
    Event(Event),
    NewPlayer(Player),
    Quit,
}