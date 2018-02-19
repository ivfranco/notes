#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

fn decode2(x: i32, y: i32, z: i32) -> i32 {
    let diff = y - z;
    let mut ret = diff << 31;
    ret >>= 31;
    ret ^ (x * diff)
}
