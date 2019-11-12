use std::{env, mem, process, thread};

fn main() {
    let mut args = env::args();
    args.next();
    let len = args
        .next()
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXEC LENGTH_OF_SEQUENCE");
            process::exit(1);
        });

    // use implicitly shared memory in a JoinHandle allocated for the return value
    // otherwise the sequence can be stored in a global variable initialized by lazy_static macro
    // and protected by a Mutex
    let sequence = thread::spawn(move || fibonacci(len)).join().unwrap();
    println!("{:?}", sequence);
}

fn fibonacci(len: usize) -> Vec<u32> {
    let mut sequence = vec![];

    let mut a: u32 = 0;
    let mut b: u32 = 1;

    for i in 0..len {
        sequence.push(b);
        mem::swap(&mut a, &mut b);
        if let Some(sum) = a.checked_add(b) {
            b = sum;
        } else {
            eprintln!(
                "Error: overflow occurred when calculating {} fibonacci number",
                i + 2
            );
            process::exit(1);
        }
    }

    sequence
}
