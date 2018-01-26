extern crate crypto;
extern crate rand;

use crypto::digest::Digest;
use crypto::sha2::Sha512;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::Rng;

fn problem5_2() {
    let m = [
        0x48,
        0x65,
        0x6c,
        0x6c,
        0x6f,
        0x2c,
        0x20,
        0x77,
        0x6f,
        0x72,
        0x6c,
        0x64,
        0x2e,
        0x20,
        0x20,
        0x20,
    ];

    let mut sha = Sha512::new();
    sha.input(&m);
    println!("{}", sha.result_str());
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

fn find_collision(n: usize) -> ([u8; 32], [u8; 32]) {
    let mut sha = Sha512::new();
    let mut rnd = rand::thread_rng();
    let mut map = HashMap::new();
    let mut text: [u8; 32];
    let mut hash: String;
    let mut prev_text: [u8; 32];

    loop {
        text = rnd.gen::<[u8; 32]>();
        sha.reset();
        sha.input(&text);
        hash = sha.result_str();
        hash.truncate(n / 4);

        prev_text = *map.entry(hash).or_insert(text);
        if prev_text != text {
            return (prev_text, text);
        }
    }
}

fn collision_timing(n: usize) {
    println!("Looking for collisions in SHA-512-{}", n);
    let now = Instant::now();
    for _ in 0..5 {
        let (bytes1, bytes2) = find_collision(n);
        println!("{}\n{}", bytes_to_hex(&bytes1), bytes_to_hex(&bytes2));
    }
    println!("Found 5 collisions in {:?}", now.elapsed());
}

fn problem5_3() {
    for i in 1..7 {
        collision_timing(i * 8);
    }
}

fn find_preimage(target_hash: &[u8]) -> [u8; 32] {
    let mut rnd = rand::thread_rng();
    let mut sha = Sha512::new();
    let mut text: [u8; 32];
    let mut hash: [u8; 64] = [0; 64];

    loop {
        text = rnd.gen::<[u8; 32]>();
        sha.reset();
        sha.input(&text);
        sha.result(&mut hash);

        if hash.starts_with(target_hash) {
            return text;
        }
    }
}

fn preimage_timing(target_hash: &[u8]) {
    println!("Looking for preimage of {}", bytes_to_hex(target_hash));
    let now = Instant::now();
    for _ in 0..5 {
        println!("{}", bytes_to_hex(&find_preimage(target_hash)))
    }
    println!("Found 5 preimages in {:?}", now.elapsed());
}

fn problem5_4() {
    let hash1 = [0xa9];
    let hash2 = [0x3d, 0x4b];
    let hash3 = [0x3a, 0x7f, 0x27];
    let hash4 = [0xc3, 0xc0, 0x35, 0x7c];

    preimage_timing(&hash1);
    preimage_timing(&hash2);
    preimage_timing(&hash3);
    preimage_timing(&hash4);
}

fn main() {
    // problem5_2();
    // problem5_3();
    problem5_4();
}
