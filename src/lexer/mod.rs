use crate::token::*;

#[derive(Debug)]
pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    pub(crate) fn new(input: String) -> Self {
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
            self.ch = self
                .input
                .chars()
                .nth(self.read_pos)
                .expect("Failed to get char");
        }
        self.position = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_pos >= self.input.len() {
            '0'
        } else {
            // This is safe because we know that read_pos is less than input.len()
            self.input.chars().nth(self.read_pos).unwrap()
        }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::Eq
                }
                _ => Token::Assign,
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::NotEq
                }
                _ => Token::Bang,
            },
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\0' => Token::Eof,
            _ => {
                if self.is_valid_identifier_char(self.ch) {
                    return self.read_identifier();
                } else if self.ch.is_digit(10) {
                    return self.read_number();
                } else {
                    Token::Illegal(self.ch.to_string())
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> Token {
        let pos = self.position;

        while self.is_valid_identifier_char(self.ch) {
            self.read_char();
        }

        let identifier = self.input[pos..self.position].to_string();

        match identifier.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(identifier),
        }
    }

    fn read_number(&mut self) -> Token {
        let pos = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }

        Token::Int(
            self.input[pos..self.position]
                .to_string()
                .parse()
                .expect("parse() failed"),
        )
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
    fn simple() -> Result<(), ()> {
        let input = "=+-(){}<>!*";

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::Minus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::LessThan,
            Token::GreaterThan,
            Token::Bang,
            Token::Asterisk,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_string());

        for expected_token in expected {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }

        Ok(())
    }

    #[test]
    fn advanced() {
        let input = "let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            ";

        let tests = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::GreaterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut l = Lexer::new(input.to_string());

        for t in tests.iter() {
            let tok = l.next_token();
            assert_eq!(&tok, t);
            assert_eq!(&tok, t);
        }
    }
}
