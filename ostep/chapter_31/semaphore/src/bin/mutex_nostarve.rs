use std::{
    cell::UnsafeCell,
    collections::HashMap,
    env::{self, Args},
    fmt::Display,
    ops::{Deref, DerefMut},
    process,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use semaphore::Semaphore;

fn main() {
    let mut args = env::args();
    args.next();
    let num_threads = parse_next_arg(&mut args, "invalid NUM_THREADS");
    let loops = parse_next_arg(&mut args, "invalid LOOPS");

    let freq = Arc::new(NsMutex::new(HashMap::new()));

    println!("begin");

    let handles = (0..num_threads)
        .map(|_| {
            let local = Arc::clone(&freq);
            thread::spawn(move || {
                for _ in 0..loops {
                    let id = thread::current().id();
                    let mut freq = local.acquire();
                    *freq.entry(id).or_insert(0u32) += 1;
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    let mut total = 0;
    for (id, turns) in &*freq.acquire() {
        println!("{:?} acquired the lock {} times", id, turns);
        total += turns;
    }

    assert_eq!(total, num_threads * loops);
    println!("end");
}

fn parse_next_arg<T: FromStr>(args: &mut Args, desc: impl Display) -> T {
    args.next()
        .and_then(|arg| arg.parse::<T>().ok())
        .unwrap_or_else(|| {
            error_exit(desc);
        })
}

fn error_exit(err: impl Display) -> ! {
    eprintln!("USAGE: EXEC NUM_THREADS LOOPS");
    eprintln!("{}", err);
    process::exit(1);
}

struct NsMutex<T> {
    mutex: Semaphore,
    t1: Semaphore,
    t2: Semaphore,
    room1: UnsafeCell<u32>,
    room2: UnsafeCell<u32>,

    poisoned: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> NsMutex<T> {
    fn new(data: T) -> Self {
        Self {
            mutex: Semaphore::new(1),
            t1: Semaphore::new(1),
            t2: Semaphore::new(0),
            room1: UnsafeCell::new(0),
            room2: UnsafeCell::new(0),

            poisoned: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    fn acquire(&self) -> NsMutexGuard<T> {
        unsafe {
            self.mutex.wait().unwrap();
            *self.room1.get() += 1;
            self.mutex.post().unwrap();

            self.t1.wait().unwrap();
            *self.room2.get() += 1;
            self.mutex.wait().unwrap();
            *self.room1.get() -= 1;

            if *self.room1.get() == 0 {
                self.mutex.post().unwrap();
                self.t2.post().unwrap();
            } else {
                self.mutex.post().unwrap();
                self.t1.post().unwrap();
            }

            self.t2.wait().unwrap();
            *self.room2.get() -= 1;

            NsMutexGuard { lock: self }
        }
    }

    fn release(&self) {
        unsafe {
            if *self.room2.get() == 0 {
                self.t1.post().unwrap();
            } else {
                self.t2.post().unwrap();
            }
        }
    }
}

unsafe impl<T: Send> Send for NsMutex<T> {}
unsafe impl<T: Send + Sync> Sync for NsMutex<T> {}

struct NsMutexGuard<'a, T> {
    lock: &'a NsMutex<T>,
}

impl<'a, T> Deref for NsMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for NsMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for NsMutexGuard<'a, T> {
    fn drop(&mut self) {
        if thread::panicking() {
            self.lock.poisoned.store(true, Ordering::Relaxed);
        }

        self.lock.release();
    }
}
