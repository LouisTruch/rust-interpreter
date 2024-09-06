#[cfg(test)]
mod tests;

use crate::{
    ast::{Expression, InfixOperator, PrefixOperator, Program, Statement},
    lexer::Lexer,
    token::Token,
};

#[derive(Debug)]
struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub(super) errors: Vec<ParserError>,
}

#[derive(Clone, Debug)]
pub(super) enum ParserError {
    UnexpectedToken { expected: Token, got: Token },
    InvalidPrefixOperator { operator: Token },
    InvalidInfixOperator { operator: Token },
    MissRightParenthesis { operator: Token },
    UnhandledError,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken { expected, got } => {
                write!(f, "Expected token {:?}, got {:?}", expected, got)
            }
            ParserError::InvalidPrefixOperator { operator } => {
                write!(f, "Invalid prefix operator {:?}", operator)
            }
            ParserError::InvalidInfixOperator { operator } => {
                write!(f, "Invalid infix operator {:?}", operator)
            }
            ParserError::MissRightParenthesis { operator } => {
                write!(f, "Missing Closing parenthesis, got {} instead", operator)
            }
            ParserError::UnhandledError => write!(f, "Unhandled error"),
        }
    }
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            curr_token,
            peek_token,
            errors: vec![],
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Result<Program, ()> {
        let mut program = Program { statements: vec![] };

        while self.curr_token != Token::Eof {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(e) => self.errors.push(e),
            }
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.curr_token {
            Token::Let => self.parse_statement_let(),
            Token::Return => self.parse_statement_ret(),
            _ => self.parse_statement_expr(),
        }
    }

    fn parse_statement_let(&mut self) -> Result<Statement, ParserError> {
        // First thing after the let keyword should be an identifier
        let Token::Ident(name) = self.peek_token.clone() else {
            return Err(ParserError::UnexpectedToken {
                expected: Token::Ident("".to_string()),
                got: self.peek_token.clone(),
            });
        };

        // Then it should be followed by an assign token
        self.next_token();
        let _ = self.expect_peek(Token::Assign)?;

        // Then we parse the expression
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;

        // Then we might have a semicolon (but not necessarily)
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Let { name, value })
    }

    fn parse_statement_ret(&mut self) -> Result<Statement, ParserError> {
        // Skip the return keyword
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        // Then we might have a semicolon (but not necessarily)
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Return(expr))
    }

    fn parse_statement_expr(&mut self) -> Result<Statement, ParserError> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Expression(expr))
    }

    fn parse_statement_block(&mut self) -> Vec<Statement> {
        let mut block: Vec<Statement> = vec![];

        self.next_token();

        while !self.curr_token_is(&Token::RBrace) && !self.curr_token_is(&Token::Eof) {
            let statement = self.parse_statement();
            if let Ok(statement) = statement {
                block.push(statement);
            }
            self.next_token();
        }

        block
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        // This is in replacement of the prefix fns map in the book
        let mut left = match self.curr_token.clone() {
            Token::Ident(str) => self.parse_expr_identifier(&str),
            Token::Int(nb) => self.parse_expr_integer(nb),
            Token::Bang | Token::Minus => self.parse_expr_prefix()?,
            Token::True | Token::False => self.parse_expr_boolean(),
            Token::LParen => self.parse_expr_grouped()?,
            Token::If => self.parse_expr_if()?,
            Token::Function => self.parse_expr_function()?,
            _ => {
                return Err(ParserError::InvalidPrefixOperator {
                    operator: self.curr_token.clone(),
                })
            }
        };

        while !self.peek_token_is(&Token::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            // This is in replacement of the infix fns map in the book
            left = match self.curr_token {
                Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::Slash
                | Token::Eq
                | Token::NotEq
                | Token::LessThan
                | Token::GreaterThan => self.parse_expr_infix(&left)?,
                Token::LParen => self.parse_expr_call(&left)?,
                _ => return Ok(left),
            };
        }

        Ok(left)
    }

    fn parse_expr_identifier(&mut self, str: &str) -> Expression {
        Expression::Identifier(str.to_string())
    }

    fn parse_expr_integer(&mut self, nb: i64) -> Expression {
        Expression::Int(nb)
    }

    fn parse_expr_prefix(&mut self) -> Result<Expression, ParserError> {
        let Ok(prefix) = PrefixOperator::try_from(&self.curr_token) else {
            return Err(ParserError::InvalidPrefixOperator {
                operator: self.curr_token.clone(),
            });
        };

        self.next_token();

        let expr = self.parse_expression(Precedence::Prefix).unwrap();

        Ok(Expression::Prefix {
            operator: prefix,
            right: Box::new(expr),
        })
    }

    fn parse_expr_infix(&mut self, left: &Expression) -> Result<Expression, ParserError> {
        let precedence = self.curr_precedence();
        let operator = InfixOperator::from(&self.curr_token);

        self.next_token();
        let right = self.parse_expression(precedence)?;

        Ok(Expression::Infix {
            left: Box::new(left.clone()),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_expr_call(&mut self, left: &Expression) -> Result<Expression, ParserError> {
        let arguments = self.parse_call_arguments()?;

        Ok(Expression::FunctionCall {
            function: Box::new(left.clone()),
            arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut arguments: Vec<Expression> = vec![];

        if self.peek_token_is(&Token::RParen) {
            self.next_token();
            return Ok(arguments);
        }

        self.next_token();
        arguments.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            arguments.push(self.parse_expression(Precedence::Lowest)?);
        }

        let _ = self.expect_peek(Token::RParen)?;

        return Ok(arguments);
    }

    fn parse_expr_boolean(&mut self) -> Expression {
        Expression::Bool(self.curr_token_is(&Token::True))
    }

    fn parse_expr_grouped(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest)?;

        let _ = self
            .expect_peek(Token::RParen)
            .map_err(|_| ParserError::MissRightParenthesis {
                operator: self.curr_token.clone(),
            })?;

        Ok(expr)
    }

    fn parse_expr_if(&mut self) -> Result<Expression, ParserError> {
        let _ = self.expect_peek(Token::LParen)?;

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        let _ = self.expect_peek(Token::RParen)?;
        let _ = self.expect_peek(Token::LBrace)?;

        let consequence = self.parse_statement_block();

        let alternative = if self.peek_token_is(&Token::Else) {
            self.next_token();
            let _ = self.expect_peek(Token::LBrace)?;
            Some(self.parse_statement_block())
        } else {
            None
        };

        Ok(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_expr_function(&mut self) -> Result<Expression, ParserError> {
        let _ = self.expect_peek(Token::LParen)?;

        let parameters = self.parse_function_parameters()?;

        let _ = self.expect_peek(Token::LBrace)?;

        let body = self.parse_statement_block();

        Ok(Expression::Function { parameters, body })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut identifiers: Vec<Expression> = vec![];

        self.next_token();
        // Empty argument list
        if self.curr_token_is(&Token::RParen) {
            return Ok(identifiers);
        }

        identifiers.push(Expression::Identifier(self.curr_token.to_string()));

        while self.peek_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(Expression::Identifier(self.curr_token.to_string()));
        }

        let _ = self.expect_peek(Token::RParen)?;

        Ok(identifiers)
    }

    fn curr_token_is(&self, t: &Token) -> bool {
        &self.curr_token == t
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        &self.peek_token == t
    }

    fn expect_peek(&mut self, t: Token) -> Result<Token, ParserError> {
        match self.peek_token_is(&t) {
            true => {
                self.next_token();
                Ok(self.curr_token.clone())
            }
            false => Err(self.peek_error(t)),
        }
    }

    fn curr_precedence(&self) -> Precedence {
        (&self.curr_token).into()
    }

    fn peek_precedence(&self) -> Precedence {
        (&self.peek_token).into()
    }

    fn peek_error(&mut self, t: Token) -> ParserError {
        ParserError::UnexpectedToken {
            expected: t,
            got: self.curr_token.clone(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Precedence {
    Lowest = 1,      // Default
    Equals = 2,      // == or !=
    LessGreater = 3, // > or <
    Sum = 4,         // + or -
    Product = 5,     // * or /
    Prefix = 6,      // -x or !x
    Call = 7,        // fn(x)
}

impl From<&Token> for Precedence {
    fn from(value: &Token) -> Self {
        match value {
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}
