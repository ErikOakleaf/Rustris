use std::path::Path;

use sdl2::{event::Event, keyboard::Keycode};

use crate::utilities::{render_bg, render_text, Theme};

pub struct Menu<'a> {
    sdl_context: &'a sdl2::Sdl,
    ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    font: sdl2::ttf::Font<'a, 'static>,
    canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &'a mut sdl2::EventPump,
    theme: &'a Theme,
    options: Vec<String>,
}

impl<'a> Menu<'a> {
    const CELL_SIZE: u32 = 40;
    const GRID_WIDTH: u32 = 10;
    const GRID_HEIGHT: u32 = 20;

    pub fn new(
        sdl_context: &'a sdl2::Sdl,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: &'a mut sdl2::EventPump,
        theme: &'a Theme,
        options: Vec<String>,
    ) -> Result<Self, String> {
        let font_path = Path::new(&"assets/FreeMono.ttf");
        let font = ttf_context.load_font(font_path, 22)?;

        Ok(Menu {
            sdl_context,
            ttf_context,
            font,
            canvas,
            event_pump,
            theme,
            options,
        })
    }

    pub fn render_options(&mut self) {
        render_bg(
            self.canvas,
            self.theme.bg_color_1,
            self.theme.bg_color_2,
            Self::CELL_SIZE,
            Self::GRID_WIDTH,
            Self::GRID_HEIGHT,
        );

        let options_x = 300;
        let mut options_y = 300;

        for option in self.options.iter() {
            let _ = render_text(self.canvas, &self.font, self.theme.text_color, &option, options_x, options_y);

            options_y += 50;
        }
    }

    pub fn run(&mut self) {
        self.render_options();

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }
    }
}
