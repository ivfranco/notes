pub fn gcd(seq: &[i32]) -> i32 {
    assert!(!seq.is_empty());

    seq.iter()
        .skip(1)
        .fold(seq[0].abs(), |g, n| gcd_euclid(g, n.abs()))
}

fn gcd_euclid(a: i32, b: i32) -> i32 {
    if a < b {
        gcd_euclid(b, a)
    } else if b == 0 {
        a
    } else {
        gcd_euclid(b, a % b)
    }
}
