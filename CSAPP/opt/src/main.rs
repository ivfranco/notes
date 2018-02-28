#![allow(dead_code)]

extern crate simd;

#[cfg(feature = "nightly")]
use simd::f32x4;
#[cfg(feature = "avx")]
use simd::x86::avx::f64x4;

fn main() {
    problem_5_21();
}

type data_t = f32;
const IDENT: data_t = 1 as data_t;
#[inline(always)]
fn op(a: data_t, b: data_t) -> data_t {
    a * b
}

fn combine5(v: &[data_t], dest: &mut data_t) {
    let mut i = 0;
    let length = v.len() as isize;
    let limit = length - 4;
    let data: *const data_t = v.as_ptr();
    let mut acc = IDENT;

    unsafe {
        while i < limit {
            let mut t = op(*data.offset(i), *data.offset(i + 1));
            t = op(t, *data.offset(i + 2));
            t = op(t, *data.offset(i + 3));
            t = op(t, *data.offset(i + 4));
            acc = op(acc, t);
            i += 5;
        }

        while i < length {
            acc = op(acc, *data.offset(i));
            i += 1;
        }
    }

    *dest = acc;
}

fn merge(src1: &mut [i32], src2: &mut [i32], dest: &mut [i32], n: usize) {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut id = 0;

    while i1 < n && i2 < n {
        let (min, i1_inc, i2_inc) = if src1[i1] < src2[i2] {
            (src1[i1], 1, 0)
        } else {
            (src2[i2], 0, 1)
        };
        dest[id] = min;
        i1 += i1_inc;
        i2 += i2_inc;
        id += 1;
    }

    while i1 < n {
        dest[id] = src1[i1];
        id += 1;
        i1 += 1;
    }
    while i2 < n {
        dest[id] = src2[i2];
        id += 1;
        i2 += 1;
    }
}

fn psum1(a: &[f32], p: &mut [f32], n: usize) {
    let mut i = 1;
    let mut acc = a[0];
    p[0] = acc;
    while i < n {
        acc += a[i];
        p[i] = acc;
        i += 1;
    }
}

fn inner4_unroll(u: &[data_t], v: &[data_t], dest: &mut data_t) {
    let length = u.len() as isize;
    let limit = length - 3;
    let udata = u.as_ptr();
    let vdata = v.as_ptr();
    let mut sum = 0 as data_t;
    let mut i = 0;

    unsafe {
        while i < limit {
            let t1 = *udata.offset(i) * *vdata.offset(i);
            let t2 = *udata.offset(i + 1) * *vdata.offset(i + 1);
            let t3 = *udata.offset(i + 2) * *vdata.offset(i + 2);
            let t4 = *udata.offset(i + 3) * *vdata.offset(i + 3);
            sum += (t1 + t2) + (t3 + t4);
            i += 4;
        }

        while i < length {
            sum += *udata.offset(i) * *vdata.offset(i);
            i += 1;
        }
    }

    *dest = sum;
}

fn inner4_para(u: &[data_t], v: &[data_t], dest: &mut data_t) {
    let length = u.len() as isize;
    let limit = length - 3;
    let udata = u.as_ptr();
    let vdata = v.as_ptr();
    let mut sum1 = 0 as data_t;
    let mut sum2 = 0 as data_t;
    let mut sum3 = 0 as data_t;
    let mut sum4 = 0 as data_t;
    let mut i = 0;

    unsafe {
        while i < limit {
            sum1 += *udata.offset(i) * *vdata.offset(i);
            sum2 += *udata.offset(i + 1) * *vdata.offset(i + 1);
            sum3 += *udata.offset(i + 2) * *vdata.offset(i + 2);
            sum4 += *udata.offset(i + 3) * *vdata.offset(i + 3);
            i += 4;
        }

        sum1 += sum2 + sum3 + sum4;

        while i < length {
            sum1 += *udata.offset(i) * *vdata.offset(i);
            i += 1;
        }
    }

    *dest = sum1;
}

fn problem_5_17() {
    let data: Vec<f32> = (0..102).map(|u| u as f32).collect();
    let mut dest = 0f32;
    inner4_para(&data, &data, &mut dest);
    println!("{}", dest);
}

#[cfg(feature = "nightly")]
// currently will not compile on stable channel
fn inner4_simd(u: &[f32], v: &[f32], dest: &mut f32) {
    let length = u.len();
    let mut sumx4 = f32x4::splat(0f32);
    let mut sum = 0f32;
    let udata = u.as_ptr();
    let vdata = v.as_ptr();
    let mut i = 0;

    if length >= 4 {
        let limit = length - 3;
        while i < limit {
            // approximated CPE: 1/4 of simd integer addition latency
            sumx4 = sumx4 + f32x4::load(u, i) * f32x4::load(v, i);
            i += 4;
        }
        for j in 0..4 {
            sum += sumx4.extract(j);
        }
    }

    unsafe {
        while i < length {
            sum += *udata.offset(i as isize) * *vdata.offset(i as isize);
            i += 1;
        }
    }

    *dest = sum;
}

#[cfg(feature = "nightly")]
fn problem_5_18() {
    let data: Vec<f32> = (0..102).map(|u| u as f32).collect();
    let mut dest = 0f32;
    inner4_simd(&data, &data, &mut dest);
    println!("{}", dest);
}

unsafe fn memset(s: *mut u8, c: i32, n: usize) {
    // std rust doesn't have void pointers, use *mut u8 (which is unsigned char * in C) instead
    use std::mem;

    let mut cnt = 0;
    let k = mem::size_of::<usize>();

    // sign extend c to k bits, the lower 8 bits will not change
    let mut c_usize: usize = mem::transmute(c as isize);
    // set all but the lower 8 bits to 0
    c_usize &= 0xff;
    let c_u8 = c_usize as u8;
    // c_usize will be the lower 8 bits of c repeated k times
    for _ in 0..k {
        c_usize = c_usize | (c_usize << 8);
    }

    // n % k == 0 iff n & (k-1) == 0 for k = 2^m
    let mask = k - 1;

    // write byte by byte until the address is aligned to multiple of k
    while s.offset(cnt as isize) as usize & mask != 0 && cnt < n {
        *s.offset(cnt as isize) = c_u8;
        cnt += 1;
    }

    let s_usize = s.offset(cnt as isize) as *mut usize;
    // compute the reminding n
    // won't underflow, the loop condition above limits cnt <= n
    let rem_n = n - cnt;
    // reset counter
    cnt = 0;
    let mut cnt_usize = 0;

    if rem_n >= k {
        // otherwise rem_n - k will underflow
        let limit = rem_n - k;
        while cnt < limit {
            // only index and its increment are on the critial path
            *s_usize.offset(cnt_usize) = c_usize;
            cnt += k;
            cnt_usize += 1;
        }
    }

    let s_u8 = s_usize as *mut u8;

    while cnt < rem_n {
        *s_u8.offset(cnt as isize) = c_u8;
        cnt += 1;
    }
}

// unsafe, full of UB if something went wrong
fn problem_5_19() {
    const N: usize = 59;
    let mut src: [u8; N] = [0; N];
    let c: i32 = 0xfefdaa;
    unsafe {
        memset((&mut src).as_mut_ptr(), c, N / 2);
    }
    for (i, byte) in src.iter().enumerate() {
        print!("{}:{:02X} ", i, byte);
    }
    println!("");
}

// utilizes avx instruction sets, which can compute 4 double-precision floating point operations as a single instruction
#[cfg(feature = "avx")]
fn poly_avx(a: &[f64], x: f64, degree: usize) -> f64 {
    let mut res = a[0];
    let mut i = 1;
    let mut xpwr = x;

    if degree >= 4 {
        let mut resx4 = f64x4::splat(0f64);
        let mut xpwrx4 = f64x4::new(x, x.powi(2), x.powi(3), x.powi(4));
        let x4x4 = f64x4::splat(x.powi(4));
        // len(a) == degree + 1
        // limit == len(a) - 3 == degree - 2
        let limit = degree - 2;
        while i < limit {
            // approximated CPE: 1/4 of avx 4x64-bit floating-point addition latency
            resx4 = resx4 + xpwrx4 * f64x4::load(a, i);
            xpwrx4 = xpwrx4 * x4x4;
            i += 4;
        }
        for j in 0..4 {
            res += resx4.extract(j);
        }
        xpwr = xpwrx4.extract(0);
    }

    while i < degree {
        res += xpwr * a[i];
        xpwr *= x;
        i += 1;
    }

    res
}

#[cfg(feature = "avx")]
fn problem_5_20() {
    const N: usize = 1003;
    let a: Vec<f64> = vec![1f64; N];
    println!("{} == 2", poly_avx(&a, 0.5, N - 1));
}

fn psum_unroll(a: &[f32], p: &mut [f32], n: usize) {
    let mut i = 0;
    let mut acc = 0f32;
    if n >= 4 {
        let limit = n - 3;
        let mut acc1;
        let mut acc2;
        let mut acc3;
        let mut acc4 = 0f32;
        while i < limit {
            // 4-way unroll, performs 10 floating-point additions every iteration
            let a1 = a[i];
            let a2 = a[i + 1];
            let a3 = a[i + 2];
            let a4 = a[i + 3];
            acc1 = acc4 + a1;
            acc2 = acc4 + (a1 + a2);
            acc3 = acc4 + (a1 + a2 + a3);
            acc4 += a1 + a2 + a3 + a4;
            p[i] = acc1;
            p[i + 1] = acc2;
            p[i + 2] = acc3;
            p[i + 3] = acc4;
            i += 4;
        }
        acc = acc4;
    }

    while i < n {
        acc += a[i];
        p[i] = acc;
        i += 1;
    }
}

fn problem_5_21() {
    const N: usize = 103;
    let a: Vec<f32> = (0..N).map(|u| u as f32).collect();
    let mut p: Vec<f32> = vec![0f32; N];
    psum_unroll(&a, &mut p, N);
    for (i, sum) in p.iter().enumerate() {
        let arith_sum = i * (i + 1) / 2;
        println!("{}, {} == {}", arith_sum == *sum as usize, sum, arith_sum);
    }
}
