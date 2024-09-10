#[cfg(test)]
mod tests;

pub mod environment;

use std::rc::Rc;

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
    MismatchedObject {
        expected: String,
        got: Object,
    },
    InvalidNumberArguments {
        expected: u64,
        got: u64,
    },
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
                expression.eval(environment).map(|obj| Object::ReturnValue {
                    value: Box::new(obj),
                })
            }
            Statement::Let { name, value } => {
                let result = value.eval(environment.clone());
                if result.is_err() {
                    return Ok(result?);
                }

                let obj = environment.set(name, result?);

                Ok(obj)
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
            Expression::Function { parameters, body } => Ok(Object::Function {
                parameters,
                body,
                env: (*environment).clone(),
            }),
            Expression::FunctionCall {
                function,
                arguments,
            } => {
                let obj_fn = function.eval(environment.clone())?;

                let arguments: Vec<Object> = arguments
                    .into_iter()
                    .map(|arg| arg.eval(environment.clone()))
                    .collect::<Result<Vec<_>, _>>()?;

                apply_function(obj_fn, arguments)
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

fn apply_function(obj_fn: Object, args: Vec<Object>) -> Result<Object, EvalError> {
    let Object::Function {
        body, parameters, ..
    } = obj_fn.clone()
    else {
        return Err(EvalError::MismatchedObject {
            expected: "Object::Function{ }".to_string(),
            got: obj_fn,
        });
    };

    if parameters.len() != args.len() {
        return Err(EvalError::InvalidNumberArguments {
            expected: parameters.len() as u64,
            got: args.len() as u64,
        });
    }

    let extended_env = extend_function_env(obj_fn, args)?;

    let obj = Statement::Block(body).eval(extended_env)?;
    Ok(match obj {
        Object::ReturnValue { value } => *value,
        _ => obj,
    })
}

fn extend_function_env(obj_fn: Object, args: Vec<Object>) -> Result<Rc<Environment>, EvalError> {
    let Object::Function {
        parameters, env, ..
    } = obj_fn
    else {
        return Err(EvalError::MismatchedObject {
            expected: "Object::Function{ }".to_string(),
            got: obj_fn,
        });
    };

    let env = Environment::with_outer(Rc::new(env));

    for (param, arg) in parameters.into_iter().zip(args) {
        env.set(param, arg);
    }

    Ok(env)
}
