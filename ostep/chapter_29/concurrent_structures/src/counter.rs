use std::sync::{Arc, Mutex};

pub trait Counter {
    fn update(&mut self, amt: i32);
    fn get(&self) -> i32;
    /// Derive a new thread-local counter from the old one. The local buffer, if any, is reset to
    /// avoid counting twice the same values.
    fn fork(&self) -> Self;
}

pub struct LockedCounter {
    count: Arc<Mutex<i32>>,
}

impl LockedCounter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
        }
    }
}

impl Counter for LockedCounter {
    fn update(&mut self, amt: i32) {
        let mut count = self.count.lock().unwrap();
        *count += amt;
    }

    fn get(&self) -> i32 {
        *self.count.lock().unwrap()
    }

    fn fork(&self) -> Self {
        Self {
            count: Arc::clone(&self.count),
        }
    }
}

pub struct SloppyCounter {
    global: LockedCounter,
    local: i32,
    threshold: u32,
}

impl SloppyCounter {
    pub fn new(threshold: u32) -> Self {
        Self {
            global: LockedCounter::new(),
            local: 0,
            threshold,
        }
    }

    fn flush(&mut self) {
        self.global.update(self.local);
        self.local = 0;
    }
}

impl Drop for SloppyCounter {
    fn drop(&mut self) {
        self.flush();
    }
}

impl Counter for SloppyCounter {
    fn update(&mut self, amt: i32) {
        self.local += amt;
        if self.local.abs() as u32 >= self.threshold {
            self.flush();
        }
    }

    fn get(&self) -> i32 {
        self.global.get()
    }

    fn fork(&self) -> Self {
        Self {
            global: self.global.fork(),
            local: 0,
            threshold: self.threshold,
        }
    }
}
