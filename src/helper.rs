use crate::game::{player::Player, Game};

use crate::geometry::Position;

pub enum G2UMessage {
    StateUpdate(Game)
}

pub enum PlayerEvent {
    Moving(Position),
}

pub enum U2GMessage {
    PlayerEvent(String, PlayerEvent),
    NewPlayer(Player),
    Quit,
}