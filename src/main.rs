mod game;
mod tetrominos;
mod utilities;

use game::Game;
use std::time::Duration;
use utilities::Gamemode;

fn main() -> Result<(), String> {
    let repeat_delay: Duration = Duration::from_millis(100);
    let repeat_interval: Duration = Duration::from_millis(20);
    let fall_interval: Duration = Duration::from_millis(20);
    let mut sdl = init_sdl()?;

    let mut game: Game = Game::new(
        &sdl.0,
        &sdl.1,
        &mut sdl.2,
        &mut sdl.3,
        true,
        repeat_delay,
        repeat_interval,
        fall_interval,
        Gamemode::Lines40,
    )?;

    game.run();

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
