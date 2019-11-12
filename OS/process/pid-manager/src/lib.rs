use std::collections::HashSet;
use std::sync::{Arc, Mutex};

type Pid = usize;

#[derive(Clone)]
pub struct PidManager {
    allocated: Arc<Mutex<HashSet<Pid>>>,
}

impl PidManager {
    pub fn new() -> Self {
        Self {
            allocated: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn acquire(&self) -> Pid {
        let mut alloc = self.allocated.lock().unwrap();
        (0..)
            .find(|pid| alloc.insert(*pid))
            .expect("the system will OOM before the usize overflows")
    }

    pub fn release(&self, pid: Pid) {
        let mut alloc = self.allocated.lock().unwrap();
        alloc.remove(&pid);
    }

    pub fn len(&self) -> usize {
        self.allocated.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[test]
fn pid_test() {
    use rand::{thread_rng, Rng};
    use std::{thread, time::Duration};

    let manager = PidManager::new();
    let mut rng = thread_rng();

    let threads = (0..100)
        .map(|_| {
            let ms = rng.gen_range(0, 2000);
            let clone = manager.clone();
            thread::spawn(move || {
                let pid = clone.acquire();
                thread::sleep(Duration::from_millis(ms));
                clone.release(pid);
            })
        })
        .collect::<Vec<_>>();

    threads
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    assert!(manager.is_empty());
}
