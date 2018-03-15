extern crate httparse;

use std::thread;
use std::process::{self, Command};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver};
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::error::Error;
use std::env;
use std::fs;

const NTHREADS: usize = 4;
const SBUFSIZE: usize = 16;

pub fn tiny_server() -> Result<(), io::Error> {
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
            if let Err(e) = tiny_thread(rx_clone) {
                eprintln!("{}", e);
            }
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

fn tiny_thread(arc_rx: Arc<Mutex<Receiver<TcpStream>>>) -> Result<(), Box<Error>> {
    let id = thread::current().id();
    loop {
        let rx = arc_rx.lock().expect("Arc error");
        let stream = rx.recv().expect("Receive error");
        println!("Thread {:?} at service", id);
        doit(&stream)?;
        drop(rx);
    }
}

fn doit(stream: &TcpStream) -> Result<(), Box<Error>> {
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream);

    let (method, uri) = parse_request(&mut reader)?;

    if method != "GET" {
        return client_error(
            &mut writer,
            &method,
            "501",
            "Not implemented",
            "Tiny does not implement this method",
        ).map_err(|e| e.into());
    }

    let (filename, args) = parse_uri(&uri);
    println!("{}, {}", filename, args);
    if let Err(e) = fs::metadata(&filename) {
        let _ = client_error(
            &mut writer,
            &filename,
            "404",
            "Not found",
            "Tiny couldn't find this file",
        );
        return Err(e.into());
    }
    // dynamic
    if uri.starts_with("/cgi-bin") {
        if let Err(e) = serve_dynamic(&mut writer, &filename, &args) {
            let _ = client_error(
                &mut writer,
                &filename,
                "403",
                "Forbidden",
                "Tiny couldn't run the CGI program",
            );
            return Err(e.into());
        }
    // static
    } else {
        if let Err(e) = serve_static(&mut writer, &filename) {
            let _ = client_error(
                &mut writer,
                &filename,
                "403",
                "Forbidden",
                "Tiny couldn't read the file",
            );
            return Err(e.into());
        }
    }
    Ok(())
}

fn invalid_input<E>(e: E) -> io::Error
where
    E: Into<Box<Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::InvalidInput, e)
}

fn client_error(
    stream: &mut BufWriter<&TcpStream>,
    cause: &str,
    errnum: &str,
    shortmsg: &str,
    longmsg: &str,
) -> Result<(), io::Error> {
    let mut body = vec![];
    write!(
        body,
        "<html>
            <title>Tiny error</title>
            <body bgcolor='#ffffff'>
                {}: {}
                <p>
                    {}: {}
                    <hr>
                    <em>The Tiny Web server</em>
                </p>
            </body>
        </html>",
        errnum, shortmsg, longmsg, cause
    )?;

    write!(
        stream,
        "HTTP/1.0 {} {}\r\n\
         Content-type: text/html\r\n\
         Content-length: {}\r\n\r\n",
        errnum,
        shortmsg,
        body.len()
    )?;
    stream.write_all(&body)?;
    stream.flush()?;
    Err(invalid_input(longmsg))
}

fn parse_request(stream: &mut BufReader<&TcpStream>) -> Result<(String, String), io::Error> {
    let mut buf: Vec<u8> = vec![];
    let mut line = String::new();
    while line != "\r\n" {
        buf.extend_from_slice(line.as_bytes());
        line.clear();
        stream.read_line(&mut line)?;
    }
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    req.parse(&buf).map_err(|e| invalid_input(e))?;
    let method = req.method.ok_or(invalid_input("method missing"))?;
    let uri = req.path.ok_or(invalid_input("uri missing"))?;

    Ok((String::from(method), String::from(uri)))
}

fn parse_uri(uri: &str) -> (String, String) {
    let (path, query) = if let Some(idx) = uri.find('?') {
        (&uri[..idx], &uri[idx + 1..])
    } else {
        (uri, "")
    };
    let filename = if path == "/" {
        String::from("./home.html")
    } else {
        format!(".{}", path)
    };
    let args = String::from(query);

    (filename, args)
}

fn get_filetype(filename: &str) -> String {
    let filetype = if filename.ends_with(".html") {
        "text/html"
    } else if filename.ends_with(".gif") {
        "image/gif"
    } else if filename.ends_with("jpg") {
        "image/jpg"
    } else {
        "text/plain"
    };

    String::from(filetype)
}

fn serve_static(stream: &mut BufWriter<&TcpStream>, filename: &str) -> Result<(), io::Error> {
    let meta = fs::metadata(filename)?;
    let filetype = get_filetype(filename);
    write!(
        stream,
        "HTTP/1.0 200 OK\r\n\
         Server: Tiny Web Server\r\n\
         Content-length: {}\r\n\
         Content-type: {}\r\n\r\n",
        meta.len(),
        filetype
    )?;

    let mut file = fs::File::open(filename)?;
    io::copy(&mut file, stream)?;
    drop(file);

    Ok(())
}

fn serve_dynamic(
    stream: &mut BufWriter<&TcpStream>,
    filename: &str,
    args: &str,
) -> Result<(), io::Error> {
    write!(
        stream,
        "HTTP/1.0 200 OK\r\n\
         Server: Tiny Web Server\r\n\r\n",
    )?;

    env::set_var("QUERY_STRING", args);
    let output = Command::new(filename).arg(args).output()?;
    if output.status.success() {
        stream.write_all(&output.stdout)?;
    } else {
        let err = String::from_utf8(output.stderr).expect("stderr parsing error");
        return Err(invalid_input(err));
    }

    Ok(())
}
