use std::io::{stdin, stdout, Write};

pub struct Logic {
    pub board: Vec<char>,
    pub turn: char,
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
        let line = "-".repeat(13);
        println!("{}", line);
        let mut n = 0;
        // index value
        let mut i = 0;
        for c in &self.board {
            if n == 0 {
                print!("| ")
            }
            if n <= 2 && n > 0 {
                print!(" | ")
            }
            if c == &' ' {
                print!("{}", i);
            } else {
                print!("{}", c);
            }
            if n == 2 {
                print!(" |");
                println!("\n{}", line);
                n = 0;
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
