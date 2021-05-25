use std::{sync::Arc, thread};

use semaphore::Semaphore;

fn main() {
    let s1 = Arc::new(Semaphore::new(0));
    let s2 = Arc::new(Semaphore::new(0));

    println!("parent: begin");

    let p1 = {
        let s1_local = Arc::clone(&s1);
        let s2_local = Arc::clone(&s2);

        thread::spawn(move || {
            println!("child 1: before");
            s2_local.post().unwrap();
            s1_local.wait().unwrap();
            println!("child 1: after");
        })
    };

    let p2 = {
        let s1_local = Arc::clone(&s1);
        let s2_local = Arc::clone(&s2);

        thread::spawn(move || {
            println!("child 2: before");
            s1_local.post().unwrap();
            s2_local.wait().unwrap();
            println!("child 2: after");
        })
    };

    p1.join().unwrap();
    p2.join().unwrap();

    println!("parent: end");
}
