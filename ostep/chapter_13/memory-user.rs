use std::env;
use std::process;

fn main() {
    const MB: usize = 1024 * 1024;

    let mut args = env::args();
    let mem = args.nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or_else(|| {
            eprintln!("USAGE: EXEC MEM_IN_MEGA");
            process::exit(1);
        });

    let mut vec = vec![0u8; mem * MB];

    loop {
        for byte in vec.iter_mut() {
            *byte = (*byte).wrapping_add(1);
        }
    }
}
