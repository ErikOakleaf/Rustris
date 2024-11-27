mod game;
mod tetrominos;
mod utilities;

use std::time::Duration; 
use game::Game;

fn main() -> Result<(), String> {

    let repeat_delay: Duration = Duration::from_millis(100);
    let repeat_interval: Duration = Duration::from_millis(20);
    let fall_interval: Duration = Duration::from_millis(20);
    
    let mut game: Game = Game::new(true, repeat_delay, repeat_interval, fall_interval)?;

    game.run();

    Ok(())
}
