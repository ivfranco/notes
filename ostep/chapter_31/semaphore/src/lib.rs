//! Rust clone of the code exercises related to semaphores as part of the book Operating Systems:
//! Three Easy Pieces. The original C code can be obtained at
//! https://pages.cs.wisc.edu/~remzi/OSTEP/Homework/homework.html.

#![deny(missing_docs)]

use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

/// Counting semaphore to control concurrent access to a fixed number of shared resource. A rust
/// clone of the code shown in Figure 31.17: Implementing Zemaphores With Locks And CVs of OSTEP.
pub struct Semaphore {
    cond: Condvar,
    limit: Mutex<u32>,
}

impl Semaphore {
    /// Create a new semaphore which limits the maximum number of threads having access to a shared
    /// resource, usually entrance to the critical section or access to shared memory.
    pub fn new(limit: u32) -> Self {
        Semaphore {
            cond: Condvar::new(),
            limit: Mutex::new(limit),
        }
    }

    /// Wait on the semaphore until the resources are available. Block the calling thread when the
    /// number of threads accessing the resources is equal ot or more than allowed.
    pub fn wait(&self) -> Result<(), PoisonError<MutexGuard<u32>>> {
        let mut value = self.limit.lock()?;
        while *value == 0 {
            value = self.cond.wait(value)?;
        }

        *value -= 1;
        Ok(())
    }

    /// Release the semaphore, allowing other threads to have access to the resources.
    pub fn post(&self) -> Result<(), PoisonError<MutexGuard<u32>>> {
        let mut value = self.limit.lock()?;
        *value += 1;
        self.cond.notify_one();
        Ok(())
    }
}
