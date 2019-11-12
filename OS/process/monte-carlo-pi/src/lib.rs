use rand::{thread_rng, Rng};
use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
};

static HITS: AtomicU32 = AtomicU32::new(0);

pub fn pi_estimation(trial: u32) -> f64 {
    let handle = thread::spawn(move || monte_carlo(trial));
    handle.join().unwrap();

    let hits = HITS.load(Ordering::Acquire);
    4.0 * f64::from(hits) / f64::from(trial)
}

fn monte_carlo(trial: u32) {
    let mut hits = 0;
    let mut rng = thread_rng();

    for _ in 0..trial {
        let x: f64 = rng.gen_range(-1.0, 1.0);
        let y: f64 = rng.gen_range(-1.0, 1.0);
        let r = (x * x + y * y).sqrt();
        if r <= 1.0 {
            hits += 1;
        }
    }

    HITS.store(hits, Ordering::Release);
}
