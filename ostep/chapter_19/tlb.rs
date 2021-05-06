use std::env;
use std::mem;
use std::process;
use std::ptr::write_volatile;
use std::time::Instant;

// As reported by getconf PAGESIZE in WSL.
const PAGESIZE: usize = 4 * 1024;
const JUMP: usize = PAGESIZE / mem::size_of::<u32>();

fn main() {
    let mut args = env::args();
    args.next();

    let num_pages = args
        .next()
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or_else(|| error_exit());

    let repeat = args
        .next()
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| error_exit());

    let mut vec = vec![0u32; num_pages * JUMP];
    let before = Instant::now();
    for _ in 0..repeat {
        for i in (0..num_pages).map(|n| n * JUMP) {
            let ptr = &mut vec[i] as *mut u32;
            unsafe {
                write_volatile(ptr, 1);
            }
        }
    }

    let pages_accessed = (num_pages as u128) * (repeat as u128);
    println!("{}", before.elapsed().as_nanos() / pages_accessed);
}

fn error_exit() -> ! {
    eprintln!("USAGE: EXEC NUM_OF_PAGES REPEAT");
    process::exit(1);
}
