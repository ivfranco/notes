use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead, Result, Write};
use std::env;

pub fn echo_server() -> Result<()> {
    let mut args = env::args();
    let bin = args.next().expect("bin path missing");
    let port: u16 = args.next()
        .expect(&format!("Usage: {} PORT", bin))
        .parse()
        .expect("Invalid port number");

    let listener = TcpListener::bind(("localhost", port))?;
    for conn in listener.incoming() {
        let stream = conn?;
        // by move semantics, stream is automatically copied to the context of the newly spawned thread
        // thereby avoided the possible data racing in the C version
        thread::spawn(move || {
            if let Err(e) = echo(&stream) {
                eprintln!("{}", e);
            }
        });
    }
    Ok(())
}

fn echo(stream: &TcpStream) -> Result<()> {
    const NEWLINE: [u8; 1] = ['\n' as u8];
    let reader = io::BufReader::new(stream);
    let mut writer = io::BufWriter::new(stream);

    for line in reader.lines() {
        writer.write((line?).as_bytes())?;
        writer.write(&NEWLINE)?;
        writer.flush()?;
    }
    // threads in Rust are detached by default if the main thread didn't call the join method
    // memory resources occupied by peer threads will be released on their return
    Ok(())
}
