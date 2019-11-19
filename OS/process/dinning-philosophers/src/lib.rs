use rand::{thread_rng, Rng};
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Thinking,
    Hungry,
    Eating,
}

impl Default for State {
    fn default() -> Self {
        Thinking
    }
}

use State::*;

const N_PHILOSOPHERS: usize = 5;
type States = [State; N_PHILOSOPHERS];

#[derive(Default)]
struct DinningTable {
    states_mutex: Mutex<States>,
    cvars: [Condvar; N_PHILOSOPHERS],
}

impl DinningTable {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn pickup_forks(&self, i: usize) {
        let mut states = self.states_mutex.lock().unwrap();
        states[i] = Hungry;
        fork_test(&mut states, &self.cvars, i);
        if states[i] != Eating {
            let _ = self.cvars[i].wait(states).unwrap();
        }
    }

    fn putdown_forks(&self, i: usize) {
        let mut states = self.states_mutex.lock().unwrap();
        states[i] = Thinking;
        fork_test(
            &mut states,
            &self.cvars,
            (i + N_PHILOSOPHERS - 1) % N_PHILOSOPHERS,
        );
        fork_test(&mut states, &self.cvars, (i + 1) % N_PHILOSOPHERS);
    }
}

fn fork_test(states: &mut [State; N_PHILOSOPHERS], cvars: &[Condvar; N_PHILOSOPHERS], i: usize) {
    if states[(i + N_PHILOSOPHERS - 1) % N_PHILOSOPHERS] != Eating
        && states[i] == Hungry
        && states[(i + 1) % N_PHILOSOPHERS] != Eating
    {
        states[i] = Eating;
        cvars[i].notify_one();
    }
}

const EATING_TIME: u64 = 1000;
const THINKING_TIME: u64 = 3000;

pub fn setup() {
    let table = DinningTable::new();
    for i in 0..N_PHILOSOPHERS {
        let clone = table.clone();
        thread::spawn(move || philosopher(i, clone));
    }

    thread::park();
}

fn philosopher(i: usize, table: Arc<DinningTable>) {
    let mut rng = thread_rng();

    loop {
        let ms = rng.gen_range(0, THINKING_TIME);
        thread::sleep(Duration::from_millis(ms));
        table.pickup_forks(i);
        println!("Philosopher {} picked up forks", i);
        thread::sleep(Duration::from_millis(EATING_TIME));
        table.putdown_forks(i);
        println!("Philosopher {} put down forks", i);
    }
}
