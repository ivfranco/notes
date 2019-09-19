use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Clone, Copy)]
pub enum Status {
    OK,
    NotFound,
    NotImplemented,
}

use self::Status::*;

impl Status {
    pub fn message(&self) -> &'static str {
        match self {
            OK => "200 OK",
            NotFound => "404 Not Found",
            NotImplemented => "501 Not Implemented",
        }
    }
}

pub struct ResponseHeader {
    status: Status,
    headers: HashMap<String, String>,
}

impl ResponseHeader {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            headers: HashMap::new(),
        }
    }

    pub fn attach_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn write_to<W>(&self, mut writer: W) -> io::Result<()>
    where
        W: Write,
    {
        write!(writer, "HTTP/1.1 {}\r\n", self.status.message())?;
        for (key, value) in self.headers.iter() {
            write!(writer, "{}: {}\r\n", key, value)?;
        }
        write!(writer, "\r\n")?;
        Ok(())
    }
}
