use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, Value as SQLValue, ValueRef},
    Connection, Result, Row, ToSql,
};
use std::{
    error::Error,
    fmt::{Debug, Display},
    str::{from_utf8, FromStr},
};

pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Null,
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => Display::fmt(s, f),
            Value::Integer(d) => Debug::fmt(d, f),
            Value::Float(r) => Debug::fmt(r, f),
            Value::Null => write!(f, "Null"),
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = if let Ok(i) = s.parse::<i64>() {
            Value::Integer(i)
        } else if let Ok(f) = s.parse::<f64>() {
            Value::Float(f)
        } else {
            Value::String(s.to_string())
        };

        Ok(value)
    }
}

impl FromSql for Value {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Null => Ok(Value::Null),
            ValueRef::Integer(i) => Ok(Value::Integer(i)),
            ValueRef::Real(r) => Ok(Value::Float(r)),
            ValueRef::Text(s) => from_utf8(s)
                .map(|s| Value::String(s.to_string()))
                .map_err(|_| FromSqlError::InvalidType),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Value {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        let value = match self {
            Value::String(s) => ToSqlOutput::Borrowed(ValueRef::Text(s.as_bytes())),
            Value::Integer(i) => ToSqlOutput::Owned(SQLValue::Integer(*i)),
            Value::Float(f) => ToSqlOutput::Owned(SQLValue::Real(*f)),
            Value::Null => ToSqlOutput::Owned(SQLValue::Null),
        };

        Ok(value)
    }
}

fn collect(row: &Row) -> Result<Vec<Value>> {
    let mut vec = vec![];
    for i in 0..row.column_count() {
        vec.push(row.get(i)?);
    }
    Ok(vec)
}

pub fn exec<P>(conn: &Connection, stmt: &str, params: P) -> Result<Vec<Vec<Value>>>
where
    P: IntoIterator,
    P::Item: ToSql,
{
    let mut stmt = conn.prepare(stmt)?;
    let rows = stmt.query_map(params, collect)?;
    rows.collect()
}

pub fn exec_and_print(conn: &Connection, stmt: &str) -> Result<()> {
    println!("{}", stmt);
    for row in exec(conn, stmt, params![])? {
        println!("{:?}", row);
    }
    Ok(())
}

pub fn insert_into(conn: &Connection, csv: &str, stmt: &str) -> Result<(), Box<dyn Error>> {
    let reader = csv::ReaderBuilder::new().delimiter(b' ').from_path(csv)?;
    for record in reader.into_records() {
        let record = record?;
        let values = record.iter().map(|s| s.parse::<Value>().unwrap());
        conn.execute(stmt, values)?;
    }

    Ok(())
}
