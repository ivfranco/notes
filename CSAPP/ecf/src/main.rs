#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate libc;

use libc::*;
use std::env;
use std::ffi::CString;
use std::ptr;

fn main() {
    problem_8_22();
}

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

fn to_null_terminated(strs: &[CString]) -> Vec<*const c_char> {
    let mut ptrs: Vec<*const c_char> = strs.iter().map(|s| s.as_ptr()).collect();
    ptrs.push(ptr::null());
    ptrs
}

unsafe fn Execve(path: &CString, args: &[CString], envs: &[CString]) -> c_int {
    let argv = to_null_terminated(args);
    let envp = to_null_terminated(envs);
    let ret_code = execve(path.as_ptr(), argv.as_ptr(), envp.as_ptr());
    if ret_code < 0 {
        unix_error("Exec error")
    } else {
        ret_code
    }
}

unsafe fn Waitpid(pid: pid_t, status: &mut c_int, options: c_int) -> pid_t {
    let reaped = waitpid(pid, status as *mut c_int, options);
    if reaped < 0 {
        unix_error("Waitpid error")
    } else {
        reaped
    }
}

// myls
fn problem_8_20() {
    let ls = CString::new("/bin/ls").unwrap();
    let args: Vec<CString> = env::args().map(|s| CString::new(s).unwrap()).collect();
    let envs: Vec<CString> = env::vars()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect();
    unsafe {
        Execve(&ls, &args, &envs);
    }
}

unsafe fn mysystem(command: CString) -> c_int {
    let mut status = 0;
    if Fork() == 0 {
        let sh = CString::new("/bin/sh").unwrap();
        let args = [sh.clone(), CString::new("-c").unwrap(), command];
        let envs: Vec<CString> = env::vars()
            .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
            .collect();
        Execve(&sh, &args, &envs);
        exit(0);
    } else {
        Waitpid(-1, &mut status, 0);
        if WIFEXITED(status) {
            return WEXITSTATUS(status);
        }
        if WIFSIGNALED(status) {
            return WTERMSIG(status);
        }
        return -1;
    }
}

fn problem_8_22() {
    unsafe {
        let command = CString::new("/bin/ps").unwrap();
        println!("{}", mysystem(command));
    }
}
