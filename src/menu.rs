use std::{path::Path, time::Duration};

use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    game::Game,
    utilities::{render_bg, render_center_box, render_text, Gamemode, Theme},
};

pub enum MenuOption<'a> {
    Action { name: String, action: &'a dyn Fn() },
    Submenu { name: String, submenu_index: usize },
}

pub struct MenuNode<'a> {
    pub title: String,
    pub options: Vec<MenuOption<'a>>,
    pub parent: Option<usize>,
}

pub struct MenuManager<'a> {
    sdl_context: &'a sdl2::Sdl,
    ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    font: sdl2::ttf::Font<'a, 'static>,
    canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: &'a mut sdl2::EventPump,
    theme: &'a Theme,
    menus: Vec<MenuNode<'a>>,
    current_menu: usize,
    current_index: usize,
}

impl<'a> MenuManager<'a> {

    const CELL_SIZE: u32 = 40;
    const GRID_WIDTH: u32 = 10;
    const GRID_HEIGHT: u32 = 20;

    pub fn new(
        sdl_context: &'a sdl2::Sdl,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: &'a mut sdl2::EventPump,
        theme: &'a Theme,
        menus: Vec<MenuNode<'a>>,
    ) -> Result<Self, String> {
        let font_path = Path::new(&"assets/FreeMono.ttf");
        let font = ttf_context.load_font(font_path, 22)?;

        Ok(MenuManager {
            sdl_context,
            ttf_context,
            font,
            canvas,
            event_pump,
            theme,
            menus,
            current_menu: 0,
            current_index: 0,
        })
    }

    pub fn navigate_to_submenu(&mut self, submenu_index: usize) {
        self.current_menu = submenu_index;
    }

    pub fn back_to_parent(&mut self) {
        if let Some(parent_index) = self.menus[self.current_menu].parent {
            self.current_menu = parent_index;
        }
    }

    pub fn select_option(&mut self, option_index: usize) {
        match &self.menus[self.current_menu].options[option_index] {
            MenuOption::Action { action, .. } => action(),
            MenuOption::Submenu { submenu_index, .. } => self.navigate_to_submenu(*submenu_index),
        }
    }

    fn render_current_menu(&mut self) {
        let menu = &self.menus[self.current_menu];
        let mut options_y = 300;

        render_bg(
            self.canvas,
            self.theme.bg_color_1,
            self.theme.bg_color_2,
            Self::CELL_SIZE,
            Self::GRID_WIDTH,
            Self::GRID_HEIGHT,
        );

        for (index, option) in menu.options.iter().enumerate() {
            let prefix = if index == self.current_index {
                "> "
            } else {
                "  "
            };
            let name = match option {
                MenuOption::Action { name, .. } => name,
                MenuOption::Submenu { name, .. } => name,
            };

            let display_text = format!("{}{}", prefix, name);

            // Replace with your actual render_text function
            let _ = render_text(
                self.canvas,
                &self.font,
                self.theme.text_color,
                &display_text,
                300,
                options_y,
            );
            options_y += 50;
        }
    }

    fn move_index(&mut self, next: bool) {
        self.current_index = if next {
            self.current_index.saturating_add(1)
        } else {
            self.current_index.saturating_sub(1)
        }
        .min(self.menus[self.current_menu].options.len() - 1);

        self.render_current_menu();
    }

    pub fn run(&mut self) {
        self.render_current_menu();

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
                        println!("selected");
                        self.select_option(self.current_index);
                    }
                    _ => {}
                }
            }
        }
    }
}

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

                self.render_options();
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
