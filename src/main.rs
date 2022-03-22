mod logic;
mod game;
mod terminal;
pub use logic::Logic;

fn main() {
    game::run()
}
