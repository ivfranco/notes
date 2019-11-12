use monte_carlo_pi::pi_estimation;
use std::{env, process};

fn main() {
    let mut args = env::args();
    args.next();

    let trial = args
        .next()
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXEC TRIAL_NUMBER");
            process::exit(1);
        });

    println!("π = {}", pi_estimation(trial));
}
