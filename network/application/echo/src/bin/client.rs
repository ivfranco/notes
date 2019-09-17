use echo::{parse_port, BUF_SIZE};
use std::{
    io::{self, ErrorKind},
    net::UdpSocket,
};

fn main() -> io::Result<()> {
    let (client, server) = parse_port("echo client");
    let socket = UdpSocket::bind(("localhost", client))?;
    socket.connect(("localhost", server))?;
    event_loop(socket)
}

fn event_loop(socket: UdpSocket) -> io::Result<()> {
    let mut line = String::new();
    let mut buf = [0; BUF_SIZE];

    line.clear();
    io::stdin().read_line(&mut line)?;
    socket.send(line.as_bytes())?;
    let len = socket.recv(&mut buf)?;
    // known bug: valid uppercase text may be longer than original text
    // and be truncated to invalid utf-8
    let received = std::str::from_utf8(&buf[..len])
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;
    print!("{}", received);

    Ok(())
}
