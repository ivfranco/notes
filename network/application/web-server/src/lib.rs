mod http_response;

use http_response::{ResponseHeader, Status};
use log::{error, info};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind},
    net::{Ipv4Addr, TcpListener, TcpStream},
    path::PathBuf,
};

const FILE_ROOT: &str = "static";

pub fn server(port: u16) -> io::Result<()> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port))?;
    info!("Tcp listener bound to {}", port);
    for result in listener.incoming() {
        let stream = result?;
        info!("Accepted connection from {:?}", stream.peer_addr());
        if let Err(err) = handle(stream) {
            error!("{:?}", err);
        }
    }
    Ok(())
}

fn handle(mut stream: TcpStream) -> io::Result<()> {
    let path = request_path(&mut stream)?;
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Err(file_not_found(&mut stream));
            } else {
                // unexpected error e.g. no permission, propagate to caller with no response
                return Err(err);
            }
        }
    };
    let file_len = file.metadata()?.len();

    ResponseHeader::new(Status::OK)
        .attach_header("Content-Length", &format!("{}", file_len))
        .attach_header("Connection", "close")
        .attach_header("Content-Type", "text/plain")
        .write_to(&mut stream)?;

    io::copy(&mut file, &mut stream)?;
    info!("File sent: {:?}, {} bytes", path, file_len);

    Ok(())
}

// only part of the first line is parsed
// all headers, request body, even the http version is skipped
fn request_path(mut stream: &mut TcpStream) -> io::Result<PathBuf> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // remove newline character
    line.pop();
    info!("Method line read: {}", line);
    stream = reader.into_inner();

    if !line.starts_with("GET") {
        return Err(method_not_implemented(stream));
    }

    line = line.split_off("GET ".len());
    line.split_off(line.find(' ').expect("Space between url and HTTP version"));
    info!("Url extracted: {}", line);

    let mut path = PathBuf::from(".");
    // file under ./static/ is served as text files
    path.push(FILE_ROOT);
    // the first slash in the url is skipped, otherwise the url is treated as absolute path, replaces current path
    // https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.push
    path.push(&line[1..]);
    Ok(path)
}

fn error_response(stream: &mut TcpStream, status: Status) -> io::Error {
    ResponseHeader::new(status)
        .attach_header("Connection", "close")
        .write_to(stream)
        .err()
        .unwrap_or_else(|| io::Error::new(ErrorKind::Other, status.message()))
}

fn method_not_implemented(stream: &mut TcpStream) -> io::Error {
    error_response(stream, Status::NotImplemented)
}

fn file_not_found(stream: &mut TcpStream) -> io::Error {
    error_response(stream, Status::NotFound)
}
