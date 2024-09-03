use lazy_static::lazy_static;
use std::collections::HashMap;

// lazy_static! {
//     static ref KEYWORDS: HashMap<&'static str, TokenType> = {
//         let mut map = HashMap::new();
//         map.insert("fn", TokenType::FUNCTION);
//         map.insert("let", TokenType::LET);
//         map.insert("true", TokenType::TRUE);
//         map.insert("false", TokenType::FALSE);
//         map.insert("if", TokenType::IF);
//         map.insert("else", TokenType::ELSE);
//         map.insert("return", TokenType::RETURN);
//         map
//     };
// }

#[derive(Clone, Default, Debug, PartialEq)]
pub(crate) enum Token {
    // Special tokens
    Illegal(String),
    #[default]
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Eq,
    NotEq,
    LessThan,
    GreaterThan,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = match self {
            Token::Illegal(_str) => "ILLEGAL",
            Token::Eof => "EOF",
            Token::Ident(str) => return write!(f, "{str}"),
            Token::Int(nb) => return write!(f, "{nb}"),
            Token::Assign => "ASSIGN",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Bang => "!",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Eq => "==",
            Token::NotEq => "!=",
            Token::LessThan => "<",
            Token::GreaterThan => ">",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Function => "FUNCTION",
            Token::Let => "LET",
            Token::True => "TRUE",
            Token::False => "FALSE",
            Token::If => "IF",
            Token::Else => "ELSE",
            Token::Return => "RETURN",
        };
        write!(f, "{}", token)
    }
}
