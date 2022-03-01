use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render;
use sdl2::image::{InitFlag};
use sdl2::pixels::Color;
use sdl2::render::Texture;

use rand::{prelude::*, thread_rng};

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;

use sdl2::rect::Rect;
use sdl2::ttf::Font;

use std::sync::mpsc::{Sender, Receiver};
use std::path::Path;

use crate::helper::{G2UMessage, U2GMessage, PlayerEvent};
use crate::settings;
use crate::geometry;
use geometry::{Position, rectangle::{Rectangle, Size, RectangleSize}, circle::Circle};
use crate::game;

use game::map::MapView;
use game::player::Player;

use game::{Game};

type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub struct UiSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct DebugOptions {
    pub game_state: bool,
    pub map_view: bool,
}

pub struct Ui {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub map_view: MapView,
    pub player_name: String,
    pub player_id: Option<String>,
    pub debug_options: DebugOptions,
    pub debugging: bool,
}

impl Ui {
    pub fn new(player_name: &str, ui_settings: UiSettings) -> Ui {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG);

        let window = video_subsystem
            .window(&ui_settings.title, ui_settings.width, ui_settings.height)
            .opengl()
            .resizable()
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let map_view = MapView {
            position: Position {
                x: settings::MAP_WIDTH as f32 / 2.0,
                y: settings::MAP_HEIGHT as f32 / 2.0,
            },
            size: Size::Rectangle(RectangleSize {
                width: settings::WINDOW_WIDTH,
                height: settings::WINDOW_HEIGHT,
            }),
        };

        Ui {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            map_view,

            player_name: player_name.to_string(),
            player_id: None,

            debug_options: settings::DEFAULT_DEBUG_OPTIONS,
            debugging: settings::DEFAULT_DEBUGGING_STATE,
        }
    }

    fn inputs(&mut self, tx: &Sender<U2GMessage>, game: &Game) {
        let mut events = vec![];
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    self.map_view.size = Size::Rectangle(RectangleSize {
                            width: width as u32,
                            height: height as u32,
                        }
                    );
                }

                Event::MouseMotion {
                    x, y,
                    ..
                } => {
                    let mut player = match Player::get(self.player_id.clone(), game) {
                        Some(player) => player,
                        None => continue,
                    };

                    let mouse_pos = Position {
                        x: x as f32,
                        y: y as f32,
                    };

                    player.body_parts[0].center = self.map_view.map_position(player.body_parts[0].center);

                    let angle = player.body_parts[0].angle_to(mouse_pos);

                    let coordinates_to = Circle::angle_to_coordinates(angle);

                    tx.send(
                        U2GMessage::PlayerEvent(
                            player.id.clone(),
                            PlayerEvent::Moving(coordinates_to)
                        )
                    ).unwrap();

                }

                Event::Quit {
                    ..
                } => {
                    tx.send(U2GMessage::Quit).unwrap();
                }
                _ => {
                    events.push(event.clone());
                }
            }
        }

        for event in events {
            self.debug_events(&event);
        }
    }

    fn write_text(
        &mut self,
        text: &str,
        color: Color,
        position: Position,
        font: &Font,
        texture_creator: &TextureCreator,
        line_height: Option<u16>,
    ) {
        let text_lines: Vec<&str> = text.split("\n").filter(|line| line.len() > 0).collect();
        let line_height = line_height.unwrap_or(15);

        for (line_i, &line) in text_lines.iter().enumerate() {
            let surface = font
                .render(line)
                .blended(color)
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();

            let render::TextureQuery { width, height, .. } = texture.query();
        
            let target = Rect::new(
                position.x as i32,
                position.y as i32 + line_height as i32 * line_i as i32,
                width,
                height
            );
            self.canvas.copy(&texture, None, Some(target)).unwrap();

        }

        
    }

    fn _draw_sprite(
        &mut self,
        texture: &Texture,
        sprite_rectangle: Rectangle,
        target_rectangle: Rectangle,
    ) {
        let sprite_rectangle_size = Rectangle::to_rectangle_size(sprite_rectangle.size);
        
        let target_corners = target_rectangle.get_corners();
        let target_rectangle_size = Rectangle::to_rectangle_size(target_rectangle.size);

        self.canvas.copy(
            texture,
            Some(
                Rect::new(
                    sprite_rectangle.position.x as i32,
                    sprite_rectangle.position.y as i32,
                    sprite_rectangle_size.width,
                    sprite_rectangle_size.height,
                )
            ),
            Some(
                Rect::new(
                    target_corners.top_left.x as i32,
                    target_corners.top_left.y as i32,
                    target_rectangle_size.width,
                    target_rectangle_size.height,
                )
            )
        ).unwrap();
    }

    fn draw_background(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 32));
        self.canvas.clear();
    }

    fn circle(&mut self, circle: geometry::circle::Circle, color: Color, filled: bool) {
        if filled {
            DrawRenderer::filled_circle(
                &self.canvas,
                circle.center.x as i16,
                circle.center.y as i16,
                circle.radius as i16,
                color,
            ).unwrap();
        } else {
            DrawRenderer::aa_circle(
                &self.canvas,
                circle.center.x as i16,
                circle.center.y as i16,
                circle.radius as i16,
                color,
            ).unwrap();
        }
    }

    fn draw_fruits(&mut self, game: &mut game::Game) {
        let fruits = self.map_view.get_visible_fruits(&game.map);
        for fruit in fruits {
            let mapped_pos = self.map_view.map_position(fruit.center);
            
            let (r, g, b) = (
                (fruit.center.x % 200.0) as u8,
                (fruit.center.y % 200.0) as u8,
                ((fruit.center.x * fruit.center.y) % 255.0) as u8,
            );
            self.circle(
                Circle {
                    center: mapped_pos,
                    ..fruit
                },
                Color::RGB(r, g, b),
                true
            );
        }
    }

    fn draw_player(
        &mut self,
        game: &mut game::Game,
        font: &Font,
        texture_creator: &TextureCreator,
    ) {
        let player = match Player::get(self.player_id.clone(), game) {
            Some(player) => player,
            None => return,
        };
        
        let players_name_bytes = player.name.as_bytes();

        let mut player_bytes_count = 0.0;
        
        for byte in players_name_bytes {
            player_bytes_count += *byte as f32;
        }

        let (r, g, b) = (
            (player_bytes_count % 255.0) as u8,
            ((player_bytes_count * 1.5) % 255.0) as u8,
            ((player_bytes_count * 0.5) % 255.0) as u8,
        );

        let body_parts = player.body_parts.clone();
        for body_part in body_parts {
            let mapped = Circle {
                center: self.map_view.map_position(body_part.center),
                ..body_part
            };
            self.circle(
                mapped,
                Color::RGB(r, g, b),
                true,
            );
            

            self.write_text(
                &player.name,
                Color::WHITE,
                Position {
                    x: mapped.center.x - body_part.radius as f32,
                    y: mapped.center.y - body_part.radius as f32 - 25.0,
                },
                font,
                texture_creator,
                None)
        }
    }

    fn draw(
        &mut self,
        game: &mut game::Game,
        font: &Font,
        texture_creator: &TextureCreator,
    ) {
        self.draw_fruits(game);
        self.draw_player(game, font, texture_creator);
    }

    pub fn run(&mut self, tx: &Sender<U2GMessage>, rx: &Receiver<G2UMessage>) {
        let mut rng = rand::thread_rng();
        if let None = self.player_id {
            let player = Player::new(&self.player_name, &mut rng);
            self.player_id = Some(player.id.clone());
            player.connect(&tx);
        }

        let ttf_context = sdl2::ttf::init().unwrap();
        let texture_creator = self.canvas.texture_creator();

        // Load Sprites
        // let sprites_texture = texture_creator.load_texture(
        //     Path::new(format!("{}/sprite.png", settings::SPRITES_FOLDER_PATH))
        // ).unwrap();

        // Load debug font
        let mut debug_font = ttf_context.load_font(
            Path::new(settings::DEBUG_FONT_PATH),
            settings::DEBUG_FONT_POINT_SIZE
        ).unwrap();
        debug_font.set_style(sdl2::ttf::FontStyle::NORMAL);

        // Load Game font
        let mut game_font = ttf_context.load_font(
            Path::new(settings::GAME_FONT_PATH),
            settings::GAME_FONT_POINT_SIZE
        ).unwrap();
        game_font.set_style(sdl2::ttf::FontStyle::NORMAL);

        
        for message in rx.iter() {
            let G2UMessage::StateUpdate(mut game) = message;
            let game = &mut game;
            
            self.inputs(tx, game);

            let player_pos = match Player::get(self.player_id.clone(), game) {
                Some(player) => player.body_parts[0].center,
                None => Position {
                    x: settings::MAP_WIDTH as f32 / 2.0,
                    y: settings::MAP_HEIGHT as f32 / 2.0,
                }
            };

            self.map_view.position = player_pos;

            self.draw_background();

            self.draw(game, &game_font, &texture_creator);

            if self.debugging {
                self.debug(
                    game,
                    &debug_font,
                    &texture_creator,
                );
            }


            self.canvas.present();
        }
    }

    pub fn debug_events(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::F5),
                ..
            } => {
                self.debugging = !self.debugging; 
            }
            Event::KeyDown {
                keycode: Some(Keycode::F6),
                ..
            } => {
                self.debug_options = DebugOptions {
                    game_state: !self.debug_options.game_state,
                    ..self.debug_options
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::F7),
                ..
            } => {
                self.debug_options = DebugOptions {
                    map_view: !self.debug_options.map_view,
                    ..self.debug_options
                }
            }

            _ => {}
        }
    }

    pub fn debug(
        &mut self,
        game: &mut Game,
        debug_font: &Font,
        texture_creator: &TextureCreator,
    ) {
        if self.debug_options.game_state {
            let fps = game.fps;

            let info_text = format!("FPS: {fps}");
            self.write_text(
                &info_text,
                settings::DEBUG_COLOR,
                Position {
                    x: 10.0,
                    y: 10.0,
                }, 
                debug_font,
                texture_creator,
                None,
            );
        }

        if self.debug_options.map_view {
            self.canvas.set_draw_color(Color::GREEN);
            let size = Rectangle::to_rectangle_size(self.map_view.size.clone());
            let pos = self.map_view.map_position(self.map_view.position);
            self.canvas.draw_rect(
                sdl2::rect::Rect::from_center(
                    sdl2::rect::Point::from((
                        pos.x as i32,
                        pos.y as i32,
                    )),
                    size.width - 5,
                    size.height - 5,
                )
            ).unwrap();
        }
    }
}