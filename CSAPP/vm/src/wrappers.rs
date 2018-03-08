use libc::*;
use std::ptr;
use std::ffi::CString;

pub const NULL: *mut c_void = 0 as *mut c_void;

#[cfg(target_os = "macos")]
pub unsafe fn errno() -> *mut c_int {
    __error()
}
#[cfg(target_os = "linux")]
pub unsafe fn errno() -> *mut c_int {
    __error_location()
}

pub unsafe fn unix_error(msg: &str) -> ! {
    let error = CString::from_raw(strerror(*errno()));
    eprintln!("{}: {:?}", msg, error);
    exit(0)
}

pub unsafe fn Open(path: &CString, oflag: c_int) -> c_int {
    let fd = open(path.as_ptr(), oflag);
    if fd < 0 {
        unix_error("Open error")
    } else {
        fd
    }
}

pub unsafe fn Mmap(
    length: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: off_t,
) -> *mut c_void {
    let ptr = mmap(ptr::null_mut(), length, prot, flags, fd, offset);
    if ptr == MAP_FAILED {
        unix_error("Mmap error")
    } else {
        ptr
    }
}

pub unsafe fn Stat(path: &CString, buf: &mut stat) {
    if stat(path.as_ptr(), buf as *mut stat) < 0 {
        unix_error("Stat error");
    }
}

pub unsafe fn Write(fd: c_int, buf: *const c_void, nbyte: size_t) -> ssize_t {
    let written = write(fd, buf, nbyte);
    if written < 0 {
        unix_error("Write error")
    } else {
        written
    }
}

pub unsafe fn Close(fd: c_int) {
    if close(fd) < 0 {
        unix_error("Close error");
    }
}

pub unsafe fn Malloc(size: size_t) -> *mut c_void {
    let start = malloc(size);
    if start == NULL {
        unix_error("Malloc error")
    } else {
        start
    }
}
