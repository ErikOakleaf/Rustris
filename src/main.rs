mod game;
mod tetrominos;
mod utilities;

use game::Game;
use std::time::Duration;

fn main() -> Result<(), String> {
    let repeat_delay: Duration = Duration::from_millis(100);
    let repeat_interval: Duration = Duration::from_millis(20);
    let fall_interval: Duration = Duration::from_millis(20);
    let sdl_context = sdl2::init()?;

    let mut game: Game = Game::new(&sdl_context, true, repeat_delay, repeat_interval, fall_interval)?;

    game.run();

    Ok(())
}
