use std::sync::{Arc, Condvar, Mutex};

pub struct Unavailable;

pub struct ResourceManager {
    count_mutex: Mutex<u32>,
    cvar: Condvar,
}

impl ResourceManager {
    pub fn new(count: u32) -> Arc<Self> {
        Arc::new(Self {
            count_mutex: Mutex::new(count),
            cvar: Condvar::new(),
        })
    }

    pub fn block_decrease_count(&self, dec: u32) -> Result<(), Unavailable> {
        let mut count = self.count_mutex.lock().unwrap();
        if *count < dec {
            Err(Unavailable)
        } else {
            *count -= dec;
            Ok(())
        }
    }

    pub fn decrease_count(&self, dec: u32) {
        let mut count = self.count_mutex.lock().unwrap();
        while *count < dec {
            count = self.cvar.wait(count).unwrap();
        }
        *count -= dec;
    }

    pub fn increase_count(&self, inc: u32) {
        *self.count_mutex.lock().unwrap() += inc;
        self.cvar.notify_all();
    }
}
