#![allow(dead_code)]

extern crate rand;
extern crate scoped_threadpool;

mod echo;
mod prethread;
mod tfgets;

use std::env;
use std::thread;
use std::time;
use scoped_threadpool::Pool;
// use echo::echo_server;
// use prethread::echo_server;

fn main() {
    problem_12_34();
}

fn problem_12_16() {
    let n: usize = env::args()
        .nth(1)
        .expect("First argument missing")
        .parse()
        .expect("Invalid unsigned integer");

    let mut thread_vec = Vec::with_capacity(n);
    for i in 0..n {
        thread_vec.push(thread::spawn(move || {
            println!("Thread {} spawned", i);
        }));
    }
    for (i, handle) in thread_vec.into_iter().enumerate() {
        handle.join().expect("Join error");
        println!("Thread {} reaped", i);
    }
}

fn problem_12_31() {
    use tfgets::tfgets;

    match tfgets() {
        Ok(line) => println!("{}", line),
        Err(e) => eprintln!("{}", e),
    }
}

const N: usize = 512;
const M: usize = 256;
const NTHREADS: usize = 16;
const CHUNK_SIZE: usize = N / NTHREADS;

fn matrix_mul(lhs: &[[f64; M]], rhs: &[[f64; N]], ret: &mut [[f64; N]]) {
    for r in 0..N {
        for c in 0..N {
            for k in 0..M {
                ret[r][c] += lhs[r][k] * rhs[k][c];
            }
        }
    }
}

fn matrix_mul_para(lhs: &[[f64; M]], rhs: &[[f64; N]], ret: &mut [[f64; N]]) {
    let mut thread_pool = Pool::new(NTHREADS as u32);
    thread_pool.scoped(|scope| {
        for (idx, chunk) in ret.chunks_mut(CHUNK_SIZE).enumerate() {
            let row_offset = CHUNK_SIZE * idx;
            scope.execute(move || {
                for r in 0..CHUNK_SIZE {
                    for c in 0..N {
                        for k in 0..M {
                            chunk[r][c] += lhs[r + row_offset][k] * rhs[k][c];
                        }
                    }
                }
            });
        }
    });
}

fn report_time<F: FnMut()>(name: &str, mut f: F) {
    let now = time::Instant::now();
    f();
    let elasped = now.elapsed();
    println!(
        "{} consumed {}ms",
        name,
        elasped.subsec_nanos() as u64 / 1000 + elasped.as_secs() * 1000
    );
}

fn problem_12_34() {
    let lhs = [[rand::random(); M]; N];
    let rhs = [[rand::random(); N]; M];
    let mut ret = [[0f64; N]; N];
    let mut ret_para = [[0f64; N]; N];

    report_time("serial multiplication", || matrix_mul(&lhs, &rhs, &mut ret));
    report_time("parallel multiplication", || {
        matrix_mul_para(&lhs, &rhs, &mut ret_para)
    });

    for r in 0..N {
        for c in 0..N {
            if ret[r][c] != ret_para[r][c] {
                println!("Parallel version incorrect");
                ::std::process::exit(1);
            }
        }
    }
}
