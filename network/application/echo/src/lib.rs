use clap::{App, Arg};
use std::{
    io::{self, ErrorKind, Read, Write},
    mem::size_of,
    process,
    str::Utf8Error,
};

/// maximum length of a line
pub const BUF_SIZE: usize = 0x100;

/// parse arguments as a (client, server) pair of port numbers, exit process if not present
pub fn parse_port(binary: &str) -> (u16, u16) {
    let matches = App::new(binary)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("client")
                .takes_value(true)
                .help("port number of the echo client on localhost"),
        )
        .arg(
            Arg::with_name("server")
                .takes_value(true)
                .help("port number of the echo server on localhost"),
        )
        .get_matches();

    let client = matches
        .value_of("client")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or_else(|| {
            eprintln!("Error: no client port number provided");
            process::exit(1);
        });
    let server = matches
        .value_of("server")
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or_else(|| {
            eprintln!("Error: no server port number provided");
            process::exit(1);
        });

    (client, server)
}

pub fn write_length_padded<W>(mut writer: W, buf: &[u8]) -> io::Result<()>
where
    W: Write,
{
    writer.write(&buf.len().to_be_bytes())?;
    writer.write_all(buf)?;
    writer.flush()
}

pub fn read_length_padded<R>(mut reader: R, buf: &mut [u8]) -> io::Result<usize>
where
    R: Read,
{
    let mut pad = [0; size_of::<usize>()];
    reader.read_exact(&mut pad)?;
    let len = usize::from_be_bytes(pad);
    reader.read_exact(&mut buf[..len])?;
    Ok(len)
}

pub fn str_err_to_io(err: Utf8Error) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, err)
}
