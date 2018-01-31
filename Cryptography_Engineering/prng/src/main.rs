extern crate rand;

use rand::{OsRng, Rng};
use std::io;

fn problem_9_2() -> Result<(), io::Error> {
    let mut rng: OsRng = OsRng::new()?;
    // a 256-bit key
    let key: [u8; 32] = rng.gen();
    let key_str: String = key.iter().map(|b| format!("{:02X}", b)).collect();
    println!("{}", key_str);
    Ok(())
}

fn problem_9_5() -> Result<(), io::Error> {
    let mut rng: OsRng = OsRng::new()?;
    let mut n: u32 = 0;
    for _ in 0..32 {
        n <<= 1;
        if rng.gen::<bool>() {
            n += 1;
        }
    }
    println!("{:x}", n);
    Ok(())
}

fn problem_9_6() {
    let mut cnt = 0;
    let mut n: u8;
    let nSample = 10000;
    for _ in 0..nSample {
        n = rand::random::<u8>() % 192;
        if (n < 64) {
            cnt += 1
        };
    }

    println!(
        "expected probability of n < 64: {} / {}",
        nSample / 3,
        nSample
    );
    println!("observed probability of n < 64: {} / {}", cnt, nSample);
}

fn main() {
    problem_9_6();
}
