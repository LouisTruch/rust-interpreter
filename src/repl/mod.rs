use crate::{evaluation::Eval, lexer::Lexer, parser::Parser, token::Token, Environment};
use std::{
    io::{self, Write},
    rc::Rc,
    str::FromStr,
};

#[derive(Default)]
pub struct Repl {
    mode: ReplMode,
    environment: Option<Rc<Environment>>,
}

impl Repl {
    pub fn start(&mut self) {
        self.swap_mode(self.mode.clone());

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
                ReplMode::Eval => self.eval_input(input),
            }
        }
    }

    fn print_current_mode(&self) {
        println!("Mode {}", self.mode);
        io::stdout().flush().expect("Failed to flush stdout");
    }

    fn swap_mode(&mut self, mode: ReplMode) {
        self.mode = mode.clone();
        if mode == ReplMode::Eval {
            self.environment = Some(Environment::new_rc());
        } else {
            self.environment = None;
        }
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

        println!("Errors {:?}", parser.errors);

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

    fn eval_input(&self, input: String) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let Ok(program) = parser.parse_program() else {
            eprintln!("Error parsing program");
            return;
        };

        if parser.errors.len() > 0 {
            eprintln!("Error parsing program");
            return;
        }

        match program.eval(self.environment.clone().unwrap()) {
            Ok(object) => println!("{}", object),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub(crate) enum ReplMode {
    Lexing,
    Parsing,
    #[default]
    Eval,
}

impl std::fmt::Display for ReplMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReplMode::Lexing => "Lexing",
                ReplMode::Parsing => "Parsing",
                ReplMode::Eval => "Eval",
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
            "eval" => Ok(Self::Eval),
            _ => Err(()),
        }
    }
}
