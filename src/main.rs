mod game;
use game::Game;

fn main() {
    let mut game = Game::new();
    loop {
        game.display();
        let m = game.get_input();
        if let Some(i) = m {
            game.update(i);
        } else {
            println!("Enter a valid choice![0-8]");
            continue;
        }
        if game.is_done() {
            println!("Payer {} won!", game.turn);
            break;
        } else {
            game.change_turn()
        }
    }
}
