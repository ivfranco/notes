use std::{sync::Arc, thread, time::Duration};

use semaphore::Semaphore;

fn main() {
    let semaphore = Arc::new(Semaphore::new(0));

    println!("parent: begin");

    {
        let s = Arc::clone(&semaphore);
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            println!("child");
            s.post().unwrap();
        })
    };

    semaphore.wait().unwrap();
    println!("parent: end");
}
