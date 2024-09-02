#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, literal: String) -> Self {
        Token { token_type, literal }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenType {
    // Special tokens
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
