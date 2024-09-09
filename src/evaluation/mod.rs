#[cfg(test)]
mod tests;

pub mod environment;

use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use environment::Environment;

use crate::{
    ast::{Expression, InfixOperator, PrefixOperator, Program, Statement},
    object::Object,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EvalError {
    MismatchedTypes {
        left: Object,
        operator: InfixOperator,
        right: Object,
    },
    UnknownPrefix {
        operator: PrefixOperator,
        value: Object,
    },
    InfixBooleanOperator {
        left: bool,
        operator: InfixOperator,
        right: bool,
    },
    IdentifierNotFound(String),
    Custom(String),
    Unhandled,
}

pub(crate) trait Eval {
    fn eval(self, environment: Rc<Environment>) -> Result<Object, EvalError>;
}

impl Eval for Program {
    fn eval(self, environment: Rc<Environment>) -> Result<Object, EvalError> {
        let mut result = Ok(Object::Null);

        for statement in self.statements {
            result = statement.eval(environment.clone());

            match result.clone() {
                Ok(Object::ReturnValue { value }) => return Ok(*value),
                Err(_) => return Ok(result?),
                _ => {}
            }
        }

        Ok(result?)
    }
}

impl Eval for Statement {
    fn eval(self, environment: Rc<Environment>) -> Result<Object, EvalError> {
        match self {
            Statement::Expression(expression) => expression.eval(environment.clone()),
            Statement::Block(statements) => {
                let mut result = Ok(Object::Null);

                for statement in statements {
                    result = statement.eval(environment.clone());

                    match result.clone() {
                        Ok(Object::ReturnValue { value: _ }) => return Ok(result?),
                        Err(_) => return Ok(result?),
                        _ => {}
                    }
                }

                Ok(result?)
            }
            Statement::Return(expression) => {
                expression
                    .eval(environment.clone())
                    .map(|obj| Object::ReturnValue {
                        value: Box::new(obj),
                    })
            }
            Statement::Let { name, value } => {
                let result = value.eval(environment.clone());
                if result.is_err() {
                    return Ok(result?);
                }

                environment.set(name, result?);

                Ok(Object::Null)
                // let mut result = Ok(Object:)
            }
        }
    }
}

impl Eval for Expression {
    fn eval(self, environment: Rc<Environment>) -> Result<Object, EvalError> {
        match self {
            Expression::Int(i) => Ok(Object::Integer(i)),
            Expression::Bool(b) => Ok(Object::Bool(b)),
            Expression::Prefix { operator, right } => {
                let right = right.eval(environment)?;
                eval_expr_prefix(operator, right)
            }
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                let left = left.eval(environment.clone())?;
                let right = right.eval(environment.clone())?;
                eval_expr_infix(operator, left, right)
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let condition = condition.eval(environment.clone())?;

                if is_true(condition) {
                    Statement::Block(consequence).eval(environment.clone())
                } else if alternative.is_some() {
                    // Safe because we of else if condition
                    Statement::Block(alternative.unwrap()).eval(environment.clone())
                } else {
                    Ok(Object::Null)
                }
            }
            Expression::Identifier(str) => {
                let val = environment.get(&str);

                if val == Object::Null {
                    return Err(EvalError::IdentifierNotFound(str.to_string()));
                }

                return Ok(val);
            }
            _ => Err(EvalError::Unhandled),
        }
    }
}

fn eval_expr_prefix(operator: PrefixOperator, right: Object) -> Result<Object, EvalError> {
    match operator {
        PrefixOperator::Bang => eval_expr_bang_operator(right),
        PrefixOperator::Minus => eval_expr_minus_operator(right),
    }
}

fn eval_expr_bang_operator(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Bool(b) => Ok(Object::Bool(!b)),
        Object::Integer(i) => Ok(Object::Bool(i == 0)),
        _ => Err(EvalError::UnknownPrefix {
            operator: PrefixOperator::Bang,
            value: right,
        }),
    }
}

fn eval_expr_minus_operator(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Integer(i) => Ok(Object::Integer(-i)),
        _ => Err(EvalError::UnknownPrefix {
            operator: PrefixOperator::Minus,
            value: right,
        }),
    }
}

fn eval_expr_infix(
    operator: InfixOperator,
    left: Object,
    right: Object,
) -> Result<Object, EvalError> {
    match (left.clone(), right.clone()) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_expr_infix_integer(operator, left, right)
        }
        (Object::Bool(left), Object::Bool(right)) => eval_expr_infix_bool(operator, left, right),
        _ => Err(EvalError::MismatchedTypes {
            left,
            operator,
            right,
        }),
    }
}

fn eval_expr_infix_integer(
    operator: InfixOperator,
    left: i64,
    right: i64,
) -> Result<Object, EvalError> {
    match operator {
        InfixOperator::Plus => Ok(Object::Integer(left + right)),
        InfixOperator::Minus => Ok(Object::Integer(left - right)),
        InfixOperator::Mult => Ok(Object::Integer(left * right)),
        InfixOperator::Division => Ok(Object::Integer(left / right)),
        InfixOperator::Equal => Ok(Object::Bool(left == right)),
        InfixOperator::NotEqual => Ok(Object::Bool(left != right)),
        InfixOperator::GreaterThan => Ok(Object::Bool(left > right)),
        InfixOperator::LessThan => Ok(Object::Bool(left < right)),
    }
}

fn eval_expr_infix_bool(
    operator: InfixOperator,
    left: bool,
    right: bool,
) -> Result<Object, EvalError> {
    match operator {
        InfixOperator::Equal => Ok(Object::Bool(left == right)),
        InfixOperator::NotEqual => Ok(Object::Bool(left != right)),
        _ => Err(EvalError::InfixBooleanOperator {
            left,
            operator,
            right,
        }),
    }
}

fn is_true(condition: Object) -> bool {
    match condition {
        Object::Null => false,
        Object::Bool(b) => b,
        _ => true,
    }
}
