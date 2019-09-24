use log::debug;
use std::{env, process};
use web_proxy::{server::run_server, Result};

fn main() -> Result<()> {
    env_logger::init();

    debug!("Program started");
    let port = parse_port_or_exit();
    run_server(port)
}

fn parse_port_or_exit() -> u16 {
    let mut args = env::args();
    args.next();

    args.next()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXEC PORT");
            process::exit(1);
        })
}
