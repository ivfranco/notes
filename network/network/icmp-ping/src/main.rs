use icmp_ping::client::ping;
use std::{env, io::Result, process};

fn main() -> Result<()> {
    env_logger::init();

    let mut args = env::args();
    args.next();
    let ip = args.next().unwrap_or_else(|| {
        eprintln!("Usage: EXEC IP");
        process::exit(1);
    });

    ping(&ip)
}
