use crate::{
    http::{HTTPRequest, HTTPResponse, HTTPResponseBuilder, Method, Status},
    resolver::DNSResolver,
    Error, Result,
};
use log::{debug, error};
use std::{
    io::{BufReader, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
    thread,
};

const HTTP_PORT: u16 = 80;

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

fn relay_and_report(mut client: TcpStream) {
    if let Err(err) = relay(&mut client) {
        error!("{:?}", err);
    }
}

fn relay(client: &mut TcpStream) -> Result<()> {
    let mut server = match relay_request(client) {
        Ok(server) => server,
        Err(err) => {
            let handled = match err {
                Error::BodyNotPresent | Error::MalformedHTTP => {
                    error_response(Status::BadRequest, client)
                }
                Error::MethodNotImplemented => error_response(Status::NotImplemented, client),
                _ => Err(err),
            };
            return handled;
        }
    };
    relay_response(client, &mut server)?;
    Ok(())
}

fn relay_request(client: &mut TcpStream) -> Result<TcpStream> {
    let client_peer = client.peer_addr();
    let mut client_reader = BufReader::new(client);
    let request = HTTPRequest::from_reader(&mut client_reader)?;
    debug!(
        "Received request from client {:?}:\n{:?}",
        client_peer, request
    );
    let resolver = DNSResolver::spawn()?;
    debug!("Resolving domain name {}", request.host());
    let ip = resolver.lookup(request.host())?;
    debug!("Domain name {} resolved to {:?}", request.host(), ip);
    let mut server = TcpStream::connect((ip, HTTP_PORT))?;
    debug!("Connected to server at {:?}", ip);
    write!(server, "{}", request)?;
    if request.method() == Method::POST {
        debug!("POST request, sending body");
        let body = request.read_body(&mut client_reader)?;
        server.write_all(&body)?;
        debug!("Sent {} bytes", body.len());
    }
    debug!("Request relayed to server, waiting for response");
    Ok(server)
}

fn relay_response(client: &mut TcpStream, server: &mut TcpStream) -> Result<()> {
    let server_peer = server.peer_addr();
    let mut server_reader = BufReader::new(server);
    let response = HTTPResponse::from_reader(&mut server_reader)?;
    debug!(
        "Received response from server {:?}:\n{:?}",
        server_peer, response
    );
    write!(client, "{}", response)?;
    match response.read_body(&mut server_reader) {
        Ok(body) => {
            debug!("Sending response body");
            client.write_all(&body)?;
            debug!("{} bytes sent", body.len());
        }
        Err(Error::BodyNotPresent) => (),
        Err(err) => return Err(err),
    }
    debug!("Response relayed to client, closing connection");

    Ok(())
}

pub fn error_response(status: Status, client: &mut TcpStream) -> Result<()> {
    let response = HTTPResponseBuilder::new(status)
        .attach_header("Connection", "close")
        .build();

    write!(client, "{}", response)?;
    Ok(())
}