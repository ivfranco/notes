use clap::{App, Arg};
use std::process;

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
