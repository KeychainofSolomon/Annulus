use std::fmt;

use crate::ast::Literal;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
    Text(String),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "integer",
            Value::Boolean(_) => "boolean",
            Value::Text(_) => "string",
        }
    }
}

impl From<&Literal> for Value {
    fn from(literal: &Literal) -> Self {
        match literal {
            Literal::Integer(value) => Value::Integer(*value),
            Literal::Boolean(value) => Value::Boolean(*value),
            Literal::Text(text) => Value::Text(text.clone()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(value) => write!(formatter, "{value}"),
            Value::Boolean(value) => write!(formatter, "{value}"),
            Value::Text(text) => formatter.write_str(text),
        }
    }
}
