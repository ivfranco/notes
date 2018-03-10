extern crate nix;

mod robust;

use std::os::unix::io::RawFd;
use std::env;
use robust::*;
use nix::sys::stat::{fstat, Mode};
use nix::fcntl::{open, OFlag};
use nix::unistd::dup2;
use nix::libc;

fn main() {
    problem_10_10();
}

fn cpfile(path: Option<String>) -> nix::Result<()> {
    const MAXBUF: usize = 512;
    let mut buf = [0; MAXBUF];
    let mut cnt;

    if let Some(file) = path {
        let fd = open(file.as_str(), OFlag::O_RDONLY, Mode::S_IRUSR)?;
        dup2(fd, libc::STDIN_FILENO)?;
    }

    let mut rio = RioFd::new(libc::STDIN_FILENO);
    while {
        cnt = rio_readlineb(&mut rio, &mut buf, MAXBUF)?;
        cnt != 0
    } {
        rio_writen(libc::STDOUT_FILENO, &buf, cnt)?;
    }

    Ok(())
}

fn problem_10_7() {
    if let Err(e) = cpfile(None) {
        eprintln!("{}", e);
    }
}

fn fstatcheck(fd: RawFd) -> nix::Result<()> {
    let stat = fstat(fd)?;
    let file_type = if stat.st_mode & libc::S_IFREG != 0 {
        "regular"
    } else if stat.st_mode & libc::S_IFDIR != 0 {
        "directory"
    } else {
        "other"
    };
    let readok = if stat.st_mode & libc::S_IRUSR != 0 {
        "yes"
    } else {
        "no"
    };

    println!("type: {}, read: {}", file_type, readok);

    Ok(())
}

fn problem_10_8() {
    let fd = env::args().nth(1).unwrap().parse().unwrap();
    if let Err(e) = fstatcheck(fd) {
        eprintln!("{}", e);
    }
}

fn problem_10_10() {
    let path = env::args().nth(1);
    if let Err(e) = cpfile(path) {
        eprintln!("{}", e);
    }
}
