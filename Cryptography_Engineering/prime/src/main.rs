extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;

use num_bigint::BigUint;
use num_bigint::RandBigInt;
use num_integer::Integer;
use num_traits::One;
use std::time::Instant;

fn small_prime_list(n: usize) -> Vec<usize> {
    assert!(2 <= n && n <= 1 << 20);

    let mut flags: Vec<bool> = vec![true; n];
    flags[0] = false;
    let mut i = 2;
    while i * i <= n {
        for j in 2..(n / i) + 1 {
            flags[i * j - 1] = false;
        }
        i += 1;
        while !flags[i - 1] {
            i += 1;
        }
    }

    flags
        .iter()
        .enumerate()
        .filter(|&(_, f)| *f)
        .map(|(i, _)| i + 1)
        .collect()
}

fn prime_list_timing(n: usize) -> u64 {
    let now = Instant::now();
    small_prime_list(n);
    let elapsed = now.elapsed();
    elapsed.as_secs() * 1000 + (elapsed.subsec_nanos() / 1000) as u64
}

fn problem_10_1() {
    let mut times = [0; 20];
    for i in 0..20 {
        let mut sum = 0;
        for _ in 0..5 {
            sum += prime_list_timing(1 << (i + 1));
        }
        times[i] = sum / 5;
    }
    println!("{:?}", times);
}

fn problem_10_2() {
    let nums: Vec<u64> = vec![13635, 16060, 8190, 21363];
    let modulo = 29101;
    let mod1 = nums.iter().sum::<u64>() % modulo;
    let mod2 = nums.iter().fold(0, |sum, n| (sum + n) % modulo);
    println!("{} = {}", mod1, mod2);
}

fn problem_10_3() {
    let nums: Vec<u64> = vec![12358, 1854, 14303];
    let modulo = 29101;
    let mod1 = nums.iter().product::<u64>() % modulo;
    let mod2 = nums.iter().fold(1, |product, n| (product * n) % modulo);
    println!("{} = {}", mod1, mod2);
}

fn extended_gcd(a: u32, b: u32) -> (u32, i64, i64) {
    let (mut c, mut d) = (a, b);
    let (mut uc, mut vc, mut ud, mut vd) = (1, 0, 0, 1);
    let mut cd: [u32; 2] = [0; 2];
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

fn gcd(a: u32, b: u32) -> u32 {
    extended_gcd(a, b).0
}

fn problem_10_5() {
    println!("{}", gcd(91261, 117035));
}

fn problem_10_6() {
    let (_, u, _) = extended_gcd(74, 167);
    println!("{}", u % 167);
}

fn is_prime(n: &BigUint) -> bool {
    assert!(n >= &BigUint::from(3u64));

    if n.is_even() {
        return false;
    }
    for i in 1..500u64 {
        if n.is_multiple_of(&BigUint::from(i * 2 + 1)) {
            return false;
        }
    }

    return robin_miller(n);
}

fn robin_miller(n: &BigUint) -> bool {
    assert!(n >= &BigUint::from(3u32));
    assert!(n.is_odd());

    let (mut s, mut t) = (n - 1u32, 0usize);
    while s.is_even() {
        s /= 2u32;
        t += 1;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..64 {
        let a: BigUint = rng.gen_biguint_range(&BigUint::from(2u32), n);
        let mut v: BigUint = a.modpow(&s, n);
        if v == BigUint::one() {
            continue;
        }
        let mut i = 0;
        while v != n - 1u32 {
            if i >= t - 1 {
                return false;
            }
            v = v.modpow(&BigUint::from(2u32), n);
            i += 1;
        }
    }

    true
}

fn generate_large_prime(l: &BigUint, u: &BigUint) -> Result<BigUint, ()> {
    assert!(l >= &BigUint::from(3u32));
    assert!(u >= l);

    let r = u.bits() * 100;
    let mut rng = rand::thread_rng();

    for _ in 0..r {
        let n = rng.gen_biguint_range(&l, &u);
        if is_prime(&n) {
            return Ok(n);
        }
    }

    Err(())
}

fn problem_10_7() {
    let l = BigUint::one() << 255;
    let u = BigUint::one() << 256;
    if let Ok(n) = generate_large_prime(&l, &u) {
        println!("{}", n);
    } else {
        println!("generation error");
    }
}

fn mod_exp(base: &BigUint, exp: &BigUint, modulo: &BigUint) -> BigUint {
    let mut n = BigUint::one();
    let mut cnt = 0;
    for bit in exp.to_str_radix(2).chars() {
        n = &n * &n % modulo;
        cnt += 1;
        if bit == '1' {
            n = (&n * base) % modulo;
            cnt += 1;
        }
    }
    println!("Performed {} multiplications", cnt);
    n
}

fn problem_10_9() {
    let base = BigUint::from(27u32);
    let exp = BigUint::from(35u32);
    let modulo = BigUint::from(569u32);

    println!(
        "{}^{} mod {} = {} = {}",
        base,
        exp,
        modulo,
        mod_exp(&base, &exp, &modulo),
        base.modpow(&exp, &modulo)
    );
}

fn main() {
    problem_10_1();
    problem_10_2();
    problem_10_3();
    problem_10_5();
    problem_10_6();
    problem_10_7();
    problem_10_9();
}
