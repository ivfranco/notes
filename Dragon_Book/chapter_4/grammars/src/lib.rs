use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq)]
pub enum Symbol<T> {
    N(usize),
    T(T),
}

impl<T: Debug> Symbol<T> {
    fn to_string(&self, rev_map: &HashMap<usize, &str>) -> String {
        match self {
            N(s) => rev_map[s].to_owned(),
            T(t) => format!("{:?}", t),
        }
    }
}

use self::Symbol::*;

#[derive(Clone)]
pub struct Production<T> {
    head: usize,
    body: Vec<Symbol<T>>,
}

impl<T: Debug> Production<T> {
    fn to_string(&self, rev_map: &HashMap<usize, &str>) -> String {
        let mut symbols = self
            .body
            .iter()
            .map(|symbol| symbol.to_string(rev_map))
            .collect::<Vec<_>>()
            .join(" ");

        if symbols.is_empty() {
            symbols = "ε".to_owned();
        }

        format!("{} -> {}", N::<T>(self.head).to_string(rev_map), symbols)
    }
}

pub struct Grammar<T> {
    start: usize,
    prod_map: HashMap<usize, Vec<Production<T>>>,
    term_map: HashMap<String, usize>,
    first: HashMap<usize, HashSet<Option<T>>>,
    follow: HashMap<usize, HashSet<Option<T>>>,
}

impl<T> Grammar<T> {
    fn new(start: usize, prods: Vec<Production<T>>, term_map: HashMap<String, usize>) -> Self {
        let mut prod_map = HashMap::new();
        for p in prods {
            prod_map.entry(p.head).or_insert_with(|| vec![]).push(p);
        }

        Grammar {
            start,
            prod_map,
            term_map,
            first: HashMap::new(),
            follow: HashMap::new(),
        }
    }
}

impl Grammar<String> {
    pub fn parse(start: &str, input: &[&str]) -> Self {
        let mut term_map: HashMap<String, usize> = HashMap::new();

        let parts: Vec<(&str, &str)> = input
            .iter()
            .map(|line| {
                let mut iter = line.split(" -> ");
                (iter.next().unwrap(), iter.next().unwrap())
            })
            .collect();

        for &head in parts.iter().map(|(head, _)| head) {
            if !term_map.contains_key(head) {
                term_map.insert(head.to_owned(), term_map.len());
            }
        }

        let prods: Vec<Production<String>> = parts
            .iter()
            .map(|(head, symbols)| {
                let mut body: Vec<Symbol<String>> = symbols
                    .split_whitespace()
                    .map(|symbol| {
                        if let Some(s) = term_map.get(symbol) {
                            N(*s)
                        } else {
                            T(symbol.to_owned())
                        }
                    })
                    .collect();
                if body.len() == 1 && body[0] == T("ε".to_owned()) {
                    body.clear();
                }

                Production {
                    head: term_map[*head],
                    body,
                }
            })
            .collect();

        let mut grammar = Grammar::new(term_map[start], prods, term_map.clone());
        for s in term_map.values() {
            grammar.alloc(*s);
        }
        grammar
    }
}

impl<T: Clone + Eq + Hash> Grammar<T> {
    fn alloc(&mut self, nonterm: usize) {
        self.prod_map.entry(nonterm).or_insert_with(|| vec![]);
        self.first.entry(nonterm).or_insert_with(HashSet::new);
        self.follow.entry(nonterm).or_insert_with(HashSet::new);
    }

    pub fn first(&self, symbol: &str) -> &HashSet<Option<T>> {
        let s = self.term_map[symbol];
        &self.first[&s]
    }

    pub fn follow(&self, symbol: &str) -> &HashSet<Option<T>> {
        let s = self.term_map[symbol];
        &self.follow[&s]
    }

    fn calc_first(&self, symbols: &[Symbol<T>]) -> HashSet<Option<T>> {
        let mut set = HashSet::new();

        let nullable = symbols.iter().all(|symbol| {
            if let N(s) = symbol {
                self.first[s].contains(&None)
            } else {
                false
            }
        });

        if nullable {
            set.insert(None);
        }

        for symbol in symbols {
            match symbol {
                T(token) => {
                    set.insert(Some(token.clone()));
                    break;
                }
                N(s) => {
                    set.extend(self.first[s].iter().cloned().filter(|o| o.is_some()));
                    if !self.first[s].contains(&None) {
                        break;
                    }
                }
            }
        }

        set
    }

    fn update_first_once(&mut self) -> bool {
        let mut updated = false;
        for p in self.prod_map.values().flatten() {
            let incre = self.calc_first(&p.body);
            let set = self.first.entry(p.head).or_insert_with(HashSet::new);
            let old_len = set.len();
            set.extend(incre);
            updated = updated || old_len < set.len();
        }

        updated
    }

    fn update_follow_once(&mut self) -> bool {
        let mut updated = false;

        self.follow
            .entry(self.start)
            .or_insert_with(HashSet::new)
            .insert(None);

        for p in self.prod_map.values().flatten() {
            for i in 0..p.body.len() {
                if let N(s) = p.body[i] {
                    let suffix = &p.body[i + 1..];
                    let mut incre = self.calc_first(suffix);
                    if incre.remove(&None) {
                        incre.extend(self.follow[&p.head].iter().cloned());
                    }
                    let set = self.follow.entry(s).or_insert_with(HashSet::new);
                    let old_len = set.len();
                    set.extend(incre);
                    updated = updated || old_len < set.len();
                }
            }
        }

        updated
    }

    pub fn update_first_and_follow(&mut self) {
        while self.update_first_once() {}
        while self.update_follow_once() {}
    }
}

impl<T: Debug> Debug for Grammar<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let rev_map = self
            .term_map
            .iter()
            .map(|(k, v)| (*v, k.as_str()))
            .collect();

        for p in self.prod_map.values().flatten() {
            writeln!(f, "{}", p.to_string(&rev_map))?;
        }

        Ok(())
    }
}

#[cfg(test)]
fn constr_set(tokens: &[&str]) -> HashSet<Option<String>> {
    tokens
        .iter()
        .map(|token| {
            if *token == "ε" || *token == "$" {
                None
            } else {
                Some(token.to_string())
            }
        })
        .collect()
}

#[test]
fn first_follow_test() {
    let mut grammar = Grammar::parse(
        "E",
        &[
            "E -> T E'",
            "E' -> + T E'",
            "E' -> ε",
            "T -> F T'",
            "T' -> * F T'",
            "T' -> ε",
            "F -> ( E )",
            "F -> id",
        ],
    );
    grammar.update_first_and_follow();

    let e_first = constr_set(&["(", "id"]);
    assert_eq!(grammar.first("E"), &e_first);
    assert_eq!(grammar.first("T"), &e_first);
    assert_eq!(grammar.first("F"), &e_first);

    assert_eq!(grammar.first("E'"), &constr_set(&["+", "ε"]));
    assert_eq!(grammar.first("T'"), &constr_set(&["*", "ε"]));

    let e_follow = constr_set(&[")", "$"]);
    assert_eq!(grammar.follow("E"), &e_follow);
    assert_eq!(grammar.follow("E'"), &e_follow);

    let t_follow = constr_set(&["+", ")", "$"]);
    assert_eq!(grammar.follow("T"), &t_follow);
    assert_eq!(grammar.follow("T'"), &t_follow);

    assert_eq!(grammar.follow("F"), &constr_set(&["+", "*", ")", "$"]));
}
