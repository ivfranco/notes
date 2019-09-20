use crate::{resolver::DNSResolver, Error, Result};
use httparse::{Request, EMPTY_HEADER};
use log::{debug, error};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
    thread,
};

pub fn spawn_server(port: u16) -> Result<()> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port))?;
    debug!("TCP listener established at {}", port);
    for result in listener.incoming() {
        let client = result?;
        debug!("Accepted client from {:?}", client.peer_addr());
        thread::spawn(|| relay_and_report(client));
    }

    Ok(())
}

fn relay_and_report(client: TcpStream) {
    if let Err(err) = relay(client) {
        error!("{:?}", err);
    }
}

fn relay(mut client: TcpStream) -> Result<()> {
    let request = HTTPRequest::from_reader(BufReader::new(&mut client))?;
    debug!("Received request from client:\n{:?}", request);
    let resolver = DNSResolver::spawn()?;
    debug!("Resolving domain name {}", request.host);
    let ip = resolver.lookup(&request.host)?;
    debug!("Domain name {} resolved to {:?}", request.host, ip);
    let mut server = TcpStream::connect((ip, HTTP_PORT))?;
    debug!("Connected to server at {:?}", ip);
    request.write_to(&mut server)?;
    debug!("Request relayed to server, waiting for response");

    // FIXME: actually parse and relay HTTP responses
    std::io::copy(&mut server, &mut client)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Method {
    GET,
    POST,
}

const HTTP_PORT: u16 = 80;
const MAX_HEADER: usize = 32;
const CRLF: &[u8] = b"\r\n";
const LF: u8 = b'\n';

struct HTTPRequest {
    method: Method,
    host: String,
    path: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HTTPRequest {
    fn from_reader<R>(mut reader: R) -> Result<Self>
    where
        R: BufRead,
    {
        let mut headers_buf = [EMPTY_HEADER; MAX_HEADER];
        let mut parser = Request::new(&mut headers_buf);
        let mut buf = vec![];
        loop {
            let len = reader.read_until(LF, &mut buf)?;
            // an empty line, end of header
            if len == 2 && buf.ends_with(CRLF) {
                break;
            }
        }
        parser.parse(&buf)?;

        let method = match parser.method {
            Some("GET") => Method::GET,
            Some("POST") => Method::POST,
            _ => {
                error!("Unexpected HTTP method {:?}", parser.method);
                return Err(Error::MethodNotImplemented);
            }
        };

        let url = if let Some(url) = parser.path {
            url
        } else {
            return Err(Error::MalformedHTTP);
        };
        let (host, path) = split_url(url)?;

        let headers: HashMap<String, String> = parser
            .headers
            .iter()
            .try_fold::<HashMap<_, _>, _, Result<_>>(HashMap::new(), |mut headers, header| {
                let name = header.name.to_string();
                let value =
                    String::from_utf8(header.value.to_vec()).map_err(|_| Error::MalformedHTTP)?;
                headers.insert(name, value);
                Ok(headers)
            })?;

        let mut body = vec![];
        if method == Method::POST {
            let size = headers
                .get("Content-Length")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(0);
            body.resize_with(size, Default::default);
            reader.read_exact(&mut body)?;
        }

        Ok(Self {
            method,
            host: host.to_string(),
            path: path.to_string(),
            headers,
            body,
        })
    }

    fn write_headers<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        // start line
        writeln!(writer, "{:?} {} HTTP/1.1\r", self.method, self.path)?;
        // headers, end with an empty line
        for (name, value) in &self.headers {
            writeln!(writer, "{}: {}\r", name, value)?;
        }
        writeln!(writer, "\r")?;

        Ok(())
    }

    fn write_to<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.write_headers(writer)?;
        writer.write_all(&self.body)?;

        Ok(())
    }
}

impl std::fmt::Debug for HTTPRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = vec![];
        self.write_headers(&mut buf).map_err(|_| std::fmt::Error)?;
        write!(
            f,
            "{}",
            String::from_utf8(buf).expect("Headers are valid strings")
        )?;
        writeln!(f, "Body length: {}", self.body.len())?;

        Ok(())
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
