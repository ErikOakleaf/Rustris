mod game;
mod menu;
mod tetrominos;
mod utilities;

use game::Game;
use menu::{InteractionType, MenuManager, MenuNode, MenuOption};
use sdl2::pixels::Color;
use std::time::Duration;
use utilities::{Gamemode, Theme};

fn main() -> Result<(), String> {
    let mut sdl = init_sdl()?;

    let classic_game = |menu_manager: &mut MenuManager| {
        let fall_interval = Duration::from_millis(20);

        let game = Game::new(
            &menu_manager.sdl_context,
            &menu_manager.ttf_context,
            &mut menu_manager.canvas,
            &mut menu_manager.event_pump,
            &menu_manager.theme,
            fall_interval,
            Gamemode::Classic,
            &menu_manager.settings,
        );

        match game {
            Ok(mut g) => g.run(),
            Err(e) => println!("Failed to start game: {}", e),
        }
    };

    let lines_40_game = |menu_manager: &mut MenuManager| {
        let fall_interval = Duration::from_millis(20);

        let game = Game::new(
            &menu_manager.sdl_context,
            &menu_manager.ttf_context,
            &mut menu_manager.canvas,
            &mut menu_manager.event_pump,
            &menu_manager.theme,
            fall_interval,
            Gamemode::Lines40,
            &menu_manager.settings,
        );

        match game {
            Ok(mut g) => g.run(),
            Err(e) => println!("Failed to start game: {}", e),
        }
    };

    // main menu

    let main_menu = MenuNode {
        title: "Main Menu".to_string(),
        options: vec![
            MenuOption::Action {
                name: "Classic".to_string(),
                dynamic_value: None,
                action: InteractionType::Toggle(&classic_game),
            },
            MenuOption::Action {
                name: "40 Lines".to_string(),
                dynamic_value: None,
                action: InteractionType::Toggle(&lines_40_game),
            },
            MenuOption::Submenu {
                name: "Options".to_string(),
                submenu_index: 1,
            },
            MenuOption::Submenu {
                name: "Controls".to_string(),
                submenu_index: 2,
            },
        ],
        parent: None,
    };

    // options menu

    let options_menu = MenuNode {
        title: "Options".to_string(),
        options: vec![
            MenuOption::Action {
                name: "Bright Mode".to_string(),
                dynamic_value: Some(&|menu_manager| menu_manager.settings.bright_mode.to_string()),
                action: InteractionType::Toggle(&|menu_manager: &mut MenuManager| {
                    menu_manager.settings.bright_mode = !menu_manager.settings.bright_mode;
                    menu_manager.theme = init_theme(menu_manager.settings.bright_mode);
                }),
            },
            MenuOption::Action {
                name: "Instant DAS".to_string(),
                dynamic_value: Some(&|menu_manager| menu_manager.settings.insta_das.to_string()),
                action: InteractionType::Toggle(&|menu_manager: &mut MenuManager| {
                    if menu_manager.settings.insta_das {
                        menu_manager.settings.insta_das = false;
                    } else {
                        menu_manager.settings.insta_das = true;
                    }
                }),
            },
            MenuOption::Action {
                name: "Instant Soft Drop".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager.settings.insta_softdrop.to_string()
                }),
                action: InteractionType::Toggle(&|menu_manager: &mut MenuManager| {
                    if menu_manager.settings.insta_softdrop {
                        menu_manager.settings.insta_softdrop = false;
                    } else {
                        menu_manager.settings.insta_softdrop = true;
                    }
                }),
            },
            MenuOption::Action {
                name: "Repeat Delay".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager.settings.repeat_delay.as_millis().to_string()
                }),
                action: InteractionType::Scrollable(
                    &|menu_manager: &mut MenuManager, increase: bool| {
                        if increase {
                            menu_manager.settings.repeat_delay = menu_manager
                                .settings
                                .repeat_delay
                                .checked_add(Duration::from_millis(1))
                                .unwrap();
                        } else {
                            menu_manager.settings.repeat_delay = menu_manager
                                .settings
                                .repeat_delay
                                .checked_sub(Duration::from_millis(1))
                                .unwrap();
                        }
                    },
                ),
            },
            MenuOption::Action {
                name: "Repeat Interval".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager
                        .settings
                        .repeat_interval
                        .as_millis()
                        .to_string()
                }),
                action: InteractionType::Scrollable(
                    &|menu_manager: &mut MenuManager, increase: bool| {
                        if increase {
                            menu_manager.settings.repeat_interval = menu_manager
                                .settings
                                .repeat_interval
                                .checked_add(Duration::from_millis(1))
                                .unwrap();
                        } else {
                            menu_manager.settings.repeat_interval = menu_manager
                                .settings
                                .repeat_interval
                                .checked_sub(Duration::from_millis(1))
                                .unwrap();
                        }
                    },
                ),
            },
            MenuOption::Action {
                name: "Soft Drop Interval".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager.settings.fall_interval.as_millis().to_string()
                }),
                action: InteractionType::Scrollable(
                    &|menu_manager: &mut MenuManager, increase: bool| {
                        if increase {
                            menu_manager.settings.fall_interval = menu_manager
                                .settings
                                .fall_interval
                                .checked_add(Duration::from_millis(1))
                                .unwrap();
                        } else {
                            menu_manager.settings.fall_interval = menu_manager
                                .settings
                                .fall_interval
                                .checked_sub(Duration::from_millis(1))
                                .unwrap();
                        }
                    },
                ),
            },
            MenuOption::Back {
                name: "Back to Main Menu".to_string(),
            },
        ],
        parent: Some(0),
    };

    // menu for keybindings

    let controls_menu = MenuNode {
        title: "Options".to_string(),
        options: vec![
            MenuOption::Action {
                name: "Move Left".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager
                        .settings
                        .key_bindings
                        .move_left
                        .name()
                        .to_string()
                }),
                action: InteractionType::Scancode("move_left"),
            },
            MenuOption::Action {
                name: "Move Right".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager
                        .settings
                        .key_bindings
                        .move_right
                        .name()
                        .to_string()
                }),
                action: InteractionType::Scancode("move_right"),
            },
            MenuOption::Action {
                name: "Rotate Clockwise".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager
                        .settings
                        .key_bindings
                        .rotate_clockwise
                        .name()
                        .to_string()
                }),
                action: InteractionType::Scancode("rotate_clockwise"),
            },
            MenuOption::Action {
                name: "Rotate Counter Clockwise".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager
                        .settings
                        .key_bindings
                        .rotate_counter_clockwise
                        .name()
                        .to_string()
                }),
                action: InteractionType::Scancode("rotate_counter_clockwise"),
            },
            MenuOption::Action {
                name: "Rotate 180".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager
                        .settings
                        .key_bindings
                        .rotate_180
                        .name()
                        .to_string()
                }),
                action: InteractionType::Scancode("rotate_180"),
            },
            MenuOption::Action {
                name: "Hard Drop".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager
                        .settings
                        .key_bindings
                        .hard_drop
                        .name()
                        .to_string()
                }),
                action: InteractionType::Scancode("hard_drop"),
            },
            MenuOption::Action {
                name: "Soft Drop".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager
                        .settings
                        .key_bindings
                        .soft_drop
                        .name()
                        .to_string()
                }),
                action: InteractionType::Scancode("soft_drop"),
            },
            MenuOption::Action {
                name: "Hold".to_string(),
                dynamic_value: Some(&|menu_manager| {
                    menu_manager.settings.key_bindings.hold.name().to_string()
                }),
                action: InteractionType::Scancode("hold"),
            },
            MenuOption::Back {
                name: "Back to Main Menu".to_string(),
            },
        ],
        parent: Some(0),
    };

    let menus = vec![main_menu, options_menu, controls_menu];

    let mut menu_manager: MenuManager =
        MenuManager::new(&sdl.0, &sdl.1, &mut sdl.2, &mut sdl.3, menus)?;

    menu_manager.run();

    Ok(())
}

fn init_sdl() -> Result<
    (
        sdl2::Sdl,
        sdl2::ttf::Sdl2TtfContext,
        sdl2::render::Canvas<sdl2::video::Window>,
        sdl2::EventPump,
    ),
    String,
> {
    const WINDOW_WIDTH: u32 = 1000;
    const WINDOW_HEIGHT: u32 = 800;

    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window("rusty-tetris", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    window
        .set_minimum_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let event_pump = sdl_context.event_pump()?;

    Ok((sdl_context, ttf_context, canvas, event_pump))
}

fn init_theme(light_mode: bool) -> Theme {
    let (bg_color_1, bg_color_2, text_color) = if light_mode {
        (
            Color::RGBA(245, 245, 245, 255),
            Color::RGBA(255, 255, 255, 255),
            Color::RGBA(0, 0, 0, 255),
        )
    } else {
        (
            Color::RGBA(10, 10, 10, 255),
            Color::RGBA(0, 0, 0, 255),
            Color::RGBA(255, 255, 255, 255),
        )
    };
    Theme {
        bg_color_1,
        bg_color_2,
        text_color,
    }
}
