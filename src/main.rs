// TO DO
// -implement AI
// -use termion or an alt
// -modify Game.end() to look better
// -implement q to quit
// -split this into game and board module

mod playersymbol;
use playersymbol::PlayerSymbol;

struct Board {
    r1: [PlayerSymbol; 3],
    r2: [PlayerSymbol; 3],
    r3: [PlayerSymbol; 3],
}
impl Board {
    // To reset the board for a new game
    fn reset(&mut self) {
        self.r1 = [PlayerSymbol::N; 3];
        self.r2 = [PlayerSymbol::N; 3];
        self.r3 = [PlayerSymbol::N; 3];
    }

    // To check if player has won
    fn check_win(&self) -> PlayerSymbol {
        // row check
        if self.r1[0] == self.r1[1] && self.r1[1] == self.r1[2] && self.r1[0] != PlayerSymbol::N {
            return self.r1[0];
        }
        if self.r2[0] == self.r2[1] && self.r2[1] == self.r2[2] && self.r2[0] != PlayerSymbol::N {
            return self.r2[0];
        }
        if self.r3[0] == self.r3[1] && self.r3[1] == self.r3[2] && self.r3[0] != PlayerSymbol::N {
            return self.r3[0];
        }
        // Column check
        if self.r1[0] == self.r2[0] && self.r2[0] == self.r3[0] && self.r3[0] != PlayerSymbol::N {
            return self.r1[0];
        }
        if self.r1[1] == self.r2[1] && self.r2[1] == self.r3[1] && self.r3[1] != PlayerSymbol::N {
            return self.r1[1];
        }
        if self.r1[2] == self.r2[2] && self.r2[2] == self.r3[2] && self.r3[2] != PlayerSymbol::N {
            return self.r1[2];
        }
        // Diagonal check
        if self.r1[0] == self.r2[1] && self.r2[1] == self.r3[2] && self.r3[2] != PlayerSymbol::N {
            return self.r1[0];
        }
        if self.r1[2] == self.r2[1] && self.r2[1] == self.r3[0] && self.r3[0] != PlayerSymbol::N {
            return self.r1[2];
        }
        // No win
        return PlayerSymbol::N;
    }

    // To check if its a draw
    fn check_draw(&self) -> bool {
        fn chance_of_winning(a: PlayerSymbol, b: PlayerSymbol, c: PlayerSymbol) -> bool {
            if a == PlayerSymbol::X {
                if b == PlayerSymbol::X {
                    if c == PlayerSymbol::O {
                        return false;
                    } else {
                        return true;
                    }
                } else if b == PlayerSymbol::O {
                    return false;
                } else {
                    if c == PlayerSymbol::X {
                        return true;
                    } else if c == PlayerSymbol::O {
                        return false;
                    } else {
                        return true;
                    }
                }
            } else if a == PlayerSymbol::O {
                if b == PlayerSymbol::X {
                    return false;
                } else if b == PlayerSymbol::O {
                    if c == PlayerSymbol::X {
                        return false;
                    } else {
                        return true;
                    }
                } else {
                    if c == PlayerSymbol::X {
                        return false;
                    } else if c == PlayerSymbol::O {
                        return true;
                    } else {
                        return true;
                    }
                }
            } else {
                if b == PlayerSymbol::X {
                    if c == PlayerSymbol::X {
                        return true;
                    } else if c == PlayerSymbol::O {
                        return false;
                    } else {
                        return true;
                    }
                } else if b == PlayerSymbol::O {
                    if c == PlayerSymbol::X {
                        return false;
                    } else if c == PlayerSymbol::O {
                        return true;
                    } else {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
        if chance_of_winning(self.r1[0], self.r1[1], self.r1[2]) ||
           chance_of_winning(self.r2[0], self.r2[1], self.r2[2]) ||
           chance_of_winning(self.r3[0], self.r3[1], self.r3[2]) ||
           chance_of_winning(self.r1[0], self.r2[0], self.r3[0]) ||
           chance_of_winning(self.r1[1], self.r2[1], self.r3[1]) ||
           chance_of_winning(self.r1[2], self.r2[2], self.r3[2]) ||
           chance_of_winning(self.r1[0], self.r2[1], self.r3[2]) ||
           chance_of_winning(self.r1[2], self.r2[1], self.r3[0]) {
            return false;
        }
        true
    }

    // To change value of a board cell
    fn mark(&mut self, r: usize, c: usize, symbol: PlayerSymbol) -> bool {
        match r {
            1 => {
                if self.r1[c] == PlayerSymbol::N {
                    self.r1[c] = symbol
                } else {
                    return false;
                }
            }
            2 => {
                if self.r2[c] == PlayerSymbol::N {
                    self.r2[c] = symbol
                } else {
                    return false;
                }
            }
            3 => {
                if self.r3[c] == PlayerSymbol::N {
                    self.r3[c] = symbol
                } else {
                    return false;
                }
            }
            _ => return false,
        };
        return true;

    }

    fn display(&self, xwin: u32, owin: u32) {
        println!(" {} | {} | {} ", self.r1[0], self.r1[1], self.r1[2]);
        println!("---*---*---\tx win : {}", xwin);
        println!(" {} | {} | {} ", self.r2[0], self.r2[1], self.r2[2]);
        println!("---*---*---\to win : {}", owin);
        println!(" {} | {} | {} ", self.r3[0], self.r3[1], self.r3[2]);

    }
}

extern crate termion;
use termion::{clear, cursor, color};

struct Game {
    board: Board,
    xwin: u32,
    owin: u32,
    currentsymbol: PlayerSymbol,
}
impl Game {
    fn new() -> Game {
        let newboard = Board {
            r1: [PlayerSymbol::N; 3],
            r2: [PlayerSymbol::N; 3],
            r3: [PlayerSymbol::N; 3],
        };
        let newgame = Game {
            board: newboard,
            xwin: 0,
            owin: 0,
            currentsymbol: PlayerSymbol::X,
        };
        newgame
    }

    fn start(&mut self) {
        self.board.reset();
        while !self.game_over() {
            self.output();
            if self.currentsymbol == PlayerSymbol::X {
                self.player_move(PlayerSymbol::X);
                self.currentsymbol = PlayerSymbol::O;
            } else {
                self.player_move(PlayerSymbol::O);
                self.currentsymbol = PlayerSymbol::X;
            }

        }

    }

    fn play_again(&self) -> bool {
        println!("Do you want to play again ? (y/n)");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Cannot read input");
        if choice.as_bytes()[0] == 78 || choice.as_bytes()[0] == 110 {
            return false;
        }
        return true;
    }

    fn game_over(&mut self) -> bool {
        if self.board.check_win() == PlayerSymbol::X {
            self.xwin += 1;
            println!("X wins the game");
            return true;
        } else if self.board.check_win() == PlayerSymbol::O {
            self.owin += 1;
            println!("O wins the game");
            return true;
        } else {
            if self.board.check_draw() == true {
                println!("It's a draw ");
                return true;
            }
        }
        return false;
    }

    fn output(&self) {
        println!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("Tic Tac Toe");
        println!("{}", color::Fg(color::Rgb(0, 255, 0)));
        self.board.display(self.xwin, self.owin);
        println!("{}", color::Fg(color::Reset));

    }

    fn player_move(&mut self, ps: PlayerSymbol) {
        println!("{}'s turn", ps);
        loop {
            let r = self.get_val("row");
            let c = self.get_val("column") - 1;
            let is_success = self.board.mark(r, c, ps);
            if is_success == true {
                break;
            } else {
                println!("Already occupied");
            }
        }
    }

    fn get_val(&self, valof: &str) -> usize {
        println!("Enter {} number", valof);
        loop {
            let mut v = String::new();
            std::io::stdin().read_line(&mut v).expect("Unable to read input");
            let v: usize = match v.trim().parse::<usize>() {
                Ok(num @ 1...3) => num,
                Ok(_) => {
                    println!("Valid {} numbers :1,2,3 ", valof);
                    continue;
                }
                Err(_) => {
                    println!("Enter a valid number");
                    continue;
                }
            };
            return v;
        }
    }

    fn end(&self) {
        println!("{}X win : {}\nO win : {}", clear::All, self.xwin, self.owin);
    }
}

fn main() {
    let mut game = Game::new();
    loop {
        game.start();
        if game.play_again() == false {
            break;
        }
    }
    game.end();

}
