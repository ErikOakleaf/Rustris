mod game;
mod tetrominos;

use game::Game;

fn main() -> Result<(), String> {
    
    let mut game: Game = Game::new()?;

    game.run();

    Ok(())
}
