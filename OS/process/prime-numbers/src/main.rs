use std::{env, process, thread};

fn main() {
    let mut args = env::args();
    args.next();
    let limit = args
        .next()
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXEC LIMIT");
            process::exit(1);
        });

    thread::spawn(move || {
        prime_numbers(limit);
    })
    .join()
    .unwrap();
}

fn prime_numbers(limit: u32) {
    let mut primes = vec![];

    for n in 2..limit {
        if primes.iter().all(|p| n % p != 0) {
            primes.push(n);
            print!("{}, ", n);
        }
    }
}
