#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate libc;

use libc::*;
use std::env;
use std::ffi::CString;
use std::ptr;

fn main() {}

#[cfg(target_os = "macos")]
unsafe fn errno() -> i32 {
    *__error()
}
#[cfg(target_os = "linux")]
unsafe fn errno() -> i32 {
    *__error_location()
}

unsafe fn unix_error(msg: &str) -> ! {
    let error = CString::from_raw(strerror(errno()));
    eprintln!("{}: {:?}", msg, error);
    exit(0)
}

unsafe fn Fork() -> pid_t {
    let pid = fork();
    if pid < 0 {
        unix_error("Fork error")
    } else {
        pid
    }
}

unsafe fn Signal(signum: c_int, handler: sighandler_t) -> sighandler_t {
    let mut action = sigaction {
        sa_sigaction: handler,
        sa_mask: 0,
        sa_flags: SA_RESTART,
    };
    let mut old_action: sigaction = ::std::mem::uninitialized();

    sigemptyset(&mut action.sa_mask as *mut u32);
    if sigaction(signum, &mut action, &mut old_action) < 0 {
        unix_error("Signal error")
    } else {
        old_action.sa_sigaction
    }
}

unsafe fn snooze(secs: c_uint) {
    let rem = sleep(secs);
    println!("Slept for {} of {} secs.", secs - rem, secs);
}

// myecho
fn problem_8_6() {
    println!("Command line arguments:");
    for (i, arg) in env::args().enumerate() {
        println!("\targv[{:>2}]: {}", i, arg);
    }
    println!("\nEnvironment variables:");
    for (i, (ekey, evar)) in env::vars().enumerate() {
        println!("\tenvp[{:>2}]: {}={}", i, ekey, evar);
    }
}

fn problem_8_7() {
    extern "C" fn return_handler(_: c_int) {
        return;
    }

    let secs: c_uint = env::args().nth(1).unwrap().parse().unwrap();
    unsafe {
        Signal(SIGINT, return_handler as *mut c_void as sighandler_t);
        snooze(secs);
    }
}
