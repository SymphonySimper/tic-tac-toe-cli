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
        let mut false_input = false;
        let ref mut logic = self.logic;
        loop {
            if self.round > 9 {
                println!("It's a tie");
                break;
            }

            if !false_input {
                logic.display();
            }

            let m = logic.get_input();
            if let Some(i) = m {
                false_input = false;
                logic.update(i);
            } else {
                println!("Enter a valid choice![0-8]");
                false_input = true;
                continue;
            }
            match logic.is_done() {
                "done" => {
                    logic.display();
                    println!("Payer {} won!", logic.turn);
                    break;
                }
                "nah" => {
                    logic.change_turn();
                    self.round += 1;
                }

                "tie" => {
                    logic.display();
                    println!("It's a tie.");
                    break;
                }

                _ => (),
            }
        }
    }
}
