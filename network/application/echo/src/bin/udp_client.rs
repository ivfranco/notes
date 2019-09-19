use echo::{parse_port, str_err_to_io, BUF_SIZE};
use std::{
    io,
    net::{Ipv4Addr, UdpSocket},
};

fn main() -> io::Result<()> {
    // there's no way to initialize a UdpSocket in Rust without providing a port number
    // otherwise client port number is unnecessary
    let (client, server) = parse_port("echo client");
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, client))?;
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
    let received = std::str::from_utf8(&buf[..len]).map_err(str_err_to_io)?;
    print!("{}", received);

    Ok(())
}
