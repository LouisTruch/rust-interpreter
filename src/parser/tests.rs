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
            value: Expression::Int(5),
        }
    );

    assert_eq!(
        program.statements[1],
        Statement::Let {
            name: "y".to_string(),
            value: Expression::Int(10),
        }
    );

    assert_eq!(
        program.statements[2],
        Statement::Let {
            name: "foobar".to_string(),
            value: Expression::Int(838383),
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

    assert_eq!(program.statements[0], Statement::Return(Expression::Int(5)));

    assert_eq!(
        program.statements[1],
        Statement::Return(Expression::Int(10))
    );

    assert_eq!(
        program.statements[2],
        Statement::Return(Expression::Int(993322))
    );
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
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
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

#[test]
fn bool_expression() {
    let input = "true; false;";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);
    assert!(program.is_ok());

    let program = program.unwrap();
    assert_eq!(program.statements.len(), 2);

    match &program.statements[0] {
        Statement::Expression(expr) => {
            assert_eq!(expr, &Expression::Bool(true));
        }
        _ => assert!(false),
    }

    match &program.statements[1] {
        Statement::Expression(expr) => {
            assert_eq!(expr, &Expression::Bool(false));
        }
        _ => assert!(false),
    }
}

#[test]
fn if_expression() {
    let input = "if (x < y) { x }";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);
    assert!(program.is_ok());

    let program = program.unwrap();
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::Expression(expr) => {
            assert_eq!(
                expr,
                &Expression::If {
                    condition: Box::new(Expression::Infix {
                        left: Box::new(Expression::Identifier("x".to_string())),
                        operator: InfixOperator::LessThan,
                        right: Box::new(Expression::Identifier("y".to_string())),
                    }),
                    consequence: vec![Statement::Expression(Expression::Identifier(
                        "x".to_string()
                    ))],
                    alternative: None,
                }
            );
        }
        _ => assert!(false),
    }
}

#[test]
fn if_else_expression() {
    let input = "if (x < y) { x } else { y }";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);
    assert!(program.is_ok());

    let program = program.unwrap();
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::Expression(expr) => {
            assert_eq!(
                expr,
                &Expression::If {
                    condition: Box::new(Expression::Infix {
                        left: Box::new(Expression::Identifier("x".to_string())),
                        operator: InfixOperator::LessThan,
                        right: Box::new(Expression::Identifier("y".to_string())),
                    }),
                    consequence: vec![Statement::Expression(Expression::Identifier(
                        "x".to_string()
                    ))],
                    alternative: Some(vec![Statement::Expression(Expression::Identifier(
                        "y".to_string()
                    ))]),
                }
            );
        }
        _ => assert!(false),
    }
}

#[test]
fn function_expression() {
    let input = "fn(x, y) { x + y; }";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);
    assert!(program.is_ok());

    let program = program.unwrap();
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::Expression(expr) => {
            assert_eq!(
                expr,
                &Expression::Function {
                    parameters: vec![
                        Expression::Identifier("x".to_string()),
                        Expression::Identifier("y".to_string())
                    ],
                    body: vec![Statement::Expression(Expression::Infix {
                        left: Box::new(Expression::Identifier("x".to_string())),
                        operator: InfixOperator::Plus,
                        right: Box::new(Expression::Identifier("y".to_string())),
                    })],
                }
            );
        }
        _ => assert!(false),
    }
}

#[test]
fn function_parameters() {
    let tests = vec![
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);
        assert!(program.is_ok());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::Expression(expr) => {
                assert_eq!(
                    expr,
                    &Expression::Function {
                        parameters: expected
                            .iter()
                            .map(|str| Expression::Identifier(str.to_string()))
                            .collect(),
                        body: vec![],
                    }
                );
            }
            _ => assert!(false),
        }
    }
}

#[test]
fn function_call() {
    let input = "add(1, 2 * 3, 4 + 5);";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(parser);
    assert!(program.is_ok());

    let program = program.unwrap();
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::Expression(expr) => {
            assert_eq!(
                expr,
                &Expression::FunctionCall {
                    function: Box::new(Expression::Identifier("add".to_string())),
                    arguments: vec![
                        Expression::Int(1),
                        Expression::Infix {
                            left: Box::new(Expression::Int(2)),
                            operator: InfixOperator::Mult,
                            right: Box::new(Expression::Int(3)),
                        },
                        Expression::Infix {
                            left: Box::new(Expression::Int(4)),
                            operator: InfixOperator::Plus,
                            right: Box::new(Expression::Int(5)),
                        },
                    ],
                }
            );
        }
        _ => assert!(false),
    }
}

fn function_call_arguments() {
    let tests = vec![
        ("add();", vec![]),
        ("add(1);", vec![Expression::Int(1)]),
        (
            "add(1, 2 * 3, 4 + 5);",
            vec![
                Expression::Int(1),
                Expression::Infix {
                    left: Box::new(Expression::Int(2)),
                    operator: InfixOperator::Mult,
                    right: Box::new(Expression::Int(3)),
                },
                Expression::Infix {
                    left: Box::new(Expression::Int(4)),
                    operator: InfixOperator::Plus,
                    right: Box::new(Expression::Int(5)),
                },
            ],
        ),
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(parser);
        assert!(program.is_ok());

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::Expression(expr) => {
                assert_eq!(
                    expr,
                    &Expression::FunctionCall {
                        function: Box::new(Expression::Identifier("add".to_string())),
                        arguments: expected,
                    }
                );
            }
            _ => assert!(false),
        }
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
        Expression::Bool(value) => test_bool_literal(expr, value),
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

fn test_bool_literal(expr: Expression, value: bool) {
    match expr {
        Expression::Bool(val) => {
            assert_eq!(
                val, value,
                "Bool Literal has value {}, instead of {}",
                val, value
            );
        }
        _ => assert!(false, "Expression is not a bool literal"),
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
