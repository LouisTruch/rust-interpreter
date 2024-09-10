use crate::{
    ast::{Expression, Statement},
    Environment,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum Object {
    #[default]
    Null,
    Integer(i64),
    Bool(bool),
    ReturnValue {
        value: Box<Object>,
    },
    Function {
        // Should be Expression::Identifiers
        parameters: Vec<Expression>,
        body: Vec<Statement>,
        env: Environment,
    },
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Integer(i) => write!(f, "{}", i),
            Object::Bool(b) => write!(f, "{}", b),
            Object::ReturnValue { value } => write!(f, "{}", value),
            Object::Function {
                parameters, body, ..
            } => {
                write!(
                    f,
                    "fn({}) ",
                    parameters
                        .iter()
                        .map(|expr| expr.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;

                write!(
                    f,
                    "{{\n {}\n}}",
                    body.iter()
                        .map(|stmt| stmt.to_string())
                        .collect::<Vec<String>>()
                        .join("\n ")
                )
            }
        }
    }
}

impl Object {
    fn object_type(&self) -> String {
        match self {
            Object::Null => "NULL".to_string(),
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Bool(_) => "BOOLEAN".to_string(),
            Object::ReturnValue { .. } => "RETURN_VALUE".to_string(),
            Object::Function { .. } => "FUNCTION".to_string(),
        }
    }
}
