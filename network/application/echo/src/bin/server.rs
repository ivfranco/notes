use echo::{parse_port, BUF_SIZE};
use std::{
    io::{self, ErrorKind},
    net::UdpSocket,
};

fn main() -> io::Result<()> {
    let (_, server) = parse_port("echo server");
    let socket = UdpSocket::bind(("localhost", server))?;
    event_loop(socket)
}

fn event_loop(socket: UdpSocket) -> io::Result<()> {
    let mut buf = [0; BUF_SIZE];

    // known bug: valid string may be truncated to invalid utf-8
    let (len, client) = socket.recv_from(&mut buf)?;
    let line = std::str::from_utf8(&buf[..len])
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;
    socket.send_to(line.to_uppercase().as_bytes(), client)?;

    Ok(())
}
