use crate::token::*;

pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_pos: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            // This is safe because we know that read_pos is less than input.len()
            self.ch = self.input.chars().nth(self.read_pos).unwrap();
        }
        self.position = self.read_pos;
        self.read_pos += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => Token::new(TokenType::ASSIGN, self.ch.to_string()),
            ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
            ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
            '+' => Token::new(TokenType::PLUS, self.ch.to_string()),
            '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),
            '\0' => Token::new(TokenType::EOF, self.ch.to_string()),
            _ => {
                if self.is_valid_identifier_char(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = TokenType::lookup_ident(&literal);
                    return Token::new(token_type, literal);
                } else if self.ch.is_digit(10)  {
                    let literal = self.read_number();
                    return Token::new(TokenType::INT, literal);
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch.to_string())
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;

        while self.is_valid_identifier_char(self.ch) {
            self.read_char();
        }

        self.input[pos..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }

        self.input[pos..self.position].to_string()
    }

    fn is_valid_identifier_char(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            ";

        let tests = vec![
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "five".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::INT, "5".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "ten".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::INT, "10".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "add".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::FUNCTION, "fn".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::IDENT, "x".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::IDENT, "y".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::LBRACE, "{".to_string()),
            Token::new(TokenType::IDENT, "x".to_string()),
            Token::new(TokenType::PLUS, "+".to_string()),
            Token::new(TokenType::IDENT, "y".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::RBRACE, "}".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::LET, "let".to_string()),
            Token::new(TokenType::IDENT, "result".to_string()),
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::IDENT, "add".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::IDENT, "five".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::IDENT, "ten".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::EOF, "\0".to_string()),
        ];

        let mut l = Lexer::new(input.to_string());

        for t in tests.iter() {
            let tok = l.next_token();
            assert_eq!(tok.token_type, t.token_type);
            assert_eq!(tok.literal, t.literal);
        }
    }
}
