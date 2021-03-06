use crate::dfa::{State, DFA, START};
use crate::parser::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

#[derive(PartialEq)]
struct Node {
    state: State,
    symbol: Option<char>,
    label: Option<String>,
    exits: Vec<SharedNode>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let c = if let Some(c) = self.symbol { c } else { 'ε' };
        write!(f, "δ({}, {}) = ", self.state, c)?;
        f.debug_set()
            .entries(self.exits.iter().map(|exit| exit.borrow().state))
            .finish()
    }
}

impl Node {
    fn new(state: State, symbol: Option<char>) -> Self {
        Node {
            state,
            symbol,
            label: None,
            exits: vec![],
        }
    }

    fn new_shared(state: State, symbol: Option<char>) -> SharedNode {
        let node = Node::new(state, symbol);
        Rc::new(RefCell::new(node))
    }

    fn install_exit(&mut self, node: &SharedNode) {
        self.exits.push(node.clone());
    }

    fn install_label(&mut self, label: &str) {
        self.label = Some(label.to_owned());
    }
}

type SharedNode = Rc<RefCell<Node>>;

pub struct NFA {
    start: SharedNode,
    end: SharedNode,
    size: usize,
}

impl NFA {
    fn new(state: State, symbol: Option<char>) -> Self {
        let start = Node::new_shared(state, symbol);
        let end = Node::new_shared(state + 1, None);
        start.borrow_mut().install_exit(&end);

        NFA {
            start,
            end,
            size: 2,
        }
    }

    pub fn from_regex(regex: &Regex, state: State) -> Self {
        use self::Regex::*;
        match regex {
            Empty => NFA::new(state, None),
            Literal(c) => NFA::new(state, Some(*c)),
            Union(l, r) => {
                let start = Node::new_shared(state, None);
                let lhs = NFA::from_regex(l, state + 1);
                let rhs = NFA::from_regex(r, state + lhs.size + 1);
                let end = Node::new_shared(state + lhs.size + rhs.size + 1, None);

                start.borrow_mut().install_exit(&lhs.start);
                start.borrow_mut().install_exit(&rhs.start);
                lhs.end.borrow_mut().install_exit(&end);
                rhs.end.borrow_mut().install_exit(&end);

                NFA {
                    start,
                    end,
                    size: lhs.size + rhs.size + 2,
                }
            }
            Concat(l, r) => {
                let lhs = NFA::from_regex(l, state);
                let rhs = NFA::from_regex(r, state + lhs.size);

                lhs.end.borrow_mut().install_exit(&rhs.start);

                NFA {
                    start: lhs.start.clone(),
                    end: rhs.end.clone(),
                    size: lhs.size + rhs.size,
                }
            }
            Kleene(inner) => {
                let start = Node::new_shared(state, None);
                let single = NFA::from_regex(inner, state + 1);
                let end = Node::new_shared(state + single.size + 1, None);

                start.borrow_mut().install_exit(&single.start);
                start.borrow_mut().install_exit(&end);
                single.end.borrow_mut().install_exit(&single.start);
                single.end.borrow_mut().install_exit(&end);

                NFA {
                    start,
                    end,
                    size: single.size + 2,
                }
            }
        }
    }

    pub fn parse(regex: &str) -> Self {
        NFA::from_regex(&Regex::parse(regex), START)
    }

    pub fn install_label(&mut self, label: &str) {
        self.end.borrow_mut().install_label(label);
    }

    pub fn multi_parse(parts: &[(&str, &str)]) -> Self {
        let start = Node::new_shared(START, None);
        let mut state = 1;

        let branches: Vec<NFA> = parts
            .iter()
            .map(|(regex, label)| {
                let mut nfa = NFA::from_regex(&Regex::parse(regex), state);
                state += nfa.size;
                nfa.install_label(label);
                nfa
            })
            .collect();

        let end = Node::new_shared(state, None);

        for branch in branches.iter() {
            start.borrow_mut().install_exit(&branch.start);
            branch.end.borrow_mut().install_exit(&end);
        }

        NFA {
            start,
            end,
            size: state + 1,
        }
    }

    pub fn parse_lookahead(regex: &str) -> (NFA, NFA) {
        let mut parts = regex.split('/');
        let lhs = parts.next().unwrap();
        let rhs = parts.next().unwrap();
        (NFA::parse(lhs), NFA::parse(rhs))
    }

    pub fn init(&self) -> NFAState {
        NFAState::new(self)
    }

    pub fn accept(&self, string: &str) -> bool {
        let mut state = self.init();
        for c in string.chars() {
            state = state.next(c);
        }
        state.accepted()
    }

    pub fn to_dfa(&self, alphabet: &str) -> (HashMap<Vec<State>, State>, DFA) {
        let init = self.init();
        let mut set_to_state: HashMap<Vec<State>, State> = HashMap::new();
        set_to_state.insert(init.sorted(), START);
        let mut dfa = DFA::new(0, &[]);
        if init.accepted() {
            dfa.install_final(START);
        }
        dfa.install_labels(START, init.labels());
        let mut stack: Vec<NFAState> = vec![init];
        let mut max_state: State = 0;

        while let Some(state) = stack.pop() {
            for c in alphabet.chars() {
                let from = set_to_state[&state.sorted()];
                let next = state.next(c);
                if !next.is_empty() {
                    let labels = next.labels();
                    let is_final = next.accepted();
                    let to = *set_to_state.entry(next.sorted()).or_insert_with(|| {
                        max_state += 1;
                        stack.push(next);
                        max_state
                    });
                    dfa.install_transition(from, c, to);
                    dfa.install_labels(to, labels);
                    if is_final {
                        dfa.install_final(to);
                    }
                }
            }
        }

        (set_to_state, dfa)
    }
}

impl Debug for NFA {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut stack = vec![self.start.clone()];
        let mut seen = HashSet::new();

        while let Some(node) = stack.pop() {
            let borrow = node.borrow();
            if let Some(ref label) = borrow.label {
                println!("{}: {}", borrow.state, label);
            }
            if !borrow.exits.is_empty() {
                writeln!(f, "{:?}", borrow)?;
            }
            seen.insert(borrow.state);
            for exit in &borrow.exits {
                if !seen.contains(&exit.borrow().state) {
                    stack.push(exit.clone());
                }
            }
        }

        writeln!(f, "Final state: {}", self.end.borrow().state)
    }
}

pub struct NFAState {
    states: Vec<SharedNode>,
    accept: State,
}

impl NFAState {
    fn new(nfa: &NFA) -> Self {
        let state = NFAState {
            states: vec![nfa.start.clone()],
            accept: nfa.end.borrow().state,
        };

        state.extend_with_empty()
    }

    fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    fn labels(&self) -> Vec<String> {
        self.states
            .iter()
            .filter_map(|state| state.borrow().label.clone())
            .collect()
    }

    pub fn sorted(&self) -> Vec<State> {
        let mut states: Vec<State> = self.states.iter().map(|cell| cell.borrow().state).collect();
        states.sort();
        states
    }

    pub fn next(&self, c: char) -> Self {
        let state = self.consume(c);
        state.extend_with_empty()
    }

    pub fn accepted(&self) -> bool {
        self.states
            .iter()
            .any(|node| node.borrow().state == self.accept)
    }

    fn consume(&self, c: char) -> Self {
        let states: Vec<SharedNode> = self
            .states
            .iter()
            .flat_map(|state| {
                let borrow = state.borrow();
                if borrow.symbol == Some(c) {
                    borrow.exits.clone()
                } else {
                    vec![]
                }
            })
            .collect();

        NFAState {
            states,
            accept: self.accept,
        }
    }

    fn extend_with_empty(&self) -> Self {
        let mut seen: HashSet<usize> = HashSet::new();
        let mut states: Vec<SharedNode> = vec![];
        let mut stack = self.states.clone();

        while let Some(node) = stack.pop() {
            let borrow = node.borrow();
            seen.insert(borrow.state);

            if borrow.symbol == None {
                stack.extend(
                    borrow
                        .exits
                        .iter()
                        .filter(|exit| !seen.contains(&exit.borrow().state))
                        .cloned(),
                );
            }

            drop(borrow);
            states.push(node);
        }

        NFAState {
            states,
            accept: self.accept,
        }
    }
}

#[test]
fn nfa_construction_test() {
    let nfa = NFA::parse("(a|b)*");
    // print!("{:?}", nfa);
    assert_eq!(nfa.size, 8);
}

#[test]
fn nfa_accept_test() {
    let nfa = NFA::parse("(a|b)*abb(a|b)*");

    assert!(nfa.accept("abb"));
    assert!(nfa.accept("aabbabbabab"));
    assert!(!nfa.accept("bba"));
    assert!(!nfa.accept(""));
    assert!(!nfa.accept("ababababab"));

    let mut dfa = nfa.to_dfa("ab").1;

    assert!(dfa.accept("abb"));
    assert!(dfa.accept("aabbabbabab"));
    assert!(!dfa.accept("bba"));
    assert!(!dfa.accept(""));
    assert!(!dfa.accept("ababababab"));

    dfa = dfa.minimize("ab");

    assert!(dfa.accept("abb"));
    assert!(dfa.accept("aabbabbabab"));
    assert!(!dfa.accept("bba"));
    assert!(!dfa.accept(""));
    assert!(!dfa.accept("ababababab"));
}

#[test]
fn multi_parse_test() {
    const DIGIT: &str = "0|1";
    const LETTER: &str = "(w|h|i|l|e|n)";

    let nfa = NFA::multi_parse(&[
        ("while", "WHILE"),
        ("when", "WHEN"),
        (&format!("{}({}|{})*", LETTER, LETTER, DIGIT), "ID"),
    ]);

    assert!(nfa.accept("while"));
    assert!(nfa.accept("when"));
    assert!(!nfa.accept(""));
    assert!(!nfa.accept("0101"));
    assert!(nfa.accept("w101"));

    let mut dfa = nfa.to_dfa("whilen01").1;

    assert!(dfa.accept("while"));
    assert!(dfa.accept("when"));
    assert!(!dfa.accept(""));
    assert!(!dfa.accept("0101"));
    assert!(dfa.accept("w101"));

    dfa = dfa.minimize("whilen01");

    assert!(dfa.accept("while"));
    assert!(dfa.accept("when"));
    assert!(!dfa.accept(""));
    assert!(!dfa.accept("0101"));
    assert!(dfa.accept("w101"));
}
