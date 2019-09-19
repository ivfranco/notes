use echo::{parse_port, read_length_padded, str_err_to_io, write_length_padded, BUF_SIZE};
use std::{
    io::{self, stdin},
    net::{Ipv4Addr, TcpStream},
};

fn main() -> io::Result<()> {
    let (_, server) = parse_port("echo client");
    let stream = TcpStream::connect((Ipv4Addr::LOCALHOST, server))?;
    event_loop(stream)
}

fn event_loop(mut stream: TcpStream) -> io::Result<()> {
    let mut line = String::new();
    let mut buf = [0; BUF_SIZE];

    stdin().read_line(&mut line)?;
    // TCP stream is not segmented as packets as UDP
    // send length of message
    write_length_padded(&mut stream, line.as_bytes())?;

    // receive length of message
    let len = read_length_padded(&mut stream, &mut buf)?;
    print!(
        "{}",
        std::str::from_utf8(&buf[..len]).map_err(str_err_to_io)?,
    );

    Ok(())
}
