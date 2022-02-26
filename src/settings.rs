use sdl2::pixels::Color;

use crate::game::DebugOptions;

// Window
pub const WINDOW_TITLE: &str = "";
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
};