use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::{
    env, process,
    sync::{Arc, Mutex},
    thread,
};

lazy_static! {
    static ref HIT_COUNT: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}

fn main() {
    let (trials, threads) = parse_args().unwrap_or_else(|| {
        eprintln!("Usage: EXEC N_TRIALS N_THREADS");
        process::exit(1);
    });

    estimate_pi(trials, threads);
}

fn parse_args() -> Option<(u32, u32)> {
    let mut args = env::args();
    args.next()?;

    let trials = args.next().and_then(|arg| arg.parse::<u32>().ok())?;
    let threads = args.next().and_then(|arg| arg.parse::<u32>().ok())?;

    Some((trials, threads))
}

fn estimate_pi(trials: u32, threads: u32) {
    let trial_per_thread = trials / threads;

    let handles = (0..threads)
        .map(|_| thread::spawn(move || monte_carlo_hit(trial_per_thread)))
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    let pi = 4.0 * f64::from(*HIT_COUNT.lock().unwrap()) / f64::from(trials);
    println!("Pi = {}", pi);
}

const RADIUS: f64 = 1.0;

fn monte_carlo_hit(trials: u32) {
    let mut hit = 0;
    let mut rng = thread_rng();

    for _ in 0..trials {
        let x = rng.gen_range(-RADIUS, RADIUS);
        let y = rng.gen_range(-RADIUS, RADIUS);
        if (x * x + y * y).sqrt() <= RADIUS {
            hit += 1;
        }
    }

    *HIT_COUNT.lock().unwrap() += hit;
}
