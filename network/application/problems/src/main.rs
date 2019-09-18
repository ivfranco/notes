fn main() {
    problem_22();
}

const K: f64 = 1024f64;
const M: f64 = K * K;
const G: f64 = M * K;

fn max_f64(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}

fn problem_22() {
    println!("\nP22");

    fn dcs(n: u32, f: f64, us: f64, d: f64) -> f64 {
        max_f64(n as f64 * f / us, f / d)
    }

    fn dp2p(n: u32, f: f64, us: f64, u: f64, d: f64) -> f64 {
        [f / us, f / d, n as f64 * f / (us + n as f64 * u)]
            .iter()
            .fold(0.0, |a, &b| max_f64(a, b))
    }

    let f = 15.0 * G;
    let us = 30.0 * M;
    let d = 2.0 * M;

    for &n in &[10, 100, 1000] {
        for &u in &[300.0 * K, 700.0 * K, 2.0 * M] {
            println!(
                "N = {}, u = {} bps, dcs = {:.3} secs, dp2p = {:.3} secs",
                n,
                u,
                dcs(n, f, us, d),
                dp2p(n, f, us, u, d)
            );
        }
    }
}
