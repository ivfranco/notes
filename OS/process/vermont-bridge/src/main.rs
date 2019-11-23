use rand::random;
use std::{env, process, thread};
use vermont_bridge::{north_villager, south_villager, BridgeLock};

fn main() {
    let mut args = env::args();
    args.next();

    let villagers = args
        .next()
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: EXER N_VILLAGERS");
            process::exit(1);
        });

    let lock = BridgeLock::new();
    let mut handles = vec![];

    for _ in 0..villagers {
        let from_south = random();
        let clone = lock.clone();
        let handle = thread::spawn(move || {
            if from_south {
                south_villager(&clone)
            } else {
                north_villager(&clone)
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
