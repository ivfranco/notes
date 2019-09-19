use echo::{parse_port, read_length_padded, str_err_to_io, write_length_padded, BUF_SIZE};
use std::{
    io,
    net::{Ipv4Addr, TcpListener, TcpStream},
};

fn main() -> io::Result<()> {
    let (_, server) = parse_port("echo server");
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, server))?;
    let (stream, _) = listener.accept()?;
    event_loop(stream)
}

fn event_loop(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; BUF_SIZE];
    let len = read_length_padded(&mut stream, &mut buf)?;
    let line = std::str::from_utf8(&buf[..len]).map_err(str_err_to_io)?;
    write_length_padded(&mut stream, line.to_uppercase().as_bytes())?;
    Ok(())
}
