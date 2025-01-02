use std::{path::Path, time::Duration};

use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    game::Game,
    utilities::{render_bg, render_text, Gamemode, Theme},
};

pub struct Menu<'a> {
    sdl_context: &'a sdl2::Sdl,
    ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    font: sdl2::ttf::Font<'a, 'static>,
    canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &'a mut sdl2::EventPump,
    theme: &'a Theme,
    options: Vec<String>,
    current_index: usize,
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
            current_index: 0,
        })
    }

    fn render_options(&mut self) {
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
            let prefix = match option {
                _ if *option == self.options[self.current_index] => "> ".to_string(),
                _ => "  ".to_string(),
            };

            let print_string = prefix + option;

            let _ = render_text(
                self.canvas,
                &self.font,
                self.theme.text_color,
                &print_string,
                options_x,
                options_y,
            );

            options_y += 50;
        }
    }

    fn move_index(&mut self, next: bool) {
        if self.options.is_empty() {
            return;
        }

        self.current_index = if next {
            self.current_index.saturating_add(1)
        } else {
            self.current_index.saturating_sub(1)
        }
        .min(self.options.len() - 1);

        self.render_options();
    }

    fn select(&mut self) {
        match self.current_index {
            0 => {
                let repeat_delay: Duration = Duration::from_millis(100);
                let repeat_interval: Duration = Duration::from_millis(20);
                let fall_interval: Duration = Duration::from_millis(20);
                let game_mode = Gamemode::Classic;

                match Game::new(
                    self.sdl_context,
                    self.ttf_context,
                    self.canvas,
                    self.event_pump,
                    self.theme,
                    repeat_delay,
                    repeat_interval,
                    fall_interval,
                    game_mode,
                ) {
                    Ok(mut game) => game.run(),
                    Err(e) => println!("Failed to create game: {}", e),
                }
            }
            _ => {}
        }
    }

    pub fn run(&mut self) {
        self.render_options();

        'running: loop {
            let events: Vec<Event> = self.event_pump.poll_iter().collect();

            for event in events {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => {
                        // Call self.move_index after the borrow ends
                        self.move_index(false);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => {
                        // Call self.move_index after the borrow ends
                        self.move_index(true);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        // Call self.move_index after the borrow ends
                        self.select();
                        println!("selected");
                    }
                    _ => {}
                }
            }
        }
    }
}
