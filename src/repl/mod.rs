use std::io::{self, Write};

use crate::lexer::Lexer;

#[derive(Default)]
pub(crate) struct Repl();

impl Repl {
    pub(crate) fn start(&self) {
        loop {
            print!(">> ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim() == "exit" {
                break;
            }

            print!("You entered: {}", input);

            let mut l = Lexer::new(input);

            loop {
                let tok = l.next_token();
                if tok.token_type == crate::token::TokenType::EOF {
                    break;
                }
                println!("{:?}", tok);
            }
        }
    }
}
