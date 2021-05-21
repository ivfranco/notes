use std::{
    thread,
    time::{Duration, Instant},
};

use concurrent_structures::counter::{Counter, LockedCounter, SloppyCounter};

fn main() {
    counter_measurement();
}

fn counter_measurement() {
    fn measure<C: Counter + Send + 'static>(counter: C, threads: u32) -> Duration {
        const REPEAT: u32 = 2u32.pow(20);
        assert_eq!(REPEAT % threads, 0);

        let before = Instant::now();

        let handles = (0..threads)
            .map(|_| {
                let mut local = counter.fork();
                thread::spawn(move || {
                    for _ in 0..REPEAT / threads {
                        local.update(1);
                    }

                    drop(local);
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }

        let elapsed = before.elapsed();
        assert_eq!(counter.get(), REPEAT as i32);
        elapsed
    }

    for threads in (0..6).map(|n| 2u32.pow(n)) {
        println!("{} threads", threads);
        println!(
            "    Global lock counter: {:?}",
            measure(LockedCounter::new(), threads)
        );
        println!(
            "    Sloppy counter: {:?}",
            measure(SloppyCounter::new(6), threads)
        );
    }

    const NUM_THREADS: u32 = 8;
    for threshold in (0..6).map(|n| 2u32.pow(n)) {
        println!("threshold = {}, {} threads", threshold, NUM_THREADS);
        println!(
            "    Sloppy counter: {:?}",
            measure(SloppyCounter::new(threshold), NUM_THREADS)
        );
    }
}
