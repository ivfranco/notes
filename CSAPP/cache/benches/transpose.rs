#[macro_use]
extern crate bencher;
extern crate rand;

use bencher::Bencher;
use bencher::black_box;

fn transpose1(dst: &mut [i32], src: &[i32], dim: usize) {
    for i in 0..dim {
        for j in 0..dim {
            dst[j * dim + i] = src[i * dim + j];
        }
    }
}

fn transpose2(dst: &mut [i32], src: &[i32], dim: usize) {
    unsafe {
        let mut p_src = src.as_ptr();
        for i in 0..dim as isize {
            let mut p_dst = dst.as_mut_ptr().offset(i);
            for j in 0..dim as isize {
                // dst[j * dim + i] = src[i * dim + j];
                *p_dst = *p_src;
                p_dst = p_dst.offset(dim as isize);
                p_src = p_src.offset(1);
            }
        }
    }
}

fn transpose3(dst: &mut [i32], src: &[i32], dim: usize) {
    // assume dim is divisible by BLOCK
    const BLOCK: usize = 8;
    let limit = dim - BLOCK + 1;
    let mut x = 0;
    let mut y = 0;
    while x < limit {
        while y < limit {
            for i in x..x + BLOCK {
                for j in y..y + BLOCK {
                    dst[j * dim + i] = src[i * dim + j];
                }
            }
            y += BLOCK;
        }
        x += BLOCK;
    }
}

fn transpose_bench<F: Fn(&mut [i32], &[i32], usize)>(f: F, b: &mut Bencher) {
    const N: usize = 128;
    let mut dst = [0; N * N];
    let src = [rand::random(); N * N];
    b.iter(|| f(&mut dst, &black_box(src), N));
}

fn transpose_bench_ordinary(b: &mut Bencher) {
    transpose_bench(transpose1, b)
}

fn transpose_bench_rawptr(b: &mut Bencher) {
    transpose_bench(transpose2, b)
}

fn transpose_bench_blocking(b: &mut Bencher) {
    transpose_bench(transpose3, b)
}

benchmark_group!(
    benches,
    transpose_bench_ordinary,
    transpose_bench_rawptr,
    transpose_bench_blocking
);
benchmark_main!(benches);
