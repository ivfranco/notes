fn main() {
    problem_12_1();
    problem_12_2();
    problem_12_3();
    problem_12_4();
    problem_12_5();
    problem_12_8();
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut c, mut d) = (a, b);
    let (mut uc, mut vc, mut ud, mut vd) = (1, 0, 0, 1);
    let mut cd: [i64; 2] = [0; 2];
    let mut uv: [i64; 4] = [0; 4];
    while c != 0 {
        let q = d / c;
        cd[0] = d - q * c;
        cd[1] = c;
        c = cd[0];
        d = cd[1];

        uv[0] = ud - (q as i64) * uc;
        uv[1] = vd - (q as i64) * vc;
        uv[2] = uc;
        uv[3] = vc;
        uc = uv[0];
        vc = uv[1];
        ud = uv[2];
        vd = uv[3];
    }
    (d, ud, vd)
}

fn gcd(a: i64, b: i64) -> i64 {
    extended_gcd(a, b).0
}

fn modulo(x: i64, p: i64) -> i64 {
    (x % p + p) % p
}

fn inverse(x: i64, p: i64) -> i64 {
    let (g, u, _) = extended_gcd(x, p);
    assert!(g == 1);
    modulo(u, p)
}

fn garner(a: i64, b: i64, p: i64, q: i64) -> i64 {
    let inv_q = inverse(q, p);
    modulo((a - b) * inv_q, p) * q + b
}

fn problem_12_1() {
    let (p, q, a, b) = (89, 107, 3, 5);
    let x = garner(a, b, p, q);
    println!("x = {}, x mod p = {}, x mod q = {}", x, x % p, x % q);
}

fn crt_op<F: Fn(i64, i64) -> i64>(x: i64, y: i64, p: i64, q: i64, f: F) -> i64 {
    let (xp, xq) = (x % p, x % q);
    let (yp, yq) = (y % p, y % q);
    garner(f(xp, yp) % p, f(xq, yq) % q, p, q)
}

fn crt_add(x: i64, y: i64, p: i64, q: i64) -> i64 {
    crt_op(x, y, p, q, |x, y| x + y)
}

fn crt_mul(x: i64, y: i64, p: i64, q: i64) -> i64 {
    crt_op(x, y, p, q, |x, y| x * y)
}

fn problem_12_2() {
    let (p, q, x, y) = (89, 107, 1796, 8931);
    println!("{} == {}", (x + y) % (p * q), crt_add(x, y, p, q));
}

fn problem_12_3() {
    let (p, q, x, y) = (89, 107, 1796, 8931);
    println!("{} == {}", x * y % (p * q), crt_mul(x, y, p, q));
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn report_pk_validity(p: i64, q: i64, e: i64) {
    let l = lcm(p - 1, q - 1);
    let g = gcd(e, l);
    println!("gcd of e and lcm(p-1, q-1) is {}", g);
    if g == 1 {
        let d = inverse(e, l);
        println!("the corresponding secret key is {}", d);
    } else {
        println!("therefore e is not a valid public key");
    }
}

fn problem_12_4() {
    report_pk_validity(83, 101, 3);
}

fn problem_12_5() {
    report_pk_validity(79, 89, 3);
}

fn mod_exp(base: i64, exp: i64, modulo: i64) -> i64 {
    let mut n = 1;
    for bit in format!("{:b}", exp).chars() {
        n = (n * n) % modulo;
        if bit == '1' {
            n = (n * base) % modulo;
        }
    }
    n
}

fn problem_12_8() {
    let (p, q, e) = (71, 89, 3);
    let n = p * q;
    let d = inverse(e, lcm(p - 1, q - 1));
    let (m1, m2) = (5416, 2397);
    let (s1, s2) = (mod_exp(m1, d, n), mod_exp(m2, d, n));
    let m3 = (m1 * m2) % n;
    let s3 = mod_exp(m3, d, n);
    println!("σ1 = {}, σ2 = {}", s1, s2);
    println!("σ1 * σ2 = {} = σ3 = {}", s1 * s2 % n, s3);
}
