use crate::{Error, Result};
use httparse::{Header, Request, Response, EMPTY_HEADER};
use log::{debug, error};
use std::{collections::HashMap, io::BufRead, mem};

struct HTTPHeaders {
    fields: HashMap<String, String>,
}

impl HTTPHeaders {
    fn new(buf: &[Header]) -> Result<Self> {
        let fields = buf
            .iter()
            .map(|header| {
                let name = header.name.to_string();
                let value =
                    String::from_utf8(header.value.to_vec()).map_err(|_| Error::MalformedHTTP)?;
                Ok((name, value))
            })
            .collect::<Result<_>>()?;

        Ok(Self { fields })
    }

    fn chunked(&self) -> bool {
        self.fields
            .get("Transfer-Encoding")
            .map_or(false, |value| value.starts_with("chunked"))
    }

    // return 0 when Content-Length: header is missing
    fn content_length(&self) -> Option<usize> {
        self.fields
            .get("Content-Length")
            .and_then(|value| value.parse().ok())
    }

    fn read_body<R>(&self, reader: &mut R) -> Result<Vec<u8>>
    where
        R: BufRead,
    {
        if self.chunked() {
            let mut buf = read_until_empty_line(reader)?;
            let mut tail = [0; 256];
            let len = reader.read(&mut tail)?;
            if len > 0 {
                debug!("FIXME: {} bytes read after supposed EOF", len);
                buf.extend_from_slice(&tail[..len]);
            }
            Ok(buf)
        } else if let Some(len) = self.content_length() {
            let mut buf = vec![0; len];
            reader.read_exact(&mut buf)?;
            Ok(buf)
        } else {
            Err(Error::BodyNotPresent)
        }
    }
}

impl std::fmt::Display for HTTPHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (name, value) in &self.fields {
            writeln!(f, "{}: {}\r", name, value)?;
        }
        writeln!(f, "\r")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    GET,
    POST,
}

const MAX_HEADER: usize = 32;
const CRLF: &[u8] = b"\r\n";
const LF: u8 = b'\n';

pub struct HTTPRequest {
    method: Method,
    host: String,
    path: String,
    headers: HTTPHeaders,
}

impl HTTPRequest {
    pub fn from_reader<R>(reader: &mut R) -> Result<Self>
    where
        R: BufRead,
    {
        let buf = read_until_empty_line(reader)?;
        Self::from_buf(&buf)
    }

    fn from_buf(buf: &[u8]) -> Result<Self> {
        let mut headers_buf = [EMPTY_HEADER; MAX_HEADER];
        let mut parser = Request::new(&mut headers_buf);
        parser.parse(&buf)?;

        let method = match parser.method {
            Some("GET") => Ok(Method::GET),
            Some("POST") => Ok(Method::POST),
            Some(method) => {
                error!("Unexpected HTTP method {:?}", method);
                Err(Error::MethodNotImplemented)
            }
            _ => {
                error!("No HTTP method in request");
                Err(Error::MalformedHTTP)
            }
        }?;

        let url = if let Some(url) = parser.path {
            url
        } else {
            return Err(Error::MalformedHTTP);
        };
        let (host, path) = split_url(url)?;

        let headers = HTTPHeaders::new(parser.headers)?;

        Ok(Self {
            method,
            host: host.to_string(),
            path: path.to_string(),
            headers,
        })
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn method(&self) -> Method {
        self.method
    }

    // https://tools.ietf.org/html/rfc7230#section-3.3.3
    pub fn read_body<R>(&self, reader: &mut R) -> Result<Vec<u8>>
    where
        R: BufRead,
    {
        self.headers.read_body(reader)
    }
}

impl std::fmt::Display for HTTPRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{:?} {} HTTP/1.1\r", self.method, self.path)?;
        // headers, end with an empty line
        write!(f, "{}", self.headers)?;

        Ok(())
    }
}

impl std::fmt::Debug for HTTPRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

fn split_url(mut url: &str) -> Result<(String, String)> {
    if url.starts_with("http://") {
        // configured as the http proxy in the browser
        url = &url["http://".len()..];
    } else if url.starts_with('/') {
        // accessed from url as localhost:port/url
        url = &url[1..];
    }

    let slash = url.find('/').unwrap_or_else(|| url.len());
    let host = url[..slash].to_string();
    let path = if slash == url.len() {
        "/".to_string()
    } else {
        url[slash..].to_string()
    };
    Ok((host, path))
}

fn read_until_empty_line<R>(reader: &mut R) -> Result<Vec<u8>>
where
    R: BufRead,
{
    let mut buf = vec![];
    loop {
        let len = reader.read_until(LF, &mut buf)?;
        // an empty line, end of header
        if len == 2 && buf.ends_with(CRLF) {
            break;
        }
    }
    Ok(buf)
}

pub struct HTTPResponse {
    code: u16,
    reason: String,
    headers: HTTPHeaders,
}

impl HTTPResponse {
    pub fn from_reader<R>(reader: &mut R) -> Result<Self>
    where
        R: BufRead,
    {
        let buf = read_until_empty_line(reader)?;
        Self::from_buf(&buf)
    }

    fn from_buf(buf: &[u8]) -> Result<Self> {
        let mut headers_buf = [EMPTY_HEADER; MAX_HEADER];
        let mut parser = Response::new(&mut headers_buf);
        parser.parse(buf)?;

        let code = parser.code.ok_or_else(|| {
            error!("No status code in response");
            Error::MalformedHTTP
        })?;

        let reason = parser
            .reason
            .ok_or_else(|| {
                error!("No reason in response");
                Error::MalformedHTTP
            })?
            .to_string();

        let headers = HTTPHeaders::new(parser.headers)?;

        Ok(Self {
            code,
            reason,
            headers,
        })
    }

    // https://tools.ietf.org/html/rfc7230#section-3.3.3
    pub fn read_body<R>(&self, reader: &mut R) -> Result<Vec<u8>>
    where
        R: BufRead,
    {
        self.headers.read_body(reader)
    }
}

impl std::fmt::Display for HTTPResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "HTTP/1.1 {} {}\r", self.code, self.reason)?;
        write!(f, "{}", self.headers)?;

        Ok(())
    }
}

impl std::fmt::Debug for HTTPResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

pub enum Status {
    BadRequest,
    NotImplemented,
}

use Status::*;

impl Status {
    fn code(&self) -> u16 {
        match self {
            BadRequest => 400,
            NotImplemented => 501,
        }
    }

    fn reason(&self) -> &'static str {
        match self {
            BadRequest => "Bad Request",
            NotImplemented => "Not Implemented",
        }
    }
}

pub struct HTTPResponseBuilder {
    status: Status,
    fields: HashMap<String, String>,
}

impl HTTPResponseBuilder {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            fields: HashMap::new(),
        }
    }

    pub fn attach_header(&mut self, name: &str, value: &str) -> &mut Self {
        self.fields.insert(name.to_string(), value.to_string());
        self
    }

    pub fn build(&mut self) -> HTTPResponse {
        let fields = mem::replace(&mut self.fields, HashMap::new());
        HTTPResponse {
            code: self.status.code(),
            reason: self.status.reason().to_string(),
            headers: HTTPHeaders { fields },
        }
    }
}
