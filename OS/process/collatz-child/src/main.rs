use nix::{sys::wait::wait, unistd::fork};
use std::{env, process};

fn main() {
    let mut args = env::args();
    args.next();

    let collatz = args
        .next()
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| {
            eprintln!("Error: input is not a non-negative number");
            process::exit(1);
        });

    if let Err(err) = fork_collatz_child(collatz) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn fork_collatz_child(mut collatz: u32) -> nix::Result<()> {
    if fork()?.is_parent() {
        wait()?;
    } else {
        while collatz != 1 {
            println!("{}", collatz);
            if collatz % 2 == 0 {
                collatz /= 2;
            } else {
                collatz = collatz * 3 + 1;
            }
        }
        println!("1");
    }

    Ok(())
}
