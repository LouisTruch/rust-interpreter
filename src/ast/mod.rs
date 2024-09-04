use std::str::FromStr;

use crate::token::Token;

// Program is the root node of the AST
#[derive(Default)]
pub struct Program {
    pub(crate) statements: Vec<Statement>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.to_string());
        }
        write!(f, "{}", out)
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let { name, value } => write!(f, "let {name} = {value};"),
            Statement::Return(value) => write!(f, "return {value};"),
            Statement::Expression(value) => write!(f, "{value}"),
            Statement::Block(statements) => {
                let mut out = String::new();
                for stmt in statements {
                    out.push_str(&stmt.to_string());
                }
                write!(f, "{out}")
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Expression {
    #[default]
    A,
    Identifier(String),
    Int(i64),
    Prefix {
        operator: PrefixOperator,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    Bool(bool),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::A => write!(f, "A"),
            Expression::Identifier(name) => write!(f, "{name}"),
            Expression::Int(value) => write!(f, "{value}"),
            Expression::Prefix { operator, right } => write!(f, "({operator}{right})"),
            Expression::Infix {
                left,
                operator,
                right,
            } => write!(f, "({left} {operator} {right})"),
            Expression::Bool(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PrefixOperator {
    Bang,
    Minus,
}

impl std::fmt::Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOperator::Bang => write!(f, "!"),
            PrefixOperator::Minus => write!(f, "-"),
        }
    }
}

impl TryFrom<&Token> for PrefixOperator {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Bang => Ok(Self::Bang),
            Token::Minus => Ok(Self::Minus),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InfixOperator {
    Plus,
    Minus,
    Mult,
    Division,
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}

impl std::fmt::Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperator::Plus => write!(f, "+"),
            InfixOperator::Minus => write!(f, "-"),
            InfixOperator::Mult => write!(f, "*"),
            InfixOperator::Division => write!(f, "/"),
            InfixOperator::GreaterThan => write!(f, ">"),
            InfixOperator::LessThan => write!(f, "<"),
            InfixOperator::Equal => write!(f, "=="),
            InfixOperator::NotEqual => write!(f, "!="),
        }
    }
}

impl From<&Token> for InfixOperator {
    fn from(value: &Token) -> Self {
        match value {
            Token::Plus => Self::Plus,
            Token::Minus => Self::Minus,
            Token::Asterisk => Self::Mult,
            Token::Slash => Self::Division,
            Token::GreaterThan => Self::GreaterThan,
            Token::LessThan => Self::LessThan,
            Token::Eq => Self::Equal,
            Token::NotEq => Self::NotEqual,
            _ => panic!("Invalid token"),
        }
    }
}

// Used for tests
impl FromStr for InfixOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Mult),
            "/" => Ok(Self::Division),
            ">" => Ok(Self::GreaterThan),
            "<" => Ok(Self::LessThan),
            "==" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_to_string() {
        let program = Program {
            statements: vec![
                Statement::Let {
                    name: "myVar".to_string(),
                    value: Expression::Identifier("anotherVar".to_string()),
                },
                Statement::Return(Expression::A),
            ],
        };

        assert_eq!(program.to_string(), "let myVar = anotherVar;return A;");
    }
}
