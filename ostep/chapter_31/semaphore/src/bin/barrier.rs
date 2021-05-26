use std::{
    env, process,
    sync::{Arc, Mutex},
    thread,
};

use semaphore::Semaphore;

fn main() {
    let mut args = env::args();
    let num_threads = args
        .nth(1)
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| {
            eprintln!("USAGE: EXEC NUM_THREADS");
            process::exit(1);
        });

    println!("parent: begin");

    let barrier = Arc::new(Barrier::new(num_threads));

    let handles = (0..num_threads)
        .map(|_| {
            let local_barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                let id = thread::current().id();
                println!("child {:?}: before", id);
                local_barrier.wait();
                println!("child {:?}: after", id);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("parent: end");
}

struct Barrier {
    target_threads: u32,
    /// The same to a counter protected by a binary semaphore, though the simple API of a semaphore
    /// won't allow mutable access to the counter from multiple threads.
    /// It's possible to implement a mutex lock on top of semaphore and [std::cell::UnsafeCell], but
    /// if done right the code would be strikingly similar to the implementation of [Mutex] in std
    /// with a proper poison guard.
    waiting_threads: Mutex<u32>,
    semaphore: Semaphore,
}

impl Barrier {
    fn new(num_threads: u32) -> Self {
        Self {
            target_threads: num_threads,
            waiting_threads: Mutex::new(0),
            semaphore: Semaphore::new(0),
        }
    }

    fn wait(&self) {
        let mut waiting = self.waiting_threads.lock().unwrap();
        *waiting += 1;
        if *waiting == self.target_threads {
            for _ in 0..self.target_threads {
                self.semaphore.post().unwrap();
            }
        }

        drop(waiting);
        self.semaphore.wait().unwrap();
    }
}
