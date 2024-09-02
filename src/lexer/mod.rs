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
        let tok_type = match self.ch {
            '=' => TokenType::ASSIGN,
            ';' => TokenType::SEMICOLON,
            '(' => TokenType::LPAREN,
            ')' => TokenType::RPAREN,
            ',' => TokenType::COMMA,
            '+' => TokenType::PLUS,
            '{' => TokenType::LBRACE,
            '}' => TokenType::RBRACE,
            '\0' => TokenType::EOF,
            _ => TokenType::ILLEGAL,
        };
        let tok = Token::new(tok_type, self.ch.to_string());

        self.read_char();
        tok
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "+(){},;";
        let tests = vec![
            Token { token_type: TokenType::PLUS, literal: "+".to_string() },
            Token { token_type: TokenType::LPAREN, literal: "(".to_string() },
            Token { token_type: TokenType::RPAREN, literal: ")".to_string() },
            Token { token_type: TokenType::LBRACE, literal: "{".to_string() },
            Token { token_type: TokenType::RBRACE, literal: "}".to_string() },
            Token { token_type: TokenType::COMMA, literal: ",".to_string() },
            Token { token_type: TokenType::SEMICOLON, literal: ";".to_string() },
            Token { token_type: TokenType::EOF, literal: "\0".to_string() },
        ];

        let mut l = Lexer::new(input.to_string());

        for tt in tests.iter() {
            let tok = l.next_token();
            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }
}
