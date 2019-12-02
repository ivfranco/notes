use std::{env, ffi::CStr, mem::size_of, process, ptr, thread};
use winapi::{
    shared::{minwindef::*, ntdef::NULL},
    um::{
        errhandlingapi::*, fileapi::*, handleapi::*, memoryapi::*,
        minwinbase::LPSECURITY_ATTRIBUTES, winbase::*, winnt::*,
    },
};

const TEMP_FILE_NAME: &[u8] = b"temp.txt\0";
const SHARED_MEMORY_NAME: &[u8] = b"SharedObject\0";

struct Win32Error(DWORD);

fn main() {
    let (start, iter) = parse_args().unwrap_or_else(|| {
        eprintln!("Usage: EXEC START_NUMBER N_ITERATION");
        process::exit(1);
    });

    let handle = thread::spawn(move || unsafe {
        if let Err(Win32Error(code)) = producer(start, iter) {
            eprintln!("Producer error code: {}", code);
        }
    });

    handle.join().unwrap();

    unsafe {
        if let Err(Win32Error(code)) = consumer(iter) {
            eprintln!("Consumer error code: {}", code);
        }
    }
}

fn parse_args() -> Option<(u32, usize)> {
    let mut args = env::args();
    args.next()?;

    let start = args.next().and_then(|arg| arg.parse::<u32>().ok())?;
    let iter = args.next().and_then(|arg| arg.parse::<usize>().ok())?;

    Some((start, iter))
}

unsafe fn producer(start: u32, iter: usize) -> Result<(), Win32Error> {
    let mut buf = vec![0; iter];
    buf[0] = start;
    for i in 1..buf.len() {
        let collatz = buf[i - 1];
        buf[i] = if collatz % 2 == 0 {
            collatz / 2
        } else {
            collatz * 3 + 1
        };
    }

    let temp_file_name = CStr::from_bytes_with_nul(TEMP_FILE_NAME).unwrap();
    let shared_memory_name = CStr::from_bytes_with_nul(SHARED_MEMORY_NAME).unwrap();

    let h_file = CreateFileA(
        temp_file_name.as_ptr(),
        GENERIC_READ | GENERIC_WRITE,
        0,
        NULL as LPSECURITY_ATTRIBUTES,
        OPEN_ALWAYS,
        FILE_ATTRIBUTE_NORMAL,
        NULL,
    );
    if h_file == INVALID_HANDLE_VALUE {
        return Err(capture_error_code("producer: error opening file"));
    }

    let h_map_file = CreateFileMappingA(
        h_file,
        NULL as LPSECURITY_ATTRIBUTES,
        PAGE_READWRITE,
        0,
        // otherwise the call will return error code 1006
        // see https://social.msdn.microsoft.com/Forums/vstudio/en-US/0f5a275f-a125-48ea-a15d-512d5a2a22d8
        (buf.len() * size_of::<u32>()) as u32,
        shared_memory_name.as_ptr(),
    );
    if h_map_file == NULL {
        return Err(capture_error_code("producer: error creating file mapping"));
    }

    let file_view = MapViewOfFile(h_map_file, FILE_MAP_ALL_ACCESS, 0, 0, 0);
    if file_view == NULL {
        return Err(capture_error_code("producer: error creating file view"));
    }

    ptr::copy(
        buf.as_ptr() as *const u8,
        file_view as *mut u8,
        buf.len() * size_of::<u32>(),
    );

    UnmapViewOfFile(file_view);
    CloseHandle(h_file);

    Ok(())
}

unsafe fn consumer(iter: usize) -> Result<(), Win32Error> {
    let mut buf = vec![0; iter];
    let shared_memory_name = CStr::from_bytes_with_nul(SHARED_MEMORY_NAME).unwrap();

    let h_map_file = OpenFileMappingA(FILE_MAP_ALL_ACCESS, FALSE, shared_memory_name.as_ptr());
    if h_map_file == NULL {
        return Err(capture_error_code("consumer: error opening file mapping"));
    }

    let file_view = MapViewOfFile(h_map_file, FILE_MAP_ALL_ACCESS, 0, 0, 0);
    if file_view == NULL {
        return Err(capture_error_code("consumer: error creating file view"));
    }

    ptr::copy(
        file_view as *const u8,
        buf.as_mut_ptr() as *mut u8,
        iter * size_of::<u32>(),
    );

    println!("{:?}", buf);

    UnmapViewOfFile(file_view);
    CloseHandle(h_map_file);

    Ok(())
}

unsafe fn capture_error_code(msg: &str) -> Win32Error {
    // error code must be obtained before calling eprintln!
    // otherwise standard io will flush error code to 0
    let err = Win32Error(GetLastError());
    eprintln!("{}", msg);
    err
}
