use std::{
    env,
    io::{self, BufRead, BufReader, Stdout, Write},
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    let mut args = env::args();
    // skip executable name
    args.next();

    let port = args
        .next()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXEC PORT");
            process::exit(1);
        });

    for result in TcpListener::bind(("localhost", port))?.incoming() {
        let stream = result?;
        // otherwise each stream will hold the stdout lock for 5+ seconds, blocking others
        stream.set_read_timeout(Some(Duration::from_millis(500)))?;
        let stdout = io::stdout();
        thread::spawn(move || printout(stream, stdout));
    }

    Ok(())
}

fn printout(stream: TcpStream, stdout: Stdout) -> io::Result<()> {
    // so lines of a single request will be printed together
    let mut lock = stdout.lock();
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        writeln!(&mut lock, "{}", line?)?;
        lock.flush()?;
    }
    Ok(())
}
