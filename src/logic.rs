use std::io::{stdin, stdout, Write};

pub struct Logic {
    pub board: Vec<char>,
    pub turn: char,
}

fn print_border(a: &[&str], cb: &[&str]) {
    println!(
        "{}{}{}{}",
        a[0],
        (cb[0].repeat(3) + a[1]).repeat(2),
        cb[0].repeat(3),
        a[2]
    );
}

impl Logic {
    pub fn new() -> Self {
        Self {
            board: vec![' '; 9],
            turn: 'X',
        }
    }
    pub fn get_input(&self) -> Option<usize> {
        let mut n = String::new();
        println!("It's player {}'s turn: ", self.turn);
        print!("Enter your choice [0-8]: ");
        stdout().flush().expect("Couldn't flush to stdout");
        stdin().read_line(&mut n).expect("Could't read from stdin");
        let n = n.trim().parse::<usize>();
        if let Ok(i) = n {
            if i > 8 {
                None
            } else {
                Some(i)
            }
        } else {
            None
        }
    }

    pub fn display(&self) {
        let mut count = 0;
        let cb = ["─", "│"];
        let b = [["┌", "┬", "┐"], ["├", "┼", "┤"], ["└", "┴", "┘"]];

        print_border(&b[0], &cb);
        let mut n = 0;
        // index value
        let mut i = 0;
        for c in &self.board {
            if n == 0 {
                print!("{} ", cb[1]);
            }
            if n <= 2 && n > 0 {
                print!(" {} ", cb[1]);
            }
            if c == &' ' {
                print!("{}", i);
            } else {
                // \x1b[1;3m bold italic text
                // \x1b[0m reset
                print!("\x1b[1;3m{}\x1b[0m", c);
            }
            if n == 2 {
                print!(" {}", cb[1]);
                println!();
                if count == 0 || count == 1 {
                    print_border(&b[1], &cb);
                } else {
                    print_border(&b[2], &cb);
                }
                n = 0;
                count += 1;
            } else {
                n += 1
            }
            i += 1;
        }
        println!();
    }

    pub fn update(&mut self, n: usize) {
        if self.board[n] == ' ' {
            self.board[n] = self.turn;
        }
    }

    pub fn change_turn(&mut self) {
        if self.turn == 'X' {
            self.turn = 'O'
        } else {
            self.turn = 'X'
        }
    }

    pub fn is_done(&self) -> bool {
        let win_combi = vec![
            // horizontal
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            // vertical
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            // plus
            (1, 4, 7),
            (3, 4, 5),
            // cross
            (0, 4, 8),
            (2, 4, 6),
        ];

        let mut win = false;
        for &(x, y, z) in win_combi.iter() {
            if self.board[x] == self.board[y]
                && self.board[y] == self.board[z]
                && self.board[z] == self.turn
            {
                win = true
            }
        }

        win
    }
}
