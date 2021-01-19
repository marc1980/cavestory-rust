mod modules;
use modules::Game;
pub fn main() -> Result<(), String> {
    let mut game = Game::new();
    game.game_loop();

    Ok(())
}
