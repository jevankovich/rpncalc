use std::io;
use std::io::prelude::*;
use std::ops::Index;
use std::collections::HashMap;
use std::rc::Rc;

struct Calc {
    snowman_powers: bool,
    stack: Vec<f64>,
}

impl Index<usize> for Calc {
    type Output = f64;
    fn index<'a>(&'a self, index: usize) -> &'a f64 {
        &self.stack[self.stack.len() - 1 - index]
    }
}

impl Calc {
    fn new() -> Self {
        let ret = Calc { 
            snowman_powers: false,
            stack: Vec::new(),
        };

        return ret;
    }

    fn interpret(&mut self, input: &str) {
        if self.snowman_powers {
            println!("☃☃☃ Snowman party! ☃☃☃")
        }

        for word in input.split_whitespace() {
            self.interpret_word(word);
        }

        println!("{:?}", self.stack);
    }

    fn unknown(&mut self, word: &str) {
        let val = word.parse::<f64>();
        match val {
            Ok(val) => self.stack.push(val),
            Err(err) => println!("Unknown word: \"{}\"", word),
        }
    }

    fn interpret_word(&mut self, word: &str) {
        match word {
            "exit" | "quit" | "q" => std::process::exit(0),
            "+" => {
                if self.stack.len() < 2 {
                    println!("Not enough arguments!");
                    return;
                }
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a + b);
            },
            // Super secret snowman sauce
            "☃" => { 
                self.snowman_powers = !self.snowman_powers;
                println!("Enter \"☃\" again to disable snowman mode");
            },
            _ => self.unknown(word)
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut calc = Calc::new();

    print!("> ");
    stdout.flush();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => calc.interpret(&line),
            Err(err) => panic!("{:?}", err)
        }

        print!("> ");
        stdout.flush();
    }

}
