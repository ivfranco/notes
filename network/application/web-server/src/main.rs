use std::{env, io, process};
use web_server::server;

fn main() -> io::Result<()> {
    env_logger::init();

    let port = parse_port();
    server(port)
}

fn parse_port() -> u16 {
    let mut args = env::args();
    // skip executable name
    args.next();
    args.next()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXEC PORT");
            process::exit(1);
        })
}
