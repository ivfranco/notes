use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use semaphore::Semaphore;

fn main() {}

struct NsMutex<T> {
    poisoned: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> NsMutex<T> {
    fn new(data: T) -> Self {
        Self {
            poisoned: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    fn acquire(&self) -> NsMutexGuard<T> {
        todo!()
    }

    fn release(&self) {}
}

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
