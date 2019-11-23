use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug)]
enum Bridge {
    South(u32),
    Empty,
    North(u32),
}

use Bridge::*;

pub struct BridgeLock {
    state_mutex: Mutex<Bridge>,
    cvar: Condvar,
}

impl BridgeLock {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            state_mutex: Mutex::new(Empty),
            cvar: Condvar::new(),
        })
    }

    pub fn south_acquire(&self) {
        let mut state = self.state_mutex.lock().unwrap();
        loop {
            match *state {
                Empty => {
                    *state = South(1);
                    break;
                }
                South(n) => {
                    *state = South(n + 1);
                    break;
                }
                North(..) => state = self.cvar.wait(state).unwrap(),
            }
        }
    }

    pub fn north_acquire(&self) {
        let mut state = self.state_mutex.lock().unwrap();
        loop {
            match *state {
                Empty => {
                    *state = North(1);
                    break;
                }
                North(n) => {
                    *state = North(n + 1);
                    break;
                }
                South(..) => state = self.cvar.wait(state).unwrap(),
            }
        }
    }

    pub fn release(&self) {
        let mut state = self.state_mutex.lock().unwrap();
        *state = match *state {
            South(1) | North(1) => {
                self.cvar.notify_all();
                Empty
            }
            South(n) => South(n - 1),
            North(n) => North(n - 1),
            Empty => unreachable!(),
        }
    }
}

const PASS_TIME: u64 = 1000;

pub fn south_villager(lock: &BridgeLock) {
    lock.south_acquire();
    println!("A new villager from south on the bridge");
    thread::sleep(Duration::from_millis(PASS_TIME));
    println!("A villager from south left the bridge");
    lock.release();
}

pub fn north_villager(lock: &BridgeLock) {
    lock.north_acquire();
    println!("A new villager from north on the bridge");
    thread::sleep(Duration::from_millis(PASS_TIME));
    println!("A villager from north left the bridge");
    lock.release();
}
