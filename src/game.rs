use crate::Logic;
use std::io::{stdin, stdout, Write};
use crate::terminal;

pub struct Game {
    logic: Logic,
    round: i8,
}

fn read() -> String {
    let mut input = String::new();
    stdout().flush().expect("couldn't flush to stdout");
    stdin()
        .read_line(&mut input)
        .expect("Couldn't read from stdin");
    String::from(input.trim())
}

fn new() -> Game {
    Game {
        logic: Logic::new(),
        round: 1,
    }
}

fn playagain() {
    print!("Would you like to play again[Y/n]? ");
    let ans = read();
    match ans.as_str() {
        "Y" | "y" => run(),
        _ => std::process::exit(0),
    }
}
fn help() {
    println!("h - help\nq - quit\nr - restart");
}
pub fn run() {
    let mut game = new();
    let mut false_input = false;
    let ref mut logic = game.logic;
    loop {
        if game.round > 9 {
            println!("It's a tie");
            break;
        }

        if !false_input {
            // clear screen
            terminal::clear_sreen();
            logic.display();
        }

        print!("Enter a tile no. player {}: ", terminal::turn(&logic.turn));
        let user_input = read();
        let m = logic.get_input(&user_input);
        if let Some(i) = m {
            false_input = false;
            if !logic.update(i) {
                println!("Choose an empty tile!");
            }
        } else {
            match user_input.as_str() {
                "h" => help(),
                "q" => std::process::exit(0),
                "r" => run(),
                _ => println!("Enter a valid tile![0-8]"),
            }
                false_input = true;
                continue;
        }
        match logic.is_done() {
            "done" => {
                terminal::clear_sreen();
                logic.display();
                println!("Payer {} won!", logic.turn);
                playagain()
            }
            "nah" => {
                logic.change_turn();
                game.round += 1;
            }

            "tie" => {
                terminal::clear_sreen();
                logic.display();
                println!("It's a tie.");
                playagain()
            }

            _ => (),
        }
    }
}
