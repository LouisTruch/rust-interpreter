#[cfg(test)]
mod tests;

use crate::{
    ast::{Expression, Program, Statement},
    object::Object,
};

pub(crate) trait Eval {
    fn eval(self) -> Result<Object, ()>;
}

impl Eval for Program {
    fn eval(self) -> Result<Object, ()> {
        let mut result = Object::default();

        for statement in self.statements {
            result = statement.eval()?;
        }

        Ok(result)
    }
}

impl Eval for Statement {
    fn eval(self) -> Result<Object, ()> {
        match self {
            Statement::Expression(expression) => expression.eval(),
            _ => Err(()),
        }
    }
}

impl Eval for Expression {
    fn eval(self) -> Result<Object, ()> {
        match self {
            Expression::Int(i) => Ok(Object::Integer(i)),
            _ => Err(()),
        }
    }
}
