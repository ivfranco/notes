use std::thread;
use std::process;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver};
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead, Write};
use std::env;

const NTHREADS: usize = 4;
const SBUFSIZE: usize = 16;

pub fn echo_server() -> Result<(), io::Error> {
    let mut args = env::args();
    let bin = args.next().expect("bin path missing");
    let port: u16 = args.next()
        .expect(&format!("Usage: {} PORT", bin))
        .parse()
        .expect("Invalid port number");

    let (tx, rx) = mpsc::sync_channel(SBUFSIZE);
    let arc_rx = Arc::new(Mutex::new(rx));
    for _ in 0..NTHREADS {
        let rx_clone = arc_rx.clone();
        thread::spawn(|| {
            echo_thread(rx_clone);
        });
    }

    let listener: TcpListener = TcpListener::bind(("localhost", port))?;

    for conn in listener.incoming() {
        let stream = conn?;
        if let Err(e) = tx.send(stream) {
            eprintln!("Send error: {}", e);
            process::exit(1);
        }
    }

    Ok(())
}

fn echo_thread(arc_rx: Arc<Mutex<Receiver<TcpStream>>>) {
    let id = thread::current().id();
    let greeting = format!("Worker thread {:?} at service\n", id);
    loop {
        let rx = arc_rx.lock().expect("Arc error");
        let mut stream = rx.recv().expect("Receive error");
        if let Err(e) = stream.write(greeting.as_bytes()) {
            eprintln!("{}", e);
            process::exit(1);
        }
        drop(rx);
        if let Err(e) = echo(&stream) {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn echo(stream: &TcpStream) -> Result<(), io::Error> {
    const NEWLINE: [u8; 1] = ['\n' as u8];
    let reader = io::BufReader::new(stream);
    let mut writer = io::BufWriter::new(stream);

    for line in reader.lines() {
        writer.write((line?).as_bytes())?;
        writer.write(&NEWLINE)?;
        writer.flush()?;
    }
    Ok(())
}
