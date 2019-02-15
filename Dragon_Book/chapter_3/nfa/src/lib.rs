use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};

type State = usize;
type Trans = HashMap<Option<char>, Vec<State>>;

pub struct NFA {
    map: Vec<Trans>,
    finals: HashSet<State>,
    pub states: HashSet<State>,
}

impl NFA {
    pub fn new(n_state: usize, finals: &[State]) -> Self {
        NFA {
            map: vec![HashMap::new(); n_state],
            finals: finals.iter().cloned().collect(),
            states: HashSet::new(),
        }
    }

    pub fn install_transition(&mut self, from: State, symbol: Option<char>, to: Vec<State>) {
        self.map[from].insert(symbol, to);
    }

    pub fn init(&mut self) {
        self.states.clear();
        self.states.insert(0);
        self.states = extend_with_empty(self, &self.states);
    }

    pub fn next(&mut self, symbol: char) {
        self.states = next(self, &self.states, symbol);
    }

    fn accepted(&self) -> bool {
        !self.states.is_disjoint(&self.finals)
    }

    pub fn accept(&mut self, string: &str) -> bool {
        self.init();

        for c in string.chars() {
            self.next(c);
        }

        self.accepted()
    }

    pub fn all_paths(&self, string: &str) -> Vec<Vec<State>> {
        all_paths(self, string)
    }
}

fn consume(nfa: &NFA, states: &HashSet<State>, symbol: Option<char>) -> HashSet<State> {
    let mut new_states: HashSet<State> = HashSet::new();

    for state in states.iter() {
        if let Some(tos) = nfa.map[*state].get(&symbol) {
            new_states.extend(tos);
        }
    }

    new_states
}

fn extend_with_empty(nfa: &NFA, states: &HashSet<State>) -> HashSet<State> {
    let mut empty_extended = states.clone();
    let mut size = empty_extended.len();

    // extend the states with ε transitions until the set of states no longer grow
    loop {
        let new_states = consume(nfa, &empty_extended, None);
        empty_extended.extend(new_states);
        if empty_extended.len() == size {
            break;
        } else {
            size = empty_extended.len();
        }
    }

    empty_extended
}

fn next(nfa: &NFA, states: &HashSet<State>, symbol: char) -> HashSet<State> {
    let new_states = consume(nfa, states, Some(symbol));
    extend_with_empty(nfa, &new_states)
}

fn all_paths(nfa: &NFA, string: &str) -> Vec<Vec<State>> {
    let mut paths = vec![vec![0]];

    for c in string.chars() {
        paths = paths
            .into_iter()
            .flat_map(|path| {
                let last = *path.last().unwrap();
                let mut states = HashSet::new();
                states.insert(last);
                next(nfa, &states, c).into_iter().map(move |state| {
                    let mut next = path.clone();
                    next.push(state);
                    next
                })
            })
            .collect();
    }

    paths
}

impl Debug for NFA {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (from, trans) in self.map.iter().enumerate() {
            for (symbol, tos) in trans {
                if let Some(c) = symbol {
                    write!(f, "δ({}, {}) = ", from, c)?;
                } else {
                    write!(f, "δ({}, ε) = ", from)?;
                }
                f.debug_set().entries(tos).finish()?;
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

#[test]
fn all_path_test() {
    let mut nfa = NFA::new(4, &[3]);
    nfa.install_transition(0, Some('a'), vec![0, 1]);
    nfa.install_transition(0, Some('b'), vec![0]);
    nfa.install_transition(1, Some('b'), vec![2]);
    nfa.install_transition(2, Some('b'), vec![3]);

    let paths = nfa.all_paths("aabb");

    assert_eq!(paths.len(), 2);
    assert!(paths.contains(&vec![0, 0, 1, 2, 3]));
    assert!(paths.contains(&vec![0, 0, 0, 0, 0]));
}
