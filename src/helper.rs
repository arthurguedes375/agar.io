use sdl2::event::Event;


use crate::game::Game;

pub enum G2UMessage {
    StateUpdate(Game)
}

pub enum U2GMessage {
    Event(Event),
    Quit,
}