use sleeping_ta::setup;
use std::{env, process};

fn main() {
    env_logger::init();

    let mut args = env::args();
    args.next();

    let students = args
        .next()
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXEC N_STUDENTS");
            process::exit(1);
        });

    setup(students);
}
