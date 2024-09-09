#[allow(dead_code)]
mod ast;

#[allow(dead_code)]
mod lexer;

#[allow(dead_code)]
mod parser;

#[allow(dead_code)]
mod object;

#[allow(dead_code)]
mod evaluation;
pub use evaluation::environment::Environment;

mod repl;
pub use repl::Repl;

mod token;
