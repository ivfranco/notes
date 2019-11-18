use std::{
    error::Error,
    sync::{Arc, Condvar, Mutex},
};

pub struct Barrier {
    count_mutex: Mutex<u32>,
    cvar: Condvar,
}

impl Barrier {
    pub fn new(count: u32) -> Arc<Self> {
        Arc::new(Self {
            count_mutex: Mutex::new(count),
            cvar: Condvar::new(),
        })
    }

    pub fn point<'a>(&'a self) -> Result<(), Box<dyn Error + 'a>> {
        let mut count = self.count_mutex.lock()?;
        *count = count.saturating_sub(1);
        if *count == 0 {
            self.cvar.notify_all();
        } else {
            let _ = self.cvar.wait(count)?;
        }

        Ok(())
    }
}

#[test]
fn n_m_test() {
    use std::thread;

    let barrier = Barrier::new(4);
    let handles = (0..6)
        .map(|_| {
            let clone = barrier.clone();
            thread::spawn(move || {
                clone.point().unwrap();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
}
