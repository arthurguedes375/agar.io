use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render;
use sdl2::image::{InitFlag};
use sdl2::pixels::Color;
use sdl2::render::Texture;

use sdl2::gfx::primitives::DrawRenderer;

use sdl2::rect::Rect;
use sdl2::ttf::Font;

use std::sync::mpsc::{Sender, Receiver};
use std::path::Path;

use crate::helper::{G2UMessage, U2GMessage};
use crate::settings;
use crate::geometry::{Position, rectangle::{Rectangle}};
use crate::game;

use game::{Game};

type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub struct UiSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Ui {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl Ui {
    pub fn new(ui_settings: UiSettings) -> Ui {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG);

        let window = video_subsystem
            .window(&ui_settings.title, ui_settings.width, ui_settings.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        Ui {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
        }
    }

    fn inputs(&mut self, tx: &Sender<U2GMessage>) {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {
                    ..
                } => {
                    tx.send(U2GMessage::Quit).unwrap();
                }
                _ => {
                    tx.send(U2GMessage::Event(event)).unwrap();
                }
            }
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
        
            let target = Rect::new(position.x, position.y + line_height as i32 * line_i as i32, width, height);
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
                    sprite_rectangle.position.x,
                    sprite_rectangle.position.y,
                    sprite_rectangle_size.width,
                    sprite_rectangle_size.height,
                )
            ),
            Some(
                Rect::new(
                    target_corners.top_left.x,
                    target_corners.top_left.y,
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

    fn draw(&mut self) {
        DrawRenderer::aa_trigon(
            &self.canvas,
            self.event_pump.mouse_state().x() as i16, 
            self.event_pump.mouse_state().y() as i16,
            settings::WINDOW_WIDTH as i16 / 2 ,
            settings::WINDOW_HEIGHT as i16 / 2,
            self.event_pump.mouse_state().x() as i16,
            settings::WINDOW_HEIGHT as i16 / 2,
            Color::CYAN
        ).unwrap();
    }

    pub fn run(&mut self, tx: &Sender<U2GMessage>, rx: &Receiver<G2UMessage>) {
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

        for message in rx.iter() {
            self.inputs(tx);

            let G2UMessage::StateUpdate(mut game) = message;
            let game = &mut game;

            self.draw_background();

            self.draw();

            if game.debugging {
                self.debug(
                    game,
                    &debug_font,
                    &texture_creator,
                );
            }


            self.canvas.present();
        }
    }

    pub fn debug(
        &mut self,
        game: &mut Game,
        debug_font: &Font,
        texture_creator: &TextureCreator,
    ) {
        if game.debug_options.game_state {
            let fps = game.fps;

            let info_text = format!(
"FPS: {fps}",
fps=fps,
);

            self.write_text(
                &info_text,
                settings::DEBUG_COLOR,
                Position {
                    x: 10,
                    y: 10,
                }, 
                debug_font,
                texture_creator,
                None,
            );
        }
    }
}