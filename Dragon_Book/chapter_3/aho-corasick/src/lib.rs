use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
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

type State = usize;

pub struct Trie<T: Eq + Hash + Debug> {
    map: Vec<HashMap<T, State>>,
    pub failure: Vec<State>,
}

impl<T: Copy + Eq + Debug + Hash> Trie<T> {
    pub fn new(patterns: &[&[T]]) -> Self {
        let mut max_state = 0;
        let mut map = vec![HashMap::new()];

        for pattern in patterns {
            let mut local_state = 0;
            for c in pattern.iter() {
                local_state = if map[local_state].contains_key(c) {
                    map[local_state][c]
                } else {
                    max_state += 1;
                    map.push(HashMap::new());
                    map[local_state].insert(*c, max_state);
                    max_state
                }
            }
        }

        Trie {
            failure: failure_trie(&map),
            map,
        }
    }
}

impl Debug for Trie<u8> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Transitions: ")?;
        for (from, trans) in self.map.iter().enumerate() {
            for (c, to) in trans {
                writeln!(f, "    δ({}, {}) = {}", from, char::from(*c), to)?;
            }
        }
        writeln!(f, "Failure function: ")?;
        writeln!(f, "    {:?}", self.failure)
    }
}

fn failure_trie<T: Hash + Eq>(map: &[HashMap<T, State>]) -> Vec<State> {
    let mut fail = vec![0; map.len()];
    let mut queue = VecDeque::new();

    for s in map[0].values() {
        queue.push_back(s);
    }

    while let Some(r) = queue.pop_front() {
        for (a, s) in map[*r].iter() {
            queue.push_back(s);
            let mut state = fail[*r];
            while state != 0 && map[state].get(a).is_none() {
                state = fail[state];
            }
            fail[*s] = *map[state].get(a).unwrap_or(&0);
        }
    }

    fail
}

pub fn kmp<T: PartialEq>(string: &[T], pattern: &[T]) -> Option<usize> {
    let mut s = 0;
    let mut max_app = 0;

    let fail = failure(pattern);

    for (i, c) in string.iter().enumerate() {
        let mut local_app = 0;
        while s > 0 && c != &pattern[s] {
            s = fail[s - 1];
            local_app += 1;
        }

        max_app = std::cmp::max(local_app, max_app);

        if c == &pattern[s] {
            s += 1;
        }
        if s == pattern.len() {
            // dbg!(max_app);
            return Some(i + 1 - s);
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

    #[test]
    fn fibonacci_kmp() {
        for i in 2..=20 {
            assert!(kmp(
                fibonacci_string(i + 1).as_bytes(),
                fibonacci_string(i).as_bytes(),
            )
            .is_some());
        }
    }

    #[test]
    fn trie_failure() {
        let patterns: &[&[u8]] = &[b"he", b"she", b"his", b"hers"];

        let trie = Trie::new(patterns);
        assert_eq!(trie.failure, [0, 0, 0, 0, 1, 2, 0, 3, 0, 3]);
    }
}
