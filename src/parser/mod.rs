use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::Token,
};

struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            curr_token: Token::default(),
            peek_token: Token::default(),
            errors: vec![],
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Result<Program, ()> {
        let mut program = Program { statements: vec![] };

        while self.curr_token != Token::Eof {
            match self.parse_statement() {
                Some(stmt) => program.statements.push(stmt),
                None => (),
            }
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token {
            Token::Let => self.parse_let(),
            Token::Return => self.parse_return(),
            _ => None,
        }
    }

    fn parse_let(&mut self) -> Option<Statement> {
        // First thing after the let keyword should be an identifier
        let Token::Ident(name) = self.peek_token.clone() else {
            return None;
        };

        // Then it should be followed by an assign token
        self.next_token();
        if !self.expect_peek(Token::Assign) {
            return None;
        }

        while !self.curr_token_is(&Token::Semicolon) && !self.curr_token_is(&Token::Eof) {
            self.next_token();
        }

        Some(Statement::Let {
            name,
            value: Expression::default(),
        })
    }

    fn parse_return(&mut self) -> Option<Statement> {
        self.next_token();

        while !self.curr_token_is(&Token::Semicolon) && !self.curr_token_is(&Token::Eof) {
            self.next_token();
        }

        Some(Statement::Return(Expression::default()))
    }

    fn curr_token_is(&self, t: &Token) -> bool {
        &self.curr_token == t
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        &self.peek_token == t
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        let is_correct_token = self.peek_token_is(&t);

        if is_correct_token {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: Token) {
        self.errors.push(format!(
            "Expected Token to be {:?}, got {:?} instead",
            t, self.curr_token
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn let_statements() {
        let input = "let x = 5;
            let y = 10;
            let foobar = 838383;";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);
        assert!(program.is_ok());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3);

        assert_eq!(
            program.statements[0],
            Statement::Let {
                name: "x".to_string(),
                value: Expression::default(),
            }
        );

        assert_eq!(
            program.statements[1],
            Statement::Let {
                name: "y".to_string(),
                value: Expression::default(),
            }
        );

        assert_eq!(
            program.statements[2],
            Statement::Let {
                name: "foobar".to_string(),
                value: Expression::default(),
            }
        );
    }

    #[test]
    fn return_statements() {
        let input = "return 5;
            return 10;
            return 993322;";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);
        assert!(program.is_ok());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3);
    }

    fn check_parser_errors(parser: Parser) {
        if parser.errors.len() == 0 {
            return;
        }

        println!("Parser contains {} errors", parser.errors.len());
        for error in parser.errors {
            println!("Parser error: {}", error);
        }

        assert!(false);
    }
}
