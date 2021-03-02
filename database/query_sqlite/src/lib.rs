use rusqlite::types::{FromSql, FromSqlError, FromSqlResult};
use std::fmt::Debug;

pub enum Value {
    Integer(i64),
    Float(f64),
    Null,
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(d) => Debug::fmt(d, f),
            Value::Float(r) => Debug::fmt(r, f),
            Value::Null => write!(f, "Null"),
        }
    }
}

impl FromSql for Value {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Null => Ok(Value::Null),
            rusqlite::types::ValueRef::Integer(i) => Ok(Value::Integer(i)),
            rusqlite::types::ValueRef::Real(r) => Ok(Value::Float(r)),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
