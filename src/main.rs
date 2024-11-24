mod game;
mod tetrominos;
mod utilities;

use game::Game;

fn main() -> Result<(), String> {
    
    let mut game: Game = Game::new()?;

    game.run();

    Ok(())
}
