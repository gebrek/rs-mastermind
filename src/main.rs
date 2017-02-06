extern crate rand;
use rand::Rng;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
extern crate bytecount;

const DICT_FILE_LENGTH: usize = 45403;

struct Game {
    code: String,
    guesses: i32,
    running: bool,
}

impl Game {
    fn guess(&mut self, s: String) {
        self.guesses += 1;
        if s.eq(&self.code) {
            self.win()
        } else {
            self.hint(s)
        }
    }

    fn win(&mut self) {
        self.running = false;
        println!("You made {} guesses.", &self.guesses);
        println!("You win!")
    }

    fn hint(&self, s: String) {
        let mut s_chs = s.chars();
        let c_chs = self.code.chars();
        let mut hint = String::from("");
        for sc in c_chs {
            match s_chs.next() {
                Some(cc) => {
                    if cc == sc {
                        hint.push(cc);
                    } else {
                        hint.push('*');
                    }
                }
                None => hint.push('*'),
            }
        }
        println!("{}", hint)
    }
}

impl Default for Game {
    fn default() -> Game {
        Game {
            code: random_word(),
            guesses: 0,
            running: true,
        }
    }
}

fn new_game() -> Game {
    Game { ..Default::default() }
}

fn random_word() -> String {
    let fh = match File::open("linuxwords") {
        Err(e) => panic!("couldn't open linuxwords: {}", e),
        Ok(fh) => fh,
    };
    let f = BufReader::new(fh);
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0, DICT_FILE_LENGTH);
    let mut chosen = "".to_string();
    for (i, line) in f.lines().enumerate() {
        if n == i {
            match line {
                Ok(l) => chosen = l,
                Err(e) => panic!("Error reading linuxwords: {}", e),
            }
            break;
        }
    }
    // println!("{}", chosen.to_lowercase());
    chosen.to_lowercase()
}

fn main() {
    let mut game = new_game();

    while game.running {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap(); // ._.
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                game.guess(input.trim().to_string());
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
