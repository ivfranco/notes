use std::{mem, process, str};

use anyhow::Context;
use nix::{
    fcntl::{open, OFlag},
    sys::stat::{stat, Mode},
    unistd::{close, lseek, read, Whence},
};

fn main() {
    let args = argh::from_env();
    if let Err(e) = exec(args) {
        eprintln!("{:#}", e);
        process::exit(1);
    }
}

fn exec(args: TailArgs) -> anyhow::Result<()> {
    let stat = stat(&*args.file).context("stat() failed")?;
    let mut buf = vec![0; stat.st_blksize as usize].into_boxed_slice();
    let mut lines: Vec<Vec<u8>> = vec![];
    let mut incomplete = Vec::new();

    let fd = open(&*args.file, OFlag::O_RDONLY, Mode::empty()).context("open() failed")?;
    let mut offset = lseek(fd, 0, Whence::SeekEnd).context("initial lseek() failed")?;

    while lines.len() < args.lines && offset > 0 {
        // lseek() beyond the start is an error
        let seek_to = (offset - i64::from(stat.st_blksize)).max(0);
        offset = lseek(fd, seek_to, Whence::SeekSet).context("loop lseek() failed")?;
        let rd = read(fd, &mut buf).context("read() failed")?;

        let mut first = true;
        for line in buf[..rd].rsplit(|&c| c == b'\n') {
            if !first {
                lines.push(mem::replace(&mut incomplete, Vec::new()));
            }
            first = false;
            incomplete.extend_from_slice(line);
        }
    }

    lines.push(incomplete);

    for line in lines.iter().take(args.lines).rev() {
        let s = str::from_utf8(&line).context("line is not valid utf8")?;
        println!("{}", s);
    }

    close(fd).context("close() failed")?;

    Ok(())
}

#[derive(argh::FromArgs)]
/// Display the last part of a file.
struct TailArgs {
    #[argh(option, short = 'n', default = "DEFAULT_LINES")]
    /// number of lines displayed
    lines: usize,
    #[argh(positional)]
    file: String,
}

const DEFAULT_LINES: usize = 10;
