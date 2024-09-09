#[derive(Default)]
pub(crate) enum Object {
    #[default]
    Null,
    Integer(i64),
    Bool(bool),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Integer(i) => write!(f, "{}", i),
            Object::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl Object {
    fn object_type(&self) -> String {
        match self {
            Object::Null => "NULL".to_string(),
            Object::Integer(_) => "INTEGER".to_string(),
            Object::Bool(_) => "BOOLEAN".to_string(),
        }
    }
}
