use std::{
    cell::UnsafeCell,
    env::{self, Args},
    fmt::Display,
    ops::{Deref, DerefMut},
    process,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use semaphore::Semaphore;

const SLEEP: u64 = 100;

fn reader(loops: u32, lock: &RwLock<u32>) {
    for _ in 0..loops {
        thread::sleep(Duration::from_millis(SLEEP));
        let value = lock.read();
        println!("read {}", *value);
        // lock released by RAII guard
    }
}

fn writer(loops: u32, lock: &RwLock<u32>) {
    for _ in 0..loops {
        thread::sleep(Duration::from_millis(SLEEP));
        let mut value = lock.write();
        *value += 1;
        println!("write {}", *value);
        // lock released by RAII guard
    }
}

#[allow(clippy::needless_collect)]
fn main() {
    let mut args = env::args();
    args.next();

    let num_readers: u32 = parse_next_arg(&mut args, "invalid NUM_READERS");
    let num_writers: u32 = parse_next_arg(&mut args, "invalid NUM_WRITERS");
    let loops: u32 = parse_next_arg(&mut args, "invalid LOOPS");

    let lock = Arc::new(RwLock::new(0));

    println!("begin");

    let readers = (0..num_readers)
        .map(|_| {
            let local = Arc::clone(&lock);
            thread::spawn(move || {
                reader(loops, &local);
            })
        })
        .collect::<Vec<_>>();

    let writers = (0..num_writers)
        .map(|_| {
            let local = Arc::clone(&lock);
            thread::spawn(move || {
                writer(loops, &local);
            })
        })
        .collect::<Vec<_>>();

    for handle in readers.into_iter().chain(writers) {
        handle.join().unwrap();
    }

    println!("end: value {}", *lock.read());
}

fn parse_next_arg<T: FromStr>(args: &mut Args, desc: impl Display) -> T {
    args.next()
        .and_then(|arg| arg.parse::<T>().ok())
        .unwrap_or_else(|| {
            error_exit(desc);
        })
}

fn error_exit(err: impl Display) -> ! {
    eprintln!("USAGE: EXEC NUM_READERS NUM_WRITERS LOOPS");
    eprintln!("{}", err);
    process::exit(1);
}

struct RwLock<T: ?Sized> {
    readers: Mutex<u32>,

    reader_semaphore: Semaphore,
    writer_semaphore: Semaphore,

    poisoned: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> RwLock<T> {
    fn new(data: T) -> Self {
        Self {
            readers: Mutex::new(0),

            reader_semaphore: Semaphore::new(1),
            writer_semaphore: Semaphore::new(1),

            poisoned: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    fn panic_on_poison(&self) {
        if self.poisoned.load(Ordering::Relaxed) {
            panic!("RwLock poisoned");
        }
    }

    fn read(&self) -> ReadGuard<T> {
        self.panic_on_poison();

        self.reader_semaphore.wait().unwrap();
        self.reader_semaphore.post().unwrap();

        let mut readers = self.readers.lock().unwrap();

        *readers += 1;

        if *readers == 1 {
            self.writer_semaphore.wait().unwrap();
        }

        ReadGuard { lock: self }
    }

    fn read_unlock(&self) {
        let mut readers = self.readers.lock().unwrap();
        *readers -= 1;

        if *readers == 0 {
            self.writer_semaphore.post().unwrap();
        }
    }

    fn write(&self) -> WriteGuard<T> {
        self.panic_on_poison();

        self.reader_semaphore.wait().unwrap();
        self.writer_semaphore.wait().unwrap();

        WriteGuard { lock: self }
    }

    fn write_unlock(&self) {
        self.writer_semaphore.post().unwrap();
        self.reader_semaphore.post().unwrap();
    }
}

unsafe impl<T: ?Sized + Send> Send for RwLock<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

struct ReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<'a, T> Deref for ReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> Drop for ReadGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.read_unlock();
    }
}

struct WriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<'a, T> Deref for WriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // # Safety
        // The first reader waits on the semaphore shared by readers and writers, the semaphore
        // prevents readers from having read access to the data whenever there's a concurrent writer
        // and prevent writers from having write access to the data whenever there's a concurrent
        // reader, the exact XOR borrow rules imposed by borrow checker.
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for WriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // # Safety
        // Same to above.
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for WriteGuard<'a, T> {
    fn drop(&mut self) {
        if thread::panicking() {
            // A writer panicked while holding the lock may corrupt the data, the natural solution
            // to such memory corruption is to panic every thread trying to access the same lock.
            self.lock.poisoned.store(true, Ordering::Relaxed);
        }

        self.lock.write_unlock();
    }
}
