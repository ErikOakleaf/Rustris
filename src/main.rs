mod game;
mod menu;
mod tetrominos;
mod utilities;

use game::Game;
use menu::{Menu, MenuManager, MenuNode, MenuOption};
use sdl2::pixels::Color;
use std::time::Duration;
use utilities::{Gamemode, Theme};

fn main() -> Result<(), String> {
    let repeat_delay: Duration = Duration::from_millis(100);
    let repeat_interval: Duration = Duration::from_millis(20);
    let fall_interval: Duration = Duration::from_millis(20);
    let mut sdl = init_sdl()?;
    let theme = init_theme(false);

    let game_action = |menu_manager: &mut MenuManager| {
        let repeat_delay = Duration::from_millis(100);
        let repeat_interval = Duration::from_millis(20);
        let fall_interval = Duration::from_millis(20);

        let game = Game::new(
            &menu_manager.sdl_context,
            &menu_manager.ttf_context,
            &mut menu_manager.canvas,
            &mut menu_manager.event_pump,
            &menu_manager.theme,
            repeat_delay,
            repeat_interval,
            fall_interval,
            Gamemode::Lines40,
        );

        match game {
            Ok(mut g) => g.run(),
            Err(e) => println!("Failed to start game: {}", e),
        }
    };
    let main_menu = MenuNode {
        title: "Main Menu".to_string(),
        options: vec![
            MenuOption::Action {
                name: "Classic".to_string(),
                action: &game_action,
            },
            MenuOption::Submenu {
                name: "Options".to_string(),
                submenu_index: 1,
            },
        ],
        parent: None,
    };

    let options_menu = MenuNode {
        title: "Options".to_string(),
        options: vec![
            MenuOption::Action {
                name: "Change Keybindings".to_string(),
                action: &|_menu_manager: &mut MenuManager| println!("Keybinding configuration coming soon!"),
            },
            MenuOption::Back {
                name: "Back to Main Menu".to_string(),
            },
        ],
        parent: Some(0),
    };

    let menus = vec![main_menu, options_menu];

    let mut menu_manager: MenuManager =
        MenuManager::new(&sdl.0, &sdl.1, &mut sdl.2, &mut sdl.3, &theme, menus)?;

    menu_manager.run();

    //// init menu here
    //let options = vec![
    //    "Classic".to_string(),
    //    "40 Lines".to_string(),
    //    "Options".to_string(),
    //];
    //let mut menu = Menu::new(&sdl.0, &sdl.1, &mut sdl.2, &mut sdl.3, &theme, options)?;
    //
    //menu.run();

    //let mut game: Game = Game::new(
    //    &sdl.0,
    //    &sdl.1,
    //    &mut sdl.2,
    //    &mut sdl.3,
    //    &theme,
    //    repeat_delay,
    //    repeat_interval,
    //    fall_interval,
    //    Gamemode::Lines40,
    //)?;
    //
    //game.run();

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

fn start_classic() {}
