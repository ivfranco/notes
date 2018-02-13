#![allow(dead_code)]

extern crate rand;

use std::mem;
use rand::Rng;

fn main() {
    practice_2_63();
}

fn practice_2_47() {
    fn rep(x: u8) {
        let e = (x & 0b1100) >> 2;
        let bias = 1;
        let exp = if e == 0 { 1 - bias } else { e - bias };
        // E can never be negative in this scheme
        let two_e = 1 << exp;
        let frac = x & 0b11;
        let sig = if e == 0 { frac } else { frac + 4 };
        let decimal = (sig as f32) / (4 >> exp) as f32;

        println!("Bits == {:04b}", x);
        println!("e == {}", e);
        println!("E == {}", exp);
        println!("2^E == {}", two_e);
        println!("f == {}/4", frac);
        println!("M == {}/4", sig);
        println!("2^e * M == {}/{}", sig, 4 >> exp);
        println!("Decimal == {}\n", decimal);
    }

    for i in 0b0000..0b1100 {
        rep(i);
    }
}

unsafe fn show_bytes<T>(p: &T) {
    let p_byte = mem::transmute::<&T, *const u8>(p);
    let p_size = mem::size_of::<T>();
    for i in 0..p_size {
        print!("{:02X} ", *p_byte.offset(i as isize));
    }
    println!("");
}

fn practice_2_56() {
    unsafe {
        show_bytes(&0x12345678u32);
        show_bytes(&0x1122334455667788u64);
        show_bytes(&1e10f32);
    }
}

fn is_little_endian() -> bool {
    let mask = 0x0000ffffu32;
    unsafe {
        let p_byte = mem::transmute::<&u32, *const u8>(&mask);
        *p_byte == 0xff
    }
}

fn replace_byte(x: u32, i: usize, b: u8) -> u32 {
    assert!(i <= 3);
    let mask_x = !(0xff << (i * 8));
    let mask_b = (b as u32) << (i * 8);
    (mask_x & x) | mask_b
}

fn int_shifts_are_arithmetic() -> bool {
    let x = isize::min_value();
    x >> 1 < 0
}

fn srl(x: usize, k: usize) -> usize {
    let w = mem::size_of::<usize>() * 8;
    assert!(k <= w - 1);

    let xsra = unsafe { mem::transmute::<isize, usize>(mem::transmute::<usize, isize>(x) >> k) };

    let mut mask = 0;
    // computing a mask with k leading 1s followd by (w - k) 0s
    for _ in 0..k {
        mask *= 2;
        mask += 1;
    }
    for _ in 0..(w - k) {
        mask *= 2;
    }

    !mask & xsra
}

fn sra(x: isize, k: usize) -> isize {
    let w = mem::size_of::<isize>() * 8;
    assert!(k <= w - 1);

    let xsrl = unsafe { mem::transmute::<usize, isize>(mem::transmute::<isize, usize>(x) >> k) };

    // if the leading bit is 0 (which means x >= 0), logical and arithmetic shifting give the same result
    if x >= 0 {
        return xsrl;
    }

    let mut mask = 0;
    // computing the same mask in srl
    for _ in 0..k {
        mask *= 2;
        mask += 1;
    }
    for _ in 0..(w - k) {
        mask *= 2;
    }

    unsafe { mem::transmute::<usize, isize>(mask) | xsrl }
}

fn practice_2_63() {
    let mut rng = rand::thread_rng();
    let x = rng.gen();
    let y = rng.gen();
    let k = rng.gen_range(0, mem::size_of::<usize>());
    println!("{} == {}", srl(x, k), x >> k);
    println!("{} == {}", sra(y, k), y >> k);
}

fn any_odd_one(x: u32) -> bool {
    // assuming counting from the least significant bit to the most

    // a mask that have 1 at every odd position
    let mask = 0x55555555;
    mask & x != 0
}
