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
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut game: Game = Game::new(
        &sdl_context,
        &ttf_context,
        true,
        repeat_delay,
        repeat_interval,
        fall_interval,
        Gamemode::Lines40,
    )?;

    game.run();

    Ok(())
}
