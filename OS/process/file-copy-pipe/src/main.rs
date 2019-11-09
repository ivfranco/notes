use nix::{
    fcntl::{open, OFlag},
    sys::{
        stat::{fstat, Mode},
        wait::wait,
    },
    unistd::{close, fork, pipe, read, write},
};
use std::{env, os::unix::io::RawFd, process};

fn main() {
    let (source_path, target_path) = parse_arguments().unwrap_or_else(|| {
        eprintln!("Usage: EXEC SOURCE_PATH TARGET_PATH");
        process::exit(1);
    });

    if let Err(err) = file_copy_pipe(&source_path, &target_path) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn parse_arguments() -> Option<(String, String)> {
    let mut args = env::args();
    args.next();

    let source_path = args.next()?;
    let target_path = args.next()?;

    Some((source_path, target_path))
}

fn file_copy_pipe(source_path: &str, target_path: &str) -> nix::Result<()> {
    let (pipe_read, pipe_write) = pipe()?;

    let source_file = open(source_path, OFlag::O_RDONLY, Mode::empty())?;
    let source_len = fstat(source_file)?.st_size as usize;
    file_copy(source_file, pipe_write, source_len)?;
    close(pipe_write)?;

    if fork()?.is_child() {
        let target_file = open(target_path, OFlag::O_CREAT | OFlag::O_WRONLY, Mode::all())?;
        file_copy(pipe_read, target_file, source_len)?;
    } else {
        wait()?;
    }

    Ok(())
}

const BUF_LEN: usize = 0x1000;

fn file_copy(source: RawFd, target: RawFd, mut len: usize) -> nix::Result<()> {
    let mut buf = [0; BUF_LEN];

    while len > 0 {
        let read_len = read(source, &mut buf)?;
        write_all(target, &buf[..read_len])?;
        len -= read_len;
    }

    Ok(())
}

fn write_all(fd: RawFd, mut buf: &[u8]) -> nix::Result<()> {
    while !buf.is_empty() {
        let write_len = write(fd, buf)?;
        buf = &buf[write_len..];
    }
    Ok(())
}
