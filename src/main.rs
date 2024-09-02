use repl::Repl;

mod lexer;
mod token;
mod repl;

fn main() {
    Repl::default().start();
}
