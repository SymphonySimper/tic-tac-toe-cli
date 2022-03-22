mod logic;
mod game;
pub use logic::Logic;
use game::Game;

fn main() {
    Game::new().run()
}
