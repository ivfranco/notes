use crate::parser::Regex;
use disjoint_sets::UnionFind;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};

pub type State = usize;
pub const START: State = 0;

pub struct DFA {
    map: Vec<HashMap<char, State>>,
    finals: HashSet<State>,
    labels: HashMap<State, Vec<String>>,
}

impl DFA {
    pub fn new(n_state: usize, finals: &[State]) -> Self {
        DFA {
            map: vec![HashMap::new(); n_state],
            finals: finals.iter().cloned().collect(),
            labels: HashMap::new(),
        }
    }

    pub fn from_regex(regex: &Regex, alphabet: &str) -> Self {
        let mut env = HashMap::new();
        let pos_tree = PosTree::from_regex(regex, &mut env, 0).wrap_end(&mut env);
        let mut follow_pos = HashMap::new();
        for i in 0..pos_tree.size {
            follow_pos.insert(i, HashSet::new());
        }
        pos_tree.populate_follow_pos(&mut follow_pos);

        let mut dfa = DFA::new(0, &[]);
        let mut max_state = START;
        let mut seen: HashMap<Vec<State>, State> = HashMap::new();
        let init = sorted(&pos_tree.first_pos);
        seen.insert(init.clone(), START);
        if init.contains(&pos_tree.final_state()) {
            dfa.install_final(START);
        }
        let mut stack: Vec<Vec<State>> = vec![init];

        while let Some(states) = stack.pop() {
            for symbol in alphabet.chars() {
                let from = seen[&states];
                let next = sorted(
                    &states
                        .iter()
                        .filter(|state| env[state] == symbol)
                        .flat_map(|state| &follow_pos[&state])
                        .cloned()
                        .collect(),
                );

                if !next.is_empty() {
                    let is_final = next.contains(&pos_tree.final_state());
                    let to = *seen.entry(next.clone()).or_insert_with(|| {
                        max_state += 1;
                        stack.push(next);
                        max_state
                    });

                    dfa.install_transition(from, symbol, to);
                    if is_final {
                        dfa.install_final(to);
                    }
                }
            }
        }

        dfa
    }

    pub fn parse(regex: &str, alphabet: &str) -> Self {
        DFA::from_regex(&Regex::parse(regex), alphabet)
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }

    pub fn init(&self) -> DFAState {
        DFAState::new(self)
    }

    fn alloc(&mut self, state: State) {
        if self.map.len() <= state {
            self.map.resize(state + 1, HashMap::new());
        }
    }

    pub fn install_transition(&mut self, from: State, symbol: char, to: State) {
        self.alloc(std::cmp::max(from, to));
        self.map[from].insert(symbol, to);
    }

    pub fn install_final(&mut self, state: State) {
        self.alloc(state);
        self.finals.insert(state);
    }

    pub fn install_labels(&mut self, state: State, labels: Vec<String>) {
        self.labels.insert(state, labels);
    }

    pub fn minimize(&self, alphabet: &str) -> DFA {
        minimize(self, alphabet)
    }

    fn goto(&self, state: State, symbol: char) -> Option<State> {
        assert!(self.map.len() > state, "Out of bound state");

        self.map[state].get(&symbol).cloned()
    }

    fn goto_usize(&self, state: State, symbol: char) -> State {
        self.goto(state, symbol).unwrap_or_else(|| self.size())
    }

    pub fn accept(&self, string: &str) -> bool {
        let mut state = self.init();

        for symbol in string.chars() {
            state = state.next(symbol);
        }

        state.accepted()
    }

    fn state_trace(&self, string: &str) -> Vec<(usize, DFAState)> {
        let mut stack = vec![(0, self.init())];

        for (i, c) in string.char_indices() {
            let next = stack.last().unwrap().1.next(c);
            if next.dead() {
                break;
            } else {
                // i points to the starting byte of c
                // i + c.len_utf8() points to the starting byte of the character following c
                // hence &string[.. i] will be a substring containing all characters up to c
                stack.push((i + c.len_utf8(), next));
            }
        }

        stack
    }

    pub fn capture<'a>(&self, string: &'a str) -> Option<&'a str> {
        self.state_trace(string)
            .into_iter()
            .rev()
            .find_map(|(i, state)| {
                if state.accepted() {
                    Some(&string[..i])
                } else {
                    None
                }
            })
    }

    pub fn capture_lookahead<'a>(&self, lookahead: &DFA, string: &'a str) -> Option<&'a str> {
        self.state_trace(string)
            .iter()
            .rev()
            .find_map(|(i, state)| {
                if state.accepted() && lookahead.capture(&string[*i..]).is_some() {
                    Some(&string[..*i])
                } else {
                    None
                }
            })
    }
}

fn reps(sets: &UnionFind) -> HashSet<usize> {
    sets.to_vec().into_iter().collect()
}

fn minimize(dfa: &DFA, alphabet: &str) -> DFA {
    let mut sets = init_sets(dfa);

    loop {
        let new_sets = evolve(dfa, &sets, alphabet);
        if reps(&sets).len() == reps(&new_sets).len() {
            break;
        } else {
            sets = new_sets;
        }
    }

    group_states(dfa, &sets, alphabet)
}

fn init_sets(dfa: &DFA) -> UnionFind {
    let mut sets = UnionFind::new(dfa.size() + 1);
    //  if there's no final state in DFA, final_state = dfa.size(), a virtual dead state
    //  no state will be unioned with it in that case
    let final_state = dfa
        .finals
        .iter()
        .next()
        .cloned()
        .unwrap_or_else(|| dfa.size());
    //  if there's no non-final state, non_final_state = dfa.size(), a virtual dead state
    //  no state will be unioned with it in that case
    let non_final_state = (0..=dfa.size())
        .find(|state| !dfa.finals.contains(state))
        .unwrap();

    for i in 0..dfa.size() {
        if dfa.finals.contains(&i) {
            sets.union(i, final_state);
        } else {
            sets.union(i, non_final_state);
        }
    }

    sets
}

fn evolve(dfa: &DFA, sets: &UnionFind, alphabet: &str) -> UnionFind {
    let mut new_sets = UnionFind::new(dfa.size() + 1);

    for i in 0..dfa.size() {
        for j in i + 1..dfa.size() {
            let common_dest = alphabet
                .chars()
                .all(|symbol| sets.equiv(dfa.goto_usize(i, symbol), dfa.goto_usize(j, symbol)));
            let common_orig = sets.equiv(i, j);
            if common_dest && common_orig {
                new_sets.union(i, j);
            }
        }
    }

    new_sets
}

fn group_states(dfa: &DFA, sets: &UnionFind, alphabet: &str) -> DFA {
    let mut minimized = DFA::new(0, &[]);
    let mut map: HashMap<State, State> = HashMap::new();
    map.insert(sets.find(START), START);

    let reps: Vec<State> = reps(sets)
        .into_iter()
        .filter(|state| *state < dfa.size())
        .collect();

    for i in &reps {
        if !map.contains_key(i) {
            map.insert(*i, map.len());
        }
    }

    for rep in reps {
        let from = map[&rep];
        for symbol in alphabet.chars() {
            let to_rep = sets.find(dfa.goto_usize(rep, symbol));
            if to_rep != dfa.size() {
                let to = map[&to_rep];
                minimized.install_transition(from, symbol, to);
            }
        }
    }

    for f in &dfa.finals {
        minimized.install_final(map[&sets.find(*f)]);
    }

    minimized
}

impl Debug for DFA {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Transitions:")?;

        for (from, trans) in self.map.iter().enumerate() {
            let labels = self
                .labels
                .get(&from)
                .map(|vec| vec.as_slice())
                .unwrap_or(&[]);
            if !labels.is_empty() {
                write!(f, "    {}: ", from)?;
                f.debug_set().entries(labels).finish()?;
                writeln!(f, "")?;
            }
            for (symbol, to) in trans {
                writeln!(f, "    δ({}, {}) = {}", from, symbol, to)?;
            }
        }

        let mut finals = self.finals.iter().collect::<Vec<_>>();
        finals.sort();
        write!(f, "Final states: ")?;
        f.debug_set().entries(finals).finish()
    }
}

pub struct DFAState<'a> {
    dfa: &'a DFA,
    state: Option<State>,
}

impl<'a> DFAState<'a> {
    fn new(dfa: &'a DFA) -> Self {
        DFAState {
            dfa,
            state: Some(START),
        }
    }

    fn dead(&self) -> bool {
        self.state.is_none()
    }

    #[allow(dead_code)]
    fn on_label(&self, label: &str) -> bool {
        if let Some(s) = self.state {
            self.dfa.labels[&s].iter().any(|s| s == label)
        } else {
            false
        }
    }

    fn next(&self, symbol: char) -> Self {
        let next = self.state.and_then(|s| self.dfa.goto(s, symbol));

        DFAState {
            dfa: self.dfa,
            state: next,
        }
    }

    fn accepted(&self) -> bool {
        if let Some(s) = self.state {
            self.dfa.finals.contains(&s)
        } else {
            false
        }
    }
}

enum SynNode {
    End,
    Empty,
    Literal(State),
    Kleene(Box<PosTree>),
    Union(Box<PosTree>, Box<PosTree>),
    Concat(Box<PosTree>, Box<PosTree>),
}

struct PosTree {
    size: usize,
    nullable: bool,
    first_pos: HashSet<State>,
    last_pos: HashSet<State>,
    node: SynNode,
}

impl PosTree {
    fn from_regex(regex: &Regex, env: &mut HashMap<State, char>, state: State) -> Self {
        match regex {
            Regex::Empty => PosTree {
                size: 0,
                nullable: true,
                first_pos: HashSet::new(),
                last_pos: HashSet::new(),
                node: SynNode::Empty,
            },
            Regex::Literal(symbol) => {
                env.insert(state, *symbol);

                PosTree {
                    size: 1,
                    nullable: false,
                    first_pos: [state].iter().cloned().collect(),
                    last_pos: [state].iter().cloned().collect(),
                    node: SynNode::Literal(state),
                }
            }
            Regex::Kleene(inner) => {
                let inner = PosTree::from_regex(inner, env, state);
                PosTree {
                    size: inner.size,
                    nullable: true,
                    first_pos: inner.first_pos.clone(),
                    last_pos: inner.last_pos.clone(),
                    node: SynNode::Kleene(Box::new(inner)),
                }
            }
            Regex::Union(lhs, rhs) => {
                let lhs = PosTree::from_regex(lhs, env, state);
                let rhs = PosTree::from_regex(rhs, env, state + lhs.size);

                PosTree {
                    size: lhs.size + rhs.size,
                    nullable: lhs.nullable || rhs.nullable,
                    first_pos: lhs.first_pos.union(&rhs.first_pos).cloned().collect(),
                    last_pos: lhs.last_pos.union(&rhs.last_pos).cloned().collect(),
                    node: SynNode::Union(Box::new(lhs), Box::new(rhs)),
                }
            }
            Regex::Concat(lhs, rhs) => {
                let lhs = PosTree::from_regex(lhs, env, state);
                let rhs = PosTree::from_regex(rhs, env, state + lhs.size);

                lhs.concat(rhs)
            }
        }
    }

    fn concat(self, rhs: Self) -> Self {
        let lhs = self;
        PosTree {
            size: lhs.size + rhs.size,
            nullable: lhs.nullable && rhs.nullable,
            first_pos: if lhs.nullable {
                lhs.first_pos.union(&rhs.first_pos).cloned().collect()
            } else {
                lhs.first_pos.clone()
            },
            last_pos: if rhs.nullable {
                lhs.last_pos.union(&rhs.last_pos).cloned().collect()
            } else {
                rhs.last_pos.clone()
            },
            node: SynNode::Concat(Box::new(lhs), Box::new(rhs)),
        }
    }

    fn final_state(&self) -> State {
        self.size - 1
    }

    fn wrap_end(self, env: &mut HashMap<State, char>) -> Self {
        let end = PosTree {
            size: 1,
            nullable: false,
            first_pos: [self.size].iter().cloned().collect(),
            last_pos: [self.size].iter().cloned().collect(),
            node: SynNode::End,
        };

        env.insert(self.size, '#');

        self.concat(end)
    }

    fn populate_follow_pos(&self, follow_pos: &mut HashMap<State, HashSet<State>>) {
        match &self.node {
            SynNode::Kleene(inner) => {
                inner.populate_follow_pos(follow_pos);

                for i in &self.last_pos {
                    follow_pos
                        .entry(*i)
                        .or_insert_with(HashSet::new)
                        .extend(&self.first_pos);
                }
            }
            SynNode::Concat(lhs, rhs) => {
                lhs.populate_follow_pos(follow_pos);
                rhs.populate_follow_pos(follow_pos);

                for i in &lhs.last_pos {
                    follow_pos
                        .entry(*i)
                        .or_insert_with(HashSet::new)
                        .extend(&rhs.first_pos);
                }
            }
            SynNode::Union(lhs, rhs) => {
                lhs.populate_follow_pos(follow_pos);
                rhs.populate_follow_pos(follow_pos);
            }
            _ => (),
        }
    }
}

fn sorted(set: &HashSet<State>) -> Vec<State> {
    let mut states: Vec<State> = set.iter().cloned().collect();
    states.sort();
    states
}

#[test]
fn dfa_construction_test() {
    let dfa = DFA::parse("(a|b)*abb", "ab");

    assert_eq!(dfa.map.len(), 4);
}

#[test]
fn dfa_accept_test() {
    let mut dfa = DFA::parse("(a|b)*abb(a|b)*", "ab");

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

    dfa = DFA::parse("(a|b)*a(a|b)(a|b)", "ab");

    assert!(dfa.accept("abaab"));
    assert!(dfa.accept("abababb"));
    assert!(!dfa.accept("abba"));
    assert!(!dfa.accept(""));

    let dfa = dfa.minimize("ab");

    assert!(dfa.accept("abaab"));
    assert!(dfa.accept("abababb"));
    assert!(!dfa.accept("abba"));
    assert!(!dfa.accept(""));
}

#[test]
fn dfa_capture_test() {
    let mut dfa = DFA::parse("(a|b)*abb(a|b)*", "ab");

    assert_eq!(dfa.capture("ababbababccccc"), Some("ababbabab"));
    assert_eq!(dfa.capture("ababab"), None);

    dfa = dfa.minimize("ab");

    assert_eq!(dfa.capture("ababbababccccc"), Some("ababbabab"));
    assert_eq!(dfa.capture("ababab"), None);
}

#[test]
fn lookahead_test() {
    use crate::nfa::NFA;

    let (lhs, rhs) = NFA::parse_lookahead("(a|ab)/ba");
    let (lexeme, lookahead) = (lhs.to_dfa("ab").1, rhs.to_dfa("ab").1);

    assert_eq!(lexeme.capture_lookahead(&lookahead, "aba"), Some("a"));
    assert_eq!(lexeme.capture_lookahead(&lookahead, "abba"), Some("ab"));
    assert_eq!(lexeme.capture_lookahead(&lookahead, "baba"), None);
}
