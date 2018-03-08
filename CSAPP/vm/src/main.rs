#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate libc;

mod mm;
mod wrappers;

use libc::*;
use std::ffi::CString;
use std::env;
use std::mem;
use std::ptr;

use wrappers::*;

fn main() {
    problem_9_5();
}
unsafe fn mmapcopy(path: &CString) {
    let mut buf = mem::uninitialized();
    Stat(path, &mut buf);
    let size = buf.st_size as size_t;
    let fd = Open(path, O_RDONLY);
    let fp = Mmap(size, PROT_READ, MAP_PRIVATE, fd, 0);
    Write(STDOUT_FILENO, fp, size);
    Close(fd);
}

fn problem_9_5() {
    let path = CString::new(env::args().nth(1).unwrap()).unwrap();
    unsafe {
        mmapcopy(&path);
    }
}
