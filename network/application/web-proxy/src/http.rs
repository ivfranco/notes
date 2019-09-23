//! Basic HTTP types.

use crate::{cache::CacheEntry, Error, GMTDateTime, Result};
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

    fn content_length(&self) -> Option<usize> {
        self.fields
            .get("Content-Length")
            .and_then(|value| value.parse().ok())
    }

    fn last_modified(&self) -> Option<GMTDateTime> {
        self.fields
            .get("Last-Modified")
            // treat bad formatted  date as non-present
            .and_then(|rfc2822| GMTDateTime::parse_from_rfc2822(rfc2822))
    }

    fn read_body<R>(&self, reader: &mut R) -> Result<Vec<u8>>
    where
        R: BufRead,
    {
        if self.chunked() {
            read_chunks(reader)
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

/// HTTP methods handled by the proxy server.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    /// Head-only GET Request
    GET,
    /// POST request, expecting Content-Length: or Transfer-Encoding: chunked
    POST,
    /// Head-only HEAD Request
    HEAD,
}

impl Method {
    fn parse(s: &str) -> Option<Self> {
        use Method::*;
        match s {
            "GET" => Some(GET),
            "POST" => Some(POST),
            "HEAD" => Some(HEAD),
            _ => None,
        }
    }
}

const MAX_HEADER: usize = 32;
const CRLF: &[u8] = b"\r\n";
const LF: u8 = b'\n';

/// An HTTP request without body.
pub struct HTTPRequest {
    method: Method,
    host: String,
    path: String,
    headers: HTTPHeaders,
}

impl HTTPRequest {
    /// Parse an http request from byte stream.
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
            Some(m) => Method::parse(m).ok_or_else(|| {
                error!("Unexpected HTTP method {}", m);
                Error::MethodNotImplemented
            }),
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

    /// Host domain name of the proxified request.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Method of the request.
    pub fn method(&self) -> Method {
        self.method
    }

    /// Convert this response to an cache entry if Last-Modified: is set.\
    /// Otherwise it's impossible to determine whether the cache is valid.
    pub fn to_cache_entry(&self, body: &[u8]) -> Option<CacheEntry> {
        let last_modified = self.headers.last_modified()?;
        let is_chunked = self.headers.chunked();
        Some(CacheEntry::new(last_modified, is_chunked, body.to_vec()))
    }

    /// Parse the body of the request defined by:\
    /// ]https://tools.ietf.org/html/rfc7230#section-3.3.3](https://tools.ietf.org/html/rfc7230#section-3.3.3)
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

/// A helper struct to build HTTP responses.
pub struct HTTPRequestBuilder {
    method: Method,
    fields: HashMap<String, String>,
}

impl HTTPRequestBuilder {
    /// Construct a new builder with the given method.
    pub fn new(method: Method) -> Self {
        Self {
            method,
            fields: HashMap::new(),
        }
    }

    /// Insert or replace a header field of the HTTP request.
    pub fn attach_header(&mut self, name: &str, value: &str) -> &mut Self {
        self.fields.insert(name.to_string(), value.to_string());
        self
    }

    /// Build an HTTP request from given host and path.
    pub fn build(&mut self, host: &str, path: &str) -> HTTPRequest {
        let fields = mem::replace(&mut self.fields, HashMap::new());
        HTTPRequest {
            method: self.method,
            host: host.to_string(),
            path: path.to_string(),
            headers: HTTPHeaders { fields },
        }
    }
}

/// An HTTP response without body.
pub struct HTTPResponse {
    code: u16,
    reason: String,
    headers: HTTPHeaders,
}

impl HTTPResponse {
    /// Parse an HTTP response from byte stream.
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

    /// Parse the body of the response defined by:\
    /// [https://tools.ietf.org/html/rfc7230#section-3.3.3](https://tools.ietf.org/html/rfc7230#section-3.3.3)
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

/// Status returned by the proxy server.
pub enum Status {
    /// 200 OK
    OK,
    /// 400 Bad Request
    BadRequest,
    /// 404 Not Found
    NotFound,
    /// 501 Not Implemented
    NotImplemented,
}

use Status::*;

impl Status {
    fn code(&self) -> u16 {
        match self {
            OK => 200,
            BadRequest => 400,
            NotFound => 404,
            NotImplemented => 501,
        }
    }

    fn reason(&self) -> &'static str {
        match self {
            OK => "OK",
            BadRequest => "Bad Request",
            NotFound => "Not Found",
            NotImplemented => "Not Implemented",
        }
    }
}

/// A helper struct to build HTTP responses.
pub struct HTTPResponseBuilder {
    status: Status,
    fields: HashMap<String, String>,
}

impl HTTPResponseBuilder {
    /// Construct a new builder with the given status.
    pub fn new(status: Status) -> Self {
        Self {
            status,
            fields: HashMap::new(),
        }
    }

    /// Insert or replace a header field of the HTTP response.
    pub fn attach_header(&mut self, name: &str, value: &str) -> &mut Self {
        self.fields.insert(name.to_string(), value.to_string());
        self
    }

    /// Build an HTTP response.
    pub fn build(&mut self) -> HTTPResponse {
        let fields = mem::replace(&mut self.fields, HashMap::new());
        HTTPResponse {
            code: self.status.code(),
            reason: self.status.reason().to_string(),
            headers: HTTPHeaders { fields },
        }
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

fn read_chunks<R>(reader: &mut R) -> Result<Vec<u8>>
where
    R: BufRead,
{
    debug!("Parsing chunked body");
    let mut buf = vec![];
    loop {
        if read_chunk(&mut buf, reader)? == 0 {
            break;
        }
    }
    debug!("Terminate chunk hit");
    // the test server appearently left out the last CRLF required by RFC7230
    buf.extend_from_slice(CRLF);
    // skip all trailers
    Ok(buf)
}

fn read_chunk<R>(buf: &mut Vec<u8>, reader: &mut R) -> Result<usize>
where
    R: BufRead,
{
    let begin = buf.len();
    let len = reader.read_until(LF, buf)?;
    let size = decode_size(&buf[begin..begin + len])?;
    debug!("Found chunk of size {} bytes", size);

    let chunk_start = begin + len;
    let chunk_size = size + CRLF.len();
    buf.resize(chunk_start + chunk_size, 0);
    reader.read_exact(&mut buf[chunk_start..chunk_start + chunk_size])?;

    Ok(size)
}

fn decode_size(line: &[u8]) -> Result<usize> {
    line.iter()
        .take_while(|byte| byte.is_ascii_hexdigit())
        .try_fold(0, |size, &byte| {
            let digit = hex_digit(byte)?;
            Ok((size << 4) + digit)
        })
}

fn hex_digit(byte: u8) -> Result<usize> {
    let digit = match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(Error::MalformedHTTP),
    }?;

    Ok(digit as usize)
}
