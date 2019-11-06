use libc::{
    __error, c_char, c_int, c_uint, c_void, exit, fdopen, getline, open, printf, read, strerror,
    write, FILE, O_CREAT, O_EXCL, O_RDONLY, O_WRONLY, STDERR_FILENO, STDIN_FILENO, S_IRUSR,
    S_IWUSR,
};
use std::ptr::null_mut;

const READ: *const c_char = b"r\0" as *const u8 as *const c_char;
const WRITE: *const c_char = b"w\0" as *const u8 as *const c_char;
const BUF_LEN: usize = 0x100;

fn main() {
    unsafe {
        let stdin = fdopen(STDIN_FILENO, READ);
        if stdin.is_null() {
            error_exit(b"Error opening stdin: %s\0");
        }
        let stderr = fdopen(STDERR_FILENO, WRITE);
        if stderr.is_null() {
            error_exit(b"Error opening stderr: %s\0");
        }

        let source_name = read_file_name(stdin);
        let source_file = match open(source_name, O_RDONLY) {
            -1 => error_exit(b"Error opening source file: %s\0"),
            file => file,
        };

        let target_name = read_file_name(stdin);
        let target_file = match open(
            target_name,
            O_WRONLY | O_CREAT | O_EXCL,
            c_uint::from(S_IRUSR | S_IWUSR),
        ) {
            -1 => error_exit(b"Error opening target file: %s\0"),
            file => file,
        };

        copy_file(source_file, target_file);
    }
}

unsafe fn error_exit(desc: &[u8]) -> ! {
    printf(desc.as_ptr() as *const c_char, strerror(*__error()));
    exit(1);
}

unsafe fn copy_file(source_file: c_int, target_file: c_int) {
    let mut buf = [0; BUF_LEN];
    loop {
        match read(source_file, buf.as_mut_ptr() as *mut c_void, BUF_LEN) {
            -1 => error_exit(b"Error reading source file: %s\0"),
            0 => break,
            read_len => {
                let mut write_ptr = buf.as_ptr();
                let mut remain = read_len as usize;
                while remain > 0 {
                    match write(target_file, write_ptr as *mut c_void, remain) {
                        -1 => error_exit(b"Error writing target file: %s\0"),
                        write_len => {
                            remain -= write_len as usize;
                            write_ptr = write_ptr.add(write_len as usize);
                        }
                    }
                }
            }
        }
    }
}

unsafe fn read_file_name(stdin: *mut FILE) -> *const c_char {
    let mut lineptr = null_mut();
    let mut n = 0;
    match getline(&mut lineptr, &mut n, stdin) {
        -1 => error_exit(b"Error reading file name: %s\0"),
        len => {
            *lineptr.add(len as usize - 1) = b'\0' as c_char;
            lineptr
        }
    }
}
