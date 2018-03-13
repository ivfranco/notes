mod echo;
mod prethread;

use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
// use echo::echo_server;
// use prethread::echo_server;

fn main() {
    problem_12_16();
}

fn problem_12_16() {
    let n: usize = env::args()
        .nth(1)
        .expect("First argument missing")
        .parse()
        .expect("Invalid unsigned integer");

    let mut thread_vec = Vec::with_capacity(n);
    for i in 0..n {
        thread_vec.push(thread::spawn(move || {
            println!("Thread {} spawned", i);
        }));
    }
    for (i, handle) in thread_vec.into_iter().enumerate() {
        handle.join();
        println!("Thread {} reaped", i);
    }
}
