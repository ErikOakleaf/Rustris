use game::Game;

mod tetrominos;
mod game;

fn main() -> Result<(), String> {
    
    let mut game: Game = Game::new()?;

    game.run();

    Ok(())
}
