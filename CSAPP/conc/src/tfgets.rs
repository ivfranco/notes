use std::io::{self, BufRead};
use std::sync::mpsc;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub fn tfgets() -> Result<String, Box<Error>> {
    let (tx, rx) = mpsc::channel();
    let timeout = Duration::from_secs(5);

    thread::spawn(move || {
        let mut buf = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut buf).expect("Stdin lock error");
        tx.send(buf).expect("Channel send error");
    });

    rx.recv_timeout(timeout).map_err(|e| e.into())
}
