use sdl2::pixels::Color;

use crate::ui::DebugOptions;

// Window
pub const WINDOW_TITLE: &str = "Agar.io";
pub const WINDOW_WIDTH: u32 = 600;
pub const WINDOW_HEIGHT: u32 = 600;
pub const MAX_FPS: u16 = 300;

// Sprites
pub const SPRITES_FOLDER_PATH: &str = "./assets/sprites";

// Debugging
pub const DEFAULT_DEBUGGING_STATE: bool = true;
pub const DEBUG_FONT_PATH: &str = "./assets/fonts/debug.ttf";
pub const DEBUG_FONT_POINT_SIZE: u16 = 15;
pub const DEBUG_COLOR: Color = Color::MAGENTA;
pub const DEFAULT_DEBUG_OPTIONS: DebugOptions = DebugOptions {
    game_state: true,
    map_view: true,
};

// Font
pub const GAME_FONT_PATH: &str = "./assets/fonts/game.ttf";
pub const GAME_FONT_POINT_SIZE: u16 = 15;

// Map
pub const MAP_WIDTH: u32 = 7000;
pub const MAP_HEIGHT: u32 = 7000;

// Player
pub const INITIAL_PLAYER_SCORE: u32 = 20;

// Fruits
pub const FRUITS: u16 = 600;
pub const FRUIT_RADIUS: u32 = 10;