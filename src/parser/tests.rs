use super::*;

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

    // assert_eq!(program.statements[0], Statement::Return(Expression::Int(5)));
}

#[test]
fn identifier_expression() {
    let input = "foobar;";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);
    assert!(program.is_ok());

    let program = program.unwrap();
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::Expression(expr) => {
            assert_eq!(expr, &Expression::Identifier("foobar".to_string()));
        }
        _ => assert!(false),
    }
}

#[test]
fn int_expression() {
    let input = "5;";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);
    assert!(program.is_ok());

    let program = program.unwrap();
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::Expression(expr) => {
            assert_eq!(expr, &Expression::Int(5));
        }
        _ => assert!(false),
    }
}

#[test]
fn prefix_expression() {
    let input = "!5; -15;";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);
    assert!(program.is_ok());

    let program = program.unwrap();
    assert_eq!(program.statements.len(), 2);

    let stmt = &program.statements[0];
    match stmt {
        Statement::Expression(expr) => {
            assert_eq!(
                expr,
                &Expression::Prefix {
                    operator: PrefixOperator::Bang,
                    right: Box::new(Expression::Int(5)),
                }
            );
        }
        _ => assert!(false),
    }

    let stmt = &program.statements[1];
    match stmt {
        Statement::Expression(expr) => {
            assert_eq!(
                expr,
                &Expression::Prefix {
                    operator: PrefixOperator::Minus,
                    right: Box::new(Expression::Int(15)),
                }
            );
        }
        _ => assert!(false),
    }
}

#[test]
fn infix_expressions() {
    let tests = vec![
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
        ("5 == 5;", 5, "==", 5),
        ("5 != 5;", 5, "!=", 5),
    ];

    for (input, left, operator, right) in tests {
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);
        assert!(program.is_ok());

        let program = program.unwrap();

        let stmt = &program.statements[0];

        let Statement::Expression(expr) = stmt else {
            continue;
        };

        test_infix_expression(expr.clone(), left, operator, right);
    }
}

#[test]
fn operator_precedence() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);
        assert!(program.is_ok());

        let program = program.unwrap();
        assert_eq!(program.to_string(), expected);
    }
}

fn check_parser_errors(parser: Parser) {
    let nb_error = parser.errors.len();

    println!("Parser contains {} errors", nb_error);
    for error in parser.errors {
        println!("Parser error: {}", error);
    }

    assert!(nb_error == 0);
}

fn test_literal_expression(expr: Expression) {
    match expr {
        Expression::Int(value) => test_integer_literal(expr, value),
        Expression::Identifier(ref value) => test_identifier(expr.clone(), &(value.clone())),
        _ => assert!(false, "Expression is not a literal"),
    }
}

fn test_integer_literal(expr: Expression, value: i64) {
    match expr {
        Expression::Int(val) => {
            assert_eq!(
                val, value,
                "Int Literal has value {}, instead of {}",
                val, value
            );
        }
        _ => assert!(false, "Expression is not an integer literal"),
    }
}

fn test_identifier(expr: Expression, value: &str) {
    match expr {
        Expression::Identifier(val) => {
            assert_eq!(
                val, value,
                "Identifier has value {}, instead of {}",
                val, value
            );
        }
        _ => assert!(false, "Expression is not an identifier"),
    }
}

fn test_infix_expression(expr: Expression, left: i64, operator: &str, right: i64) {
    match expr {
        Expression::Infix {
            left: left_expr,
            operator: op,
            right: right_expr,
        } => {
            test_integer_literal(*left_expr, left);
            test_integer_literal(*right_expr, right);
            assert_eq!(
                op,
                <InfixOperator as std::str::FromStr>::from_str(operator).unwrap(),
                "Operator is not {}",
                operator
            );
        }
        _ => assert!(false, "Expression is not an infix expression"),
    }
}
