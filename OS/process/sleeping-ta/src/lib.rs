use log::{info, warn};
use rand::{thread_rng, Rng};
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread::{self, Thread},
    time::Duration,
};

const SEATS: usize = 3;

pub enum Error {
    OnCapacity,
}

struct Office {
    seats_mutex: Mutex<VecDeque<Thread>>,
    alarm: Condvar,
}

impl Office {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            seats_mutex: Mutex::new(VecDeque::with_capacity(SEATS)),
            alarm: Condvar::new(),
        })
    }

    fn ask_for_help(&self) -> Result<(), Error> {
        let mut seats = self.seats_mutex.lock().unwrap();
        let id = thread::current().id();

        if seats.len() >= SEATS {
            warn!(
                "Student {:?} asked for help but the office is on capacity",
                id
            );
            Err(Error::OnCapacity)
        } else {
            // non standard pthread API (available on NetBSD)
            // otherwise student threads can push their own mutex-condvar pair to the queue and wait
            // tutor will notify them later through the pair
            seats.push_back(thread::current());
            self.alarm.notify_one();
            info!("Student {:?} seats for help and awakes the tutor", id);
            drop(seats);
            thread::park();
            Ok(())
        }
    }
}

const TUTOR_TIME: u64 = 1000;
const PROGRAMMING_TIME: u64 = 5000;

fn student(office: Arc<Office>) {
    info!("Student {:?} started", thread::current().id());
    let mut rng = thread_rng();
    loop {
        let ms = rng.gen_range(0, PROGRAMMING_TIME);
        thread::sleep(Duration::from_millis(ms));
        let _ = office.ask_for_help();
    }
}

pub fn setup(students: u32) {
    let office = Office::new();

    {
        let clone = office.clone();
        thread::spawn(|| tutor(clone));
    }

    for _ in 0..students {
        let clone = office.clone();
        thread::spawn(|| student(clone));
    }

    thread::park();
}

fn tutor(office: Arc<Office>) {
    let mut seats = office.seats_mutex.lock().unwrap();
    loop {
        info!("Tutor takes a nap");
        seats = office.alarm.wait(seats).unwrap();
        info!("Tutor is awaken by a waiting student");
        for student in seats.drain(..) {
            thread::sleep(Duration::from_millis(TUTOR_TIME));
            student.unpark();
            info!("Tutor helped student {:?}", student.id());
        }
    }
}
