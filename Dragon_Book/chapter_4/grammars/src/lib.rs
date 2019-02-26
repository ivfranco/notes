pub mod backtrack;
mod parse_table;
mod slr;

use crate::parse_table::ParseTable;
use crate::parse_table::Production;
use crate::parse_table::Symbol::{self, *};
use crate::slr::{Canonical, Token};
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;

const START: usize = 0;

pub struct Grammar<T> {
    start: usize,
    pub prod_map: HashMap<usize, Vec<Production<T>>>,
    pub term_map: HashMap<String, usize>,
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

    pub fn rev_map(&self) -> HashMap<usize, String> {
        self.term_map
            .iter()
            .map(|(k, v)| (*v, k.to_owned()))
            .collect()
    }

    fn nonterm_len(&self) -> usize {
        self.prod_map.keys().max().unwrap() + 1
    }

    fn new_dash_term(&self, nonterm: &str) -> String {
        let mut dash = format!("{}'", nonterm);
        while self.term_map.contains_key(&dash) {
            dash.push('\'');
        }
        dash
    }

    fn query(&self, nonterm: &str) -> usize {
        *self
            .term_map
            .get(nonterm)
            .expect("Error: Query of unmentioned non-terminate symbol")
    }

    pub fn first(&self, symbol: &str) -> &HashSet<Option<T>> {
        self.first_nonterm(self.query(symbol))
    }

    fn first_nonterm(&self, nonterm: usize) -> &HashSet<Option<T>> {
        &self.first[&nonterm]
    }

    pub fn follow(&self, symbol: &str) -> &HashSet<Option<T>> {
        self.follow_nonterm(self.query(symbol))
    }

    fn follow_nonterm(&self, nonterm: usize) -> &HashSet<Option<T>> {
        &self.follow[&nonterm]
    }
}

impl Grammar<String> {
    pub fn parse(start: &str, input: &[&str]) -> Self {
        let mut term_map: HashMap<String, usize> = HashMap::new();
        term_map.insert(start.to_owned(), START);

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

        let mut grammar = Grammar::new(START, prods, term_map.clone());
        for s in term_map.values() {
            grammar.alloc(*s);
        }
        grammar.update_first_and_follow();
        grammar
    }
}

impl<T: Clone + Eq + Hash> Grammar<T> {
    fn augment(&mut self) {
        let rev_map = self.rev_map();
        let start_symbol = rev_map[&self.start].as_str();
        let new_start_symbol = self.new_dash_term(&start_symbol);
        let new_start = self.nonterm_len();

        let production = Production {
            head: new_start,
            body: vec![N(self.start)],
        };

        self.alloc(new_start);
        self.prod_map.insert(new_start, vec![production]);
        self.term_map.insert(new_start_symbol, new_start);
        self.start = new_start;
        self.update_first_and_follow();
    }

    fn alphabet(&self) -> HashSet<Symbol<T>> {
        let mut alphabet: HashSet<Symbol<T>> = self
            .prod_map
            .values()
            .flatten()
            .flat_map(|p| &p.body)
            .cloned()
            .collect();

        alphabet.insert(N(self.start));
        alphabet
    }

    fn split_left_recursion(&mut self, nonterm: &str) {
        let orig = self.term_map[nonterm];
        let dash = self.nonterm_len();
        let productions = self.prod_map.remove(&orig).unwrap_or_else(|| vec![]);

        let mut orig_prod = vec![];
        let mut dash_prod = vec![];
        dash_prod.push(Production {
            head: dash,
            body: vec![],
        });

        for p in productions {
            if p.is_left_recursive() {
                let mut body = p.body;
                body.remove(0);
                body.push(N(dash));
                dash_prod.push(Production { head: dash, body });
            } else {
                let mut body = p.body;
                body.push(N(dash));
                orig_prod.push(Production { head: orig, body });
            }
        }

        self.alloc(dash);
        self.prod_map.insert(orig, orig_prod);
        self.prod_map.insert(dash, dash_prod);

        self.term_map.insert(self.new_dash_term(nonterm), dash);
    }

    pub fn eliminate_immediate_left_recursions(&mut self) {
        let nonterms: Vec<String> = self.term_map.keys().cloned().collect();
        for nonterm in nonterms {
            if self.prod_map[&self.query(&nonterm)]
                .iter()
                .any(|p| p.is_left_recursive())
            {
                self.split_left_recursion(&nonterm);
            }
        }
        self.update_first_and_follow();
    }

    fn alloc(&mut self, nonterm: usize) {
        self.prod_map.entry(nonterm).or_insert_with(|| vec![]);
        self.first.entry(nonterm).or_insert_with(HashSet::new);
        self.follow.entry(nonterm).or_insert_with(HashSet::new);
    }

    pub fn eliminate_left_recursions(&mut self) {
        let rev_map = self.rev_map();
        for i in 0..self.nonterm_len() {
            let orig_prods = self.prod_map.remove(&i).unwrap();
            let mut new_prods: Vec<Production<T>> = vec![];

            for pi in orig_prods {
                match pi.body.first() {
                    Some(N(j)) if *j < i => {
                        for pj in &self.prod_map[j] {
                            let mut body = pj.body.clone();
                            body.extend(pi.body.iter().skip(1).cloned());
                            new_prods.push(Production { head: i, body });
                        }
                    }
                    _ => new_prods.push(pi),
                }
            }
            self.prod_map.insert(i, new_prods);
            self.split_left_recursion(&rev_map[&i]);
        }

        self.update_first_and_follow();
    }

    pub fn string_first(&self, symbols: &[Symbol<T>]) -> HashSet<Option<T>> {
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
            let incre = self.string_first(&p.body);
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
                    let mut incre = self.string_first(suffix);
                    if incre.remove(&None) {
                        incre.extend(
                            self.follow
                                .entry(p.head)
                                .or_insert_with(HashSet::new)
                                .iter()
                                .cloned(),
                        );
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

    fn reset_first_and_follow(&mut self) {
        for set in self.first.values_mut() {
            set.clear();
        }
        for set in self.follow.values_mut() {
            set.clear();
        }
    }

    fn update_first_and_follow(&mut self) {
        self.reset_first_and_follow();
        while self.update_first_once() {}
        while self.update_follow_once() {}
    }

    fn safe_pair(&self, alpha: &Production<T>, beta: &Production<T>) -> bool {
        assert_eq!(alpha.head, beta.head);

        let head = alpha.head;
        let alpha_first = self.string_first(&alpha.body);
        let beta_first = self.string_first(&beta.body);
        let follow = self.follow_nonterm(head);

        let disjoint_first = alpha_first.is_disjoint(&beta_first);
        let alpha_overlapping_follow =
            beta_first.contains(&None) && !alpha_first.is_disjoint(&follow);
        let beta_overlapping_follow =
            alpha_first.contains(&None) && !beta_first.is_disjoint(&follow);

        disjoint_first && !alpha_overlapping_follow && !beta_overlapping_follow
    }

    pub fn is_ll1(&self) -> bool {
        for ps in self.prod_map.values() {
            for (i, p0) in ps.iter().enumerate() {
                for p1 in &ps[i + 1..] {
                    if !self.safe_pair(p0, p1) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn to_ll1(&self) -> ParseTable<T> {
        assert!(self.is_ll1());

        let mut tables = vec![HashMap::new(); self.nonterm_len()];

        for p in self.prod_map.values().flatten() {
            let first = self.string_first(&p.body);
            for t in first.iter().filter_map(|o| o.clone()) {
                tables[p.head].insert(Some(t), p.clone());
            }
            if first.contains(&None) {
                for t in self.follow_nonterm(p.head).iter().cloned() {
                    tables[p.head].insert(t, p.clone());
                }
            }
        }

        ParseTable::new(self.start, tables)
    }
}

impl<T: Token> Grammar<T> {
    pub fn canonical(&mut self) -> Canonical<T> {
        self.augment();
        Canonical::new(self)
    }
}

impl<T: Debug> Debug for Grammar<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let rev_map = self.rev_map();
        for p in self.prod_map.values().flatten() {
            writeln!(f, "{}", p.to_string(&rev_map))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        let grammar = Grammar::parse(
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

    #[test]
    fn left_recursion_test() {
        let mut grammar = Grammar::parse(
            "bexpr",
            &[
                "bexpr -> bexpr or bterm",
                "bexpr -> bterm",
                "bterm -> bterm and bfactor",
                "bterm -> bfactor",
                "bfactor -> not bfactor",
                "bfactor -> ( bexpr )",
                "bfactor -> true",
                "bfactor -> false",
            ],
        );

        grammar.eliminate_immediate_left_recursions();

        assert_eq!(grammar.first("bexpr'"), &constr_set(&["or", "ε"]));
        assert_eq!(grammar.first("bterm'"), &constr_set(&["and", "ε"]));
    }

    #[test]
    fn ll1_test() {
        let grammar = Grammar::parse(
            "stmt",
            &[
                "stmt -> if ( expr ) stmt else stmt",
                "stmt -> while ( expr ) stmt",
                "stmt -> { stmt_list }",
            ],
        );

        assert!(grammar.is_ll1());

        let grammar = Grammar::parse(
            "S",
            &[
                "S -> i E t S S'",
                "S -> a",
                "S' -> e S",
                "S' -> ε",
                "E -> b",
            ],
        );

        assert!(!grammar.is_ll1());
    }
}
