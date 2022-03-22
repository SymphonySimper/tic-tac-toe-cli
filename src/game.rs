use crate::Logic;

pub struct Game {
    logic: Logic,
    round: i8,
}
impl Game {
    pub fn new() -> Self {
        Self {
            logic: Logic::new(),
            round: 1,
        }
    }

    pub fn run(&mut self) {
        let ref mut logic = self.logic;
        loop {
            if self.round > 9 {
                println!("It's a tie");
                break;
            }

            // Clear screen
            println!("\x1Bc");

            logic.display();
            let m = logic.get_input();
            if let Some(i) = m {
                logic.update(i);
            } else {
                println!("Enter a valid choice![0-8]");
                continue;
            }
            if logic.is_done() {
                println!("Payer {} won!", logic.turn);
                break;
            } else {
                logic.change_turn();
                self.round += 1;
            }
        }
    }
}
