use crate::{lexer::Lexer, parser::Parser, token::Token};
use std::{
    io::{self, Write},
    str::FromStr,
};

#[derive(Default)]
pub struct Repl {
    mode: ReplMode,
}

impl Repl {
    pub fn start(&mut self) {
        self.print_current_mode();
        loop {
            print!(">> ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if let Ok(mode) = ReplMode::from_str(&input) {
                self.swap_mode(mode);
                continue;
            }

            match input.trim().to_lowercase().as_ref() {
                "exit" => break,
                _ => (),
            }

            match self.mode {
                ReplMode::Lexing => self.lex_input(input),
                ReplMode::Parsing => self.parse_input(input),
            }
        }
    }

    fn print_current_mode(&self) {
        println!("Mode {}", self.mode);
        io::stdout().flush().expect("Failed to flush stdout");
    }

    fn swap_mode(&mut self, mode: ReplMode) {
        self.mode = mode;
        self.print_current_mode();
    }

    fn lex_input(&self, input: String) {
        let mut l = Lexer::new(input);
        loop {
            let tok = l.next_token();
            if tok == Token::Eof {
                break;
            }
            println!("{:?}", tok);
        }
    }

    fn parse_input(&self, input: String) {
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        let program = match program {
            Ok(program) => program,
            Err(e) => {
                eprintln!("Error parsing program: {:?}", e);
                return;
            }
        };

        println!("Program: {}", program);
    }
}

#[derive(Default)]
pub(crate) enum ReplMode {
    Lexing,
    #[default]
    Parsing,
}

impl std::fmt::Display for ReplMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReplMode::Lexing => "Lexing",
                ReplMode::Parsing => "Parsing",
            }
        )
    }
}

impl FromStr for ReplMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "lexing" => Ok(Self::Lexing),
            "parsing" => Ok(Self::Parsing),
            _ => Err(()),
        }
    }
}
