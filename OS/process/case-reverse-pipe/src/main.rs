use nix::{
    sys::wait::wait,
    unistd::{close, fork, pipe, read, write},
};
use std::{env, process};

fn main() {
    let mut args = env::args();
    args.next();
    let input = args.next().filter(|arg| arg.is_ascii()).unwrap_or_else(|| {
        eprintln!("Usage: EXEC ASCII_STRING");
        process::exit(1);
    });

    if let Err(err) = case_reverse_pipe(&input) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

const BUF_LEN: usize = 0x100;

fn case_reverse_pipe(input: &str) -> nix::Result<()> {
    let (fwd_read, fwd_write) = pipe()?;
    let (ret_read, ret_write) = pipe()?;

    if fork()?.is_parent() {
        close(fwd_read)?;
        close(ret_write)?;

        // should handle the case where only part of the buf is written to the pipe
        write(fwd_write, input.as_bytes())?;

        wait()?;

        let mut buf = [0; BUF_LEN];
        let len = read(ret_read, &mut buf)?;
        println!("{}", std::str::from_utf8(&buf[..len]).unwrap());
    } else {
        close(fwd_write)?;
        close(ret_read)?;

        let mut buf = [0; BUF_LEN];
        let len = read(fwd_read, &mut buf)?;
        for byte in buf.iter_mut().take(len) {
            if byte.is_ascii_lowercase() {
                *byte = byte.to_ascii_uppercase();
            } else {
                *byte = byte.to_ascii_lowercase();
            }
        }
        write(ret_write, &buf[..len])?;
    }

    Ok(())
}
