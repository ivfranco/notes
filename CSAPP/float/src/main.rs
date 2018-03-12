#![allow(dead_code)]

extern crate rand;

use std::mem;
use std::f32;
use rand::Rng;

fn main() {
    let mut res = true;
    for _ in 0..100 {
        res = res && practice_2_95();
    }
    println!("all test passed: {}", res);
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
    let p_byte = p as *const T as *const u8;
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
        let p_byte = &mask as *const u32 as *const u8;
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

    // let xsra = unsafe { mem::transmute::<isize, usize>(mem::transmute::<usize, isize>(x) >> k) };
    let xsra = (x as isize >> k) as usize;

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

    // let xsrl = unsafe { mem::transmute::<usize, isize>(mem::transmute::<isize, usize>(x) >> k) };
    let xsrl = (x as usize >> k) as isize;

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

fn odd_ones(x: u32) -> bool {
    let mut y = x;
    y ^= y >> 16;
    y ^= y >> 8;
    y ^= y >> 4;
    y ^= y >> 2;
    y ^= y >> 1;
    y & 1 == 1
}

fn practice_2_65() {
    let x: u32 = rand::random();
    println!("{:b}, {} == {}", x, x.count_ones() % 2 == 1, odd_ones(x));
}

fn leftmost_one(x: u32) -> u32 {
    let mut y = x;
    y |= y >> 1;
    y |= y >> 2;
    y |= y >> 4;
    y |= y >> 8;
    y |= y >> 16;

    ((y >> 1) + 1) & x
}

fn practice_2_66() {
    let x: u32 = rand::random();
    println!("{:032b}\n{:032b}\n", x, leftmost_one(x));
}

fn lower_one_mask(n: usize) -> usize {
    let w = mem::size_of::<usize>() << 3;
    assert!(1 <= n && n <= w);

    let mut y = 1usize;
    y = y << (n - 1);
    ((y - 1) << 1) | 1
}

fn practice_2_68() {
    for i in 1..65 {
        println!("{:064b}", lower_one_mask(i));
    }
}

fn rotate_left(x: usize, n: usize) -> usize {
    let w = mem::size_of::<usize>() << 3;
    assert!(n < w);

    let higher_bits = x >> (w - n - 1) >> 1;
    (x << n) | higher_bits
}

fn practice_2_69() {
    let mut rnd = rand::thread_rng();
    let x = rnd.gen();
    let n = rnd.gen_range(0, mem::size_of::<usize>() * 8);
    println!("{:016X}, {}", x, n);
    println!("{:016X}", rotate_left(x, n));
    println!("{:016X}\n", x.rotate_left(n as u32));
}

fn fit_bits(x: isize, n: usize) -> bool {
    let w = mem::size_of::<isize>() << 3;
    assert!(1 <= n && n <= w);

    // would be equivalent if inlined the function body of lower_one_mask, no function call
    // let lower_mask = unsafe { mem::transmute::<usize, isize>(lower_one_mask(n)) };
    let lower_mask = lower_one_mask(n) as isize;
    // either x is an n-bit negative number extended to w bits (i.e. higher (w - n) bits all equal to 1)
    // or x is an (n-1) bit nonnegative number extended to w bits (i.e. higher (w - n + 1) bits all equal to 0)
    !(x | lower_mask) == 0 || (x & !(lower_mask >> 1)) == 0
}

fn practice_2_70() {
    fn nbit_min_max(n: usize) -> (isize, isize) {
        assert!(n <= mem::size_of::<isize>() * 8 - 1);
        let max = (1isize << n).wrapping_sub(1);
        let min = (1isize << n).wrapping_neg();
        (min, max)
    }

    fn arith_fit_bits(x: isize, n: usize) -> bool {
        if n >= mem::size_of::<isize>() * 8 {
            return true;
        }
        let (min, max) = nbit_min_max(n);
        min <= x && x <= max
    }

    let w = mem::size_of::<isize>() * 8;
    let mut rnd = rand::thread_rng();
    let n = rnd.gen_range(1, w);
    let k = rnd.gen_range(1, w);
    let (min, max) = nbit_min_max(k);
    let x = rnd.gen_range(min, max);
    println!("{:016x}, {}", x, n);
    println!("{} == {}", fit_bits(x, n), arith_fit_bits(x, n));
}

fn saturating_add(x: isize, y: isize) -> isize {
    let mut sum = x.wrapping_add(y);
    // has msb 1; other bits are 0
    let min = isize::min_value();
    // has msb 0; other bits are 1
    let max = !min;
    let w = mem::size_of::<isize>() << 3;

    let msb_x = x & min;
    let msb_y = y & min;
    let msb_s = sum & min;

    // a number has msb 1 if the addition overflowed, 0 otherwise; other bits are 0
    let overflowed = !msb_x & !msb_y & msb_s;
    // a number has msb 1 if the addition underflowed, 0 otherwise; other bits are 0
    let underflowed = msb_x & msb_y & !msb_s;

    // a number has msb 0 if addition overflowed, 1 otherwise; other bits are 1
    let overflow_msb_mask = !overflowed;
    sum &= overflow_msb_mask;
    // a number has all bits except msb equal to 1 if overflowed, 0 otherwise; msb is 0
    let overflow_rest_mask = overflowed >> (w - 1) & max;
    sum |= overflow_rest_mask;

    // a number has msb 1 if addition underflowed, 0 otherwise; other bits are 0
    let underflow_msb_mask = underflowed;
    sum |= underflow_msb_mask;
    // a number has all bits except msb equal to 0 if underflowed, 1 otherwise; msb is 1
    let underflow_rest_mask = !(underflowed >> (w - 1)) | min;
    sum &= underflow_rest_mask;

    sum
}

fn practice_2_73() {
    let x: isize = rand::random();
    let y: isize = rand::random();
    let lib_res = x.saturating_add(y);
    let usr_res = saturating_add(x, y);
    println!("{} == {}, {}", lib_res, usr_res, lib_res == usr_res);
}

fn unsigned_high_prod(x: u32, y: u32) -> u32 {
    fn signed_high_prod(x: i32, y: i32) -> i32 {
        let p = (x as i64) * (y as i64);
        // let low_and_high = unsafe { mem::transmute::<i64, [i32; 2]>(p) };
        // low_and_high[1]
        ((p >> 32) & 0xffffffff) as i32
    }

    let sig_x = if x > i32::max_value() as u32 { 1 } else { 0 };
    let sig_y = if y > i32::max_value() as u32 { 1 } else { 0 };
    // unsafe {
    //     let tx = mem::transmute::<u32, i32>(x);
    //     let ty = mem::transmute::<u32, i32>(y);
    //     let tp = mem::transmute::<i32, u32>(signed_high_prod(tx, ty));
    //     tp.wrapping_add(sig_x * y).wrapping_add(sig_y * x)
    // }
    let tp = signed_high_prod(x as i32, y as i32) as u32;
    tp.wrapping_add(sig_x * y).wrapping_add(sig_y * x)
}

fn practice_2_75() -> bool {
    fn arith_high_prod(x: u32, y: u32) -> u32 {
        let p = (x as u64) * (y as u64);
        let low_and_high = unsafe { mem::transmute::<u64, [u32; 2]>(p) };
        low_and_high[1]
    }

    let x = rand::random();
    let y = rand::random();
    let arith_prod = arith_high_prod(x, y);
    let usr_prod = unsigned_high_prod(x, y);
    println!("{}\n{}, {}\n", arith_prod, usr_prod, arith_prod == usr_prod);
    arith_prod == usr_prod
}

fn divide_power2(x: isize, k: usize) -> isize {
    let w = mem::size_of::<isize>() << 3;
    assert!(k < w);

    // a number that has 1 at k-th position, other bits are 0; mask == 0 when k == 0
    let mask = (1 << k) >> 1;
    // Rust do not have C-style ! operator
    // simulating int bias = !!(x & mask) with conditions
    let bias = if x & mask == 0 { 0 } else { 1 };
    let sig_x = x & isize::min_value();
    let sig_mask = ((bias << (w - 1)) & sig_x) >> (w - 2);

    (x >> k) + (bias | sig_mask)
}

fn practice_2_77() {
    let mut rnd = rand::thread_rng();
    let x = rnd.gen();
    let k = rnd.gen_range(0, mem::size_of::<isize>() * 8);
    println!(
        "{:064b}\n{:064b}\n{:064b}\n",
        x,
        x >> k,
        divide_power2(x, k)
    );
}

fn mul3div4(x: isize) -> isize {
    ((x << 1) + x) >> 2
}

fn threefourths(x: isize) -> isize {
    let higher = x & !0b11;
    let lower = x & 0b11;

    let tf_higher = (higher >> 2) + (higher >> 1);
    let tf_lower = ((lower << 1) + lower) >> 2;

    // if ((lower << 1) + lower) & 0b11 != 0 and x < 0, tf_higher + tf_lower is one less than the result
    // simulating !!(x & INT_MIN)
    let sig_x = if x & isize::min_value() == 0 { 0 } else { 1 };
    // simulating !!(((lower << 1) + lower) & 3)
    let rounded = if ((lower << 1) + lower) & 0b11 == 0 {
        0
    } else {
        1
    };
    let bias = sig_x & rounded;

    tf_higher + tf_lower + bias
}

fn practice_2_79() {
    assert!(threefourths(8) == 6);
    assert!(threefourths(9) == 6);
    assert!(threefourths(10) == 7);
    assert!(threefourths(11) == 8);
    assert!(threefourths(12) == 9);

    assert!(threefourths(-8) == -6);
    assert!(threefourths(-9) == -6);
    assert!(threefourths(-10) == -7);
    assert!(threefourths(-11) == -8);
    assert!(threefourths(-12) == -9);
}

type FloatBits = u32;

const SIG_OFFSET: FloatBits = 31;
const EXP_OFFSET: FloatBits = 23;
const EXP_MASK: FloatBits = 0xff;
const FRAC_MASK: FloatBits = (1 << 23) - 1;
const EXP_BIAS: i32 = (1 << (8 - 1)) - 1;

fn decompose(f: FloatBits) -> (u32, u32, u32) {
    let sig = f >> SIG_OFFSET;
    let exp = (f >> EXP_OFFSET) & EXP_MASK;
    let frac = f & FRAC_MASK;

    (sig, exp, frac)
}

fn compose(sig: u32, exp: u32, frac: u32) -> FloatBits {
    assert!(sig <= 1);
    assert!(exp <= EXP_MASK);
    assert!(frac <= FRAC_MASK);
    (sig << SIG_OFFSET) | (exp << EXP_OFFSET) | frac
}

fn is_nan(f: FloatBits) -> bool {
    let (_, exp, frac) = decompose(f);
    exp == EXP_MASK && frac != 0
}

fn float_negate(f: FloatBits) -> FloatBits {
    if is_nan(f) {
        return f;
    }
    f ^ (1 << SIG_OFFSET)
}

fn float_bits_op<F: Fn(FloatBits) -> FloatBits>(op: F, x: f32) -> f32 {
    unsafe {
        let f = mem::transmute::<f32, FloatBits>(x);
        mem::transmute::<FloatBits, f32>(op(f))
    }
}

fn test_framework<F, G>(lib: F, usr: G) -> bool
where
    F: Fn(f32) -> f32,
    G: Fn(FloatBits) -> FloatBits,
{
    let x: f32 = rand::random();
    let f = float_bits_op(usr, x);
    let res = lib(x) == f;
    println!("{} == {}, {}", lib(x), f, res);
    res
}

fn practice_2_91() -> bool {
    test_framework(|x| -x, float_negate)
}

fn float_absval(f: FloatBits) -> FloatBits {
    if is_nan(f) {
        return f;
    }
    f & (0xffffffff >> 1)
}

fn practice_2_92() -> bool {
    test_framework(f32::abs, float_absval)
}

fn float_twice(f: FloatBits) -> FloatBits {
    if is_nan(f) {
        return f;
    }

    let (sig, exp, frac) = decompose(f);
    if exp == EXP_MASK - 1 {
        compose(sig, EXP_MASK, 0)
    } else if exp == 0 {
        if frac >> (EXP_OFFSET - 1) == 0 {
            compose(sig, 0, frac << 1)
        } else {
            compose(sig, 1, (frac << 1) & FRAC_MASK)
        }
    } else {
        compose(sig, exp + 1, frac)
    }
}

fn practice_2_93() -> bool {
    test_framework(|x| x * 2.0, float_twice)
}

fn float_half(f: FloatBits) -> FloatBits {
    if is_nan(f) {
        return f;
    }

    let (sig, exp, mut frac) = decompose(f);
    if exp > 1 {
        return compose(sig, exp - 1, frac);
    }

    let lsb = frac & 1;
    frac >>= 1;
    if lsb == 1 && frac & 1 == 1 {
        frac += 1;
    }
    if exp == 1 {
        frac |= 1 << (EXP_OFFSET - 1);
    }

    compose(sig, 0, frac)
}

fn practice_2_94() -> bool {
    test_framework(|x| x * 0.5, float_half)
}

fn float_f2i(f: FloatBits) -> i32 {
    let w = (mem::size_of::<i32>() << 3) as u32;
    let err = i32::min_value();
    if is_nan(f) {
        return err;
    }
    let (sig, exp, mut frac) = decompose(f);
    let e = if exp == 0 {
        1 - EXP_BIAS
    } else {
        (exp as i32) - EXP_BIAS
    };

    // too small to be represented as an integer
    if e < 0 {
        return 0;
    }

    // overflow and underflow
    // for the special case where sig == 1, e == w - 1, frac == 0
    // V == -2^(w - 1) == INT_MIN == ERR
    // by returning ERR, this special case is covered
    if e > (w as i32) - 2 {
        return err;
    }

    frac |= 1 << EXP_OFFSET;
    frac <<= w - EXP_OFFSET - 1;
    frac >>= (w as i32) - e - 1;

    let s = if sig == 0 { 1 } else { -1 };
    return (frac as i32) * s;
}

fn practice_2_95() -> bool {
    let mut rnd = rand::thread_rng();
    let x: f32 = rnd.gen_range(-2e9, 2e9);
    let f = unsafe { mem::transmute(x) };
    let int_f = float_f2i(f);
    let res = (x as i32) == int_f;
    println!("{} == {}, {}", x as i32, int_f, res);
    res
}
