use std::num::Wrapping;

fn main() {
    problem_6_36();
}

fn log2(x: usize) -> usize {
    assert!(x > 0);

    let w = ::std::mem::size_of::<usize>() * 8;
    w - (x - 1).leading_zeros() as usize
}

struct CacheLine {
    valid: bool,
    tag: usize,
}

impl CacheLine {
    fn new() -> Self {
        CacheLine {
            valid: false,
            tag: 0,
        }
    }
}

// simulator of a direct-mapped cache, do not really contains data
struct DMCache {
    t: usize,
    s: usize,
    b: usize,
    sets: Vec<CacheLine>,
}

impl DMCache {
    fn new(m: usize, c: usize, b: usize) -> Self {
        let mut sets = vec![];
        let w = ::std::mem::size_of::<usize>() * 8;
        let s = c / b;
        for _ in 0..s {
            sets.push(CacheLine::new());
        }
        let log_s = log2(s);
        let log_b = log2(b);
        let t = m - log_s - log_b;

        DMCache {
            t,
            s: log_s,
            b: log_b,
            sets,
        }
    }

    fn decode(&self, addr: usize) -> (usize, usize, usize) {
        let w_addr = Wrapping(addr);
        let (t, s, b) = (self.t, self.s, self.b);
        let ones = Wrapping(usize::max_value());
        let ct_mask = !(ones << t) << (s + b);
        let ct = (w_addr & ct_mask) >> (s + b);
        let ci_mask = !(ones << s) << b;
        let ci = (w_addr & ci_mask) >> b;
        let co_mask = !(ones << b);
        let co = w_addr & co_mask;
        (ct.0, ci.0, co.0)
    }

    fn access(&mut self, addr: usize) -> bool {
        let (ct, ci, _) = self.decode(addr);
        let set = &mut self.sets[ci];
        if set.valid && set.tag == ct {
            true
        } else {
            set.valid = true;
            set.tag = ct;
            false
        }
    }
}

fn simulate_6_35_36(c: usize) {
    const S_INT: usize = 4;
    const N: usize = 4;
    fn src(i: usize, j: usize) -> usize {
        (i * N + j) * S_INT
    }
    fn dst(i: usize, j: usize) -> usize {
        (j * N + i) * S_INT + 64
    }
    let mut cache = DMCache::new(log2(N * N * S_INT * 2), c, 16);
    for i in 0..4 {
        for j in 0..4 {
            println!(
                "({}, {}):\t{}\t{}",
                i,
                j,
                cache.access(src(i, j)),
                cache.access(dst(i, j))
            );
        }
    }
}

fn problem_6_35() {
    simulate_6_35_36(32);
}

fn problem_6_36() {
    simulate_6_35_36(128);
}
