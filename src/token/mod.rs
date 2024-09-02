use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Default)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("fn", TokenType::FUNCTION);
        map.insert("let", TokenType::LET);
        map
    };
}

impl Token {
    pub(crate) fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum TokenType {
    // Special tokens
    #[default]
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

impl TokenType {
    // Could be from::str ?
    pub(crate) fn lookup_ident(ident: &str) -> TokenType {
        match KEYWORDS.get(ident) {
            Some(token) => token.clone(),
            None => TokenType::IDENT,
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = match self {
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::EOF => "EOF",
            TokenType::IDENT => "IDENT",
            TokenType::INT => "INT",
            TokenType::ASSIGN => "ASSIGN",
            TokenType::PLUS => "PLUS",
            TokenType::COMMA => "COMMA",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::LPAREN => "(",
            TokenType::RPAREN => ")",
            TokenType::LBRACE => "{",
            TokenType::RBRACE => "}",
            TokenType::FUNCTION => "FUNCTION",
            TokenType::LET => "LET",
        };
        write!(f, "{}", token)
    }
}
