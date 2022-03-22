use crate::terminal;

pub struct Logic {
    pub board: Vec<char>,
    pub turn: char,
    wc: Vec<usize>,
}

fn print_border(a: &[&str], cb: &[&str]) {
    let t = format!(
        "{}{}{}{}",
        a[0],
        (cb[0].repeat(3) + a[1]).repeat(2),
        cb[0].repeat(3),
        a[2],
    );
    println!("{}", terminal::border(&t));
}

impl Logic {
    pub fn new() -> Self {
        Self {
            board: vec![' '; 9],
            turn: 'X',
            wc: vec![0; 3],
        }
    }
    pub fn get_input(&self, n: &String) -> Option<usize> {
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

        let mut n = 0;
        // index value
        let mut i: usize = 0;

        print_border(&b[0], &cb);
        for c in &self.board {
            if n == 0 {
                print!("{} ", terminal::border(cb[1]));
            }
            if n <= 2 && n > 0 {
                print!(" {} ", terminal::border(cb[1]));
            }
            if c == &' ' && self.wc == [0, 0, 0] {
                print!("{}", terminal::num(&i));
            } else {
                if self.wc != [0, 0, 0] {
                    if self.wc[0] == i || self.wc[1] == i || self.wc[2] == i {
                        print!("{}", terminal::turn(&c));
                    } else {
                        print!("{}", terminal::nimp(&c));
                    }
                } else {
                    print!("{}", terminal::turn(&c));
                }
            }
            if n == 2 {
                print!(" {}", terminal::border(cb[1]));
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

    pub fn update(&mut self, n: usize) -> bool {
        if self.board[n] == ' ' {
            self.board[n] = self.turn;
            true
        } else {
            false
        }
    }

    pub fn change_turn(&mut self) {
        if self.turn == 'X' {
            self.turn = 'O'
        } else {
            self.turn = 'X'
        }
    }

    pub fn is_done(&mut self) -> &str {
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

        let mut win = "nah";
        for &(x, y, z) in win_combi.iter() {
            if self.board[x] == self.board[y]
                && self.board[y] == self.board[z]
                && self.board[z] == self.turn
            {
                self.wc = vec![x, y, z];
                win = "done"
            }
            if win != "done" {
                for val in ['X', 'O'] {
                    if (self.board[x] == val
                        && self.board[y] == val
                        && self.board[z] != val
                        && self.board[z] != ' ')
                        || (self.board[x] == val
                            && self.board[y] != val
                            && self.board[y] != ' '
                            && self.board[z] == val)
                        || (self.board[x] != val
                            && self.board[x] != ' '
                            && self.board[y] == val
                            && self.board[z] == val)
                    {
                        win = "tie";
                    } else {
                        win = "nah";
                    }
                }
            }
        }

        win
    }
}
