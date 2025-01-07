use std::path::Path;

use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
};

use crate::utilities::{render_bg, render_text, Settings, Theme};

#[derive(Clone)]
pub enum InteractionType<'a> {
    Toggle(&'a dyn Fn(&mut MenuManager<'a>)),
    Scrollable(&'a dyn Fn(&mut MenuManager<'a>, bool)),
    Scancode(&'a str),
}

#[derive(Clone)]
pub enum MenuOption<'a> {
    Action {
        name: String,
        dynamic_value: Option<&'a dyn Fn(&MenuManager<'a>) -> String>,
        action: InteractionType<'a>,
    },
    Submenu {
        name: String,
        submenu_index: usize,
    },
    Back {
        name: String,
    },
}

pub struct MenuNode<'a> {
    pub title: String,
    pub options: Vec<MenuOption<'a>>,
    pub parent: Option<usize>,
}

pub struct MenuManager<'a> {
    pub sdl_context: &'a sdl2::Sdl,
    pub ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    pub font: sdl2::ttf::Font<'a, 'static>,
    pub canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: &'a mut sdl2::EventPump,
    pub theme: Theme,
    menus: Vec<MenuNode<'a>>,
    current_menu: usize,
    current_index: usize,
    pub settings: Settings,
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
        theme: Theme,
        menus: Vec<MenuNode<'a>>,
    ) -> Result<Self, String> {
        let font_path = Path::new(&"assets/FreeMono.ttf");
        let font = ttf_context.load_font(font_path, 22)?;

        let settings = Settings::new().unwrap();

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
            settings,
        })
    }

    pub fn navigate_to_submenu(&mut self, submenu_index: usize) {
        self.current_menu = submenu_index;
        self.render_current_menu();
    }

    pub fn back_to_parent(&mut self) {
        if let Some(parent_index) = self.menus[self.current_menu].parent {
            self.current_menu = parent_index;
            self.render_current_menu();
        }
    }

    pub fn select_option(&mut self, option_index: usize) {
        let option = self.menus[self.current_menu].options[option_index].clone();

        match option {
            MenuOption::Action { action, .. } => match action {
                InteractionType::Toggle(toggle_action) => {
                    toggle_action(self);
                    self.render_current_menu();
                }
                InteractionType::Scancode(scancode_string) => {
                    let new_scancode = self.get_key_press();

                    self.settings
                        .key_bindings
                        .update_binding(scancode_string, new_scancode);

                    self.render_current_menu();
                }
                _ => {}
            },
            MenuOption::Submenu { submenu_index, .. } => {
                self.navigate_to_submenu(submenu_index);
                self.current_index = 0;
                self.render_current_menu();
            }
            MenuOption::Back { .. } => {
                self.back_to_parent();
                self.current_index = 0;
                self.render_current_menu();
            }
        }
    }

    pub fn scroll_option(&mut self, option_index: usize, direction: bool) {
        let option = self.menus[self.current_menu].options[option_index].clone();

        match option {
            MenuOption::Action { action, .. } => match action {
                InteractionType::Scrollable(scroll_action) => {
                    scroll_action(self, direction);
                    self.render_current_menu();
                }
                _ => {}
            },
            _ => {}
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

        // render title

        let title_text = menu.title.clone();

        let _ = render_text(
            self.canvas,
            &self.font,
            self.theme.text_color,
            &title_text,
            320,
            50,
        );

        // render options

        for (index, option) in menu.options.iter().enumerate() {
            let prefix = if index == self.current_index {
                "> "
            } else {
                "  "
            };
            let name = match option {
                MenuOption::Action {
                    name,
                    dynamic_value,
                    ..
                } => {
                    if let Some(getter) = dynamic_value {
                        // If there's a dynamic value, append it to the name
                        format!("{}: {}", name, getter(self))
                    } else {
                        name.clone()
                    }
                }
                MenuOption::Submenu { name, .. } => name.clone(),
                MenuOption::Back { name, .. } => name.clone(),
            };

            let display_text = format!("{}{}", prefix, name);

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

    pub fn get_key_press(&mut self) -> Scancode {
        render_bg(
            self.canvas,
            self.theme.bg_color_1,
            self.theme.bg_color_2,
            Self::CELL_SIZE,
            Self::GRID_WIDTH,
            Self::GRID_HEIGHT,
        );

        let _ = render_text(
            self.canvas,
            &self.font,
            self.theme.text_color,
            &"Press Key".to_string(),
            350,
            400,
        );
        loop {
            let events: Vec<Event> = self.event_pump.poll_iter().collect();

            for event in events {
                if let Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } = event
                {
                    return scancode;
                }
            }
        }
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
                        self.move_index(false);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => {
                        self.move_index(true);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        self.select_option(self.current_index);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        self.scroll_option(self.current_index, false);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        self.scroll_option(self.current_index, true);
                    }
                    _ => {}
                }
            }
        }
    }
}
