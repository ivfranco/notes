use std::mem;

pub fn failure<T: PartialEq>(pattern: &[T]) -> Vec<usize> {
    let mut fail = vec![0; pattern.len()];

    let mut t = 0;

    for s in 1..pattern.len() {
        while t > 0 && pattern[s] != pattern[t] {
            t = fail[t - 1];
        }
        if pattern[s] == pattern[t] {
            t += 1;
            fail[s] = t;
        } else {
            fail[s] = 0;
        }
    }

    fail
}

pub fn kmp<T: PartialEq>(string: &[T], pattern: &[T]) -> Option<usize> {
    let mut s = 0;

    let fail = failure(pattern);

    for (i, c) in string.iter().enumerate() {
        while s > 0 && c != &pattern[s] {
            s = fail[s - 1];
        }
        if c == &pattern[s] {
            s += 1;
        }
        if s == pattern.len() {
            return Some(i - s + 1);
        }
    }

    None
}

pub fn fibonacci_string(n: u32) -> String {
    let mut curr = "b".to_owned();
    let mut next = "a".to_owned();

    for _ in 1..n {
        mem::swap(&mut curr, &mut next);
        next = format!("{}{}", curr, next);
    }

    curr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn failure_test() {
        assert_eq!(failure(b"ababaa"), &[0, 0, 1, 2, 3, 1]);
    }

    #[test]
    fn kmp_test() {
        assert_eq!(kmp(b"abababaab", b"ababaa"), Some(2));
        assert_eq!(kmp(b"abababbaa", b"ababaa"), None);
    }

    #[test]
    fn fibonacci_test() {
        assert_eq!(fibonacci_string(3), "ab");
        assert_eq!(fibonacci_string(4), "aba");
        assert_eq!(fibonacci_string(5), "abaab");
    }

    fn fibonacci_number(n: u32) -> usize {
        let mut curr = 1;
        let mut next = 1;

        for _ in 1..n {
            mem::swap(&mut curr, &mut next);
            next += curr;
        }

        curr
    }

    fn fibonacci_failure(n: u32) -> Vec<usize> {
        let fib = fibonacci_number(n);
        let mut fail = vec![0; fib];

        for i in 3..=fib {
            let mut k = 1;
            while fibonacci_number(k + 1) <= i + 1 {
                k += 1;
            }
            fail[i - 1] = i - fibonacci_number(k - 1);
        }

        fail
    }

    #[test]
    fn fibonacci_failure_test() {
        for i in 1..=20 {
            assert_eq!(
                failure(fibonacci_string(i).as_bytes()),
                fibonacci_failure(i)
            );
        }
    }
}
