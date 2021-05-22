use std::{
    thread,
    time::{Duration, Instant},
};

use concurrent_structures::{
    counter::{Counter, LockedCounter, SloppyCounter},
    linked_list::{HandOverHandLinkedList, LockedLinkedList},
    ConcurrentSet,
};

fn main() {
    counter_measurement();

    for threads in (0..6).map(|n| 2u32.pow(n)) {
        println!("{} threads", threads);
        println!(
            "    Locked linked list: {:?}",
            set_measurement(LockedLinkedList::new(), threads)
        );
        println!(
            "    Hand-over-hand linked list: {:?}",
            set_measurement(HandOverHandLinkedList::new(), threads)
        );
    }
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

fn set_measurement<S: ConcurrentSet<i32, i32> + Clone + Send + 'static>(
    set: S,
    threads: u32,
) -> Duration {
    const REPEAT: u32 = 2u32.pow(12);

    let before = Instant::now();

    let handles = (0..threads)
        .map(|thread| {
            let local = set.clone();
            let thread_job = REPEAT / threads;
            let thread_start = thread_job * thread;

            thread::spawn(move || {
                for i in thread_start..thread_start + thread_job {
                    let key = i as i32;
                    if key != 0 {
                        local.insert(-key, key);
                        local.insert(key, key);
                        local.remove(&-key);
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    for i in 1..REPEAT as i32 {
        assert_eq!(set.get(&i), Some(i));
        assert_eq!(set.get(&-i), None);
    }

    before.elapsed()
}
