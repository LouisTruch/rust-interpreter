#[derive(Default)]
pub struct Program {
    pub(crate) statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
}

impl Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { name, .. } => name.clone(),
            Statement::Return { .. } => "return".to_string(),
            Statement::Expression(_) => "".to_string(),
            Statement::Block(_) => "".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Expression {
    #[default]
    A,
}
