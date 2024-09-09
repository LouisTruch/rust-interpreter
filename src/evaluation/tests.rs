use crate::{lexer::Lexer, parser::Parser};

use super::*;

#[test]
fn int_expression() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input.to_string());
        test_integer_object(evaluated.unwrap(), expected);
    }
}

fn test_eval(input: String) -> Result<Object, EvalError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().expect("parse_program() failed");
    let env = Environment::new_rc();

    program.eval(env)
}

fn test_integer_object(obj: Object, expected: i64) {
    match obj {
        Object::Integer(value) => {
            assert_eq!(value, expected);
        }
        _ => {
            assert!(false, "Object is not Integer but {}", obj);
        }
    }
}

#[test]
fn bool_expression() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input.to_string());
        test_boolean_object(evaluated.unwrap(), expected);
    }
}

#[test]
fn bang_operator() {
    let tests = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input.to_string());
        test_boolean_object(evaluated.unwrap(), expected);
    }
}

fn test_boolean_object(obj: Object, expected: bool) {
    match obj {
        Object::Bool(value) => {
            assert_eq!(value, expected);
        }
        _ => {
            assert!(false, "Object is not Bool");
        }
    }
}

#[test]
fn if_else_expressions() {
    let tests = vec![
        ("if (true) { 10 }", Object::Integer(10)),
        ("if (false) { 10 }", Object::Null),
        ("if (1) { 10 }", Object::Integer(10)),
        ("if (1 < 2) { 10 }", Object::Integer(10)),
        ("if (1 > 2) { 10 }", Object::Null),
        ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
        ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input.to_string());

        if let Object::Integer(expected) = expected {
            test_integer_object(evaluated.unwrap(), expected);
        } else {
            test_null_object(evaluated.unwrap());
        }
    }
}

fn test_null_object(obj: Object) {
    if let Object::Null = obj {
    } else {
        assert!(false, "Object is not Null, instead is {}", obj);
    }
}

#[test]
fn return_statement() {
    let tests = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            r#" if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }"#,
            10,
        ),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input.to_string());
        test_integer_object(evaluated.unwrap(), expected);
    }
}

#[test]
fn error_handling() {
    let tests = vec![
        (
            "5 + true; 5;",
            EvalError::MismatchedTypes {
                left: Object::Integer(5),
                operator: InfixOperator::Plus,
                right: Object::Bool(true),
            },
        ),
        (
            "-true",
            EvalError::UnknownPrefix {
                operator: PrefixOperator::Minus,
                value: Object::Bool(true),
            },
        ),
        (
            "true + false;",
            EvalError::InfixBooleanOperator {
                left: true,
                operator: InfixOperator::Plus,
                right: false,
            },
        ),
        (
            "5; true + false; 5",
            EvalError::InfixBooleanOperator {
                left: true,
                operator: InfixOperator::Plus,
                right: false,
            },
        ),
        (
            r#"
            if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }
                return 1;
            }
            "#,
            EvalError::InfixBooleanOperator {
                left: true,
                operator: InfixOperator::Plus,
                right: false,
            },
        ),
        (
            "foobar",
            EvalError::IdentifierNotFound("foobar".to_string()),
        ),
    ];

    for (input, expected) in tests {
        let result = test_eval(input.to_string());
        test_error_object(result, expected);
    }
}

fn test_error_object(result: Result<Object, EvalError>, expected: EvalError) {
    match result {
        Ok(_) => {
            assert!(false, "Expected error but got Ok");
        }
        Err(err) => {
            assert_eq!(err, expected);
        }
    }
}

#[test]
fn let_statement() {
    let tests = vec![
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input.to_string());
        test_integer_object(evaluated.unwrap(), expected);
    }
}
