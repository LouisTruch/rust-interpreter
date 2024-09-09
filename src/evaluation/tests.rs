use crate::{lexer::Lexer, parser::Parser};

use super::*;

#[test]
fn eval_int_expression() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        // ("5 + 5 + 5 + 5 - 10", 10),
        // ("2 * 2 * 2 * 2 * 2", 32),
        // ("-50 + 100 + -50", 0),
        // ("5 * 2 + 10", 20),
        // ("5 + 2 * 10", 25),
        // ("20 + 2 * -10", 0),
        // ("50 / 2 * 2 + 10", 60),
        // ("2 * (5 + 10)", 30),
        // ("3 * 3 * 3 + 10", 37),
        // ("3 * (3 * 3) + 10", 37),
        // ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input.to_string());
        test_integer_object(evaluated, expected);
    }
}

fn test_eval(input: String) -> Object {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().expect("parse_program() failed");

    return program.eval().unwrap();
}

fn test_integer_object(obj: Object, expected: i64) {
    match obj {
        Object::Integer(value) => {
            assert_eq!(value, expected);
        }
        _ => {
            assert!(false, "Object is not Integer");
        }
    }
}
