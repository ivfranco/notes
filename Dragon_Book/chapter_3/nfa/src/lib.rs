use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};

type State = usize;
type Trans = HashMap<Option<char>, Vec<State>>;

pub struct NFA {
    map: Vec<Trans>,
    finals: HashSet<State>,
}

impl NFA {
    pub fn new(n_state: usize, finals: &[State]) -> Self {
        NFA {
            map: vec![HashMap::new(); n_state],
            finals: finals.iter().cloned().collect(),
        }
    }

    pub fn install_transition(&mut self, from: State, symbol: Option<char>, to: Vec<State>) {
        if self.map.len() <= from {
            self.map.resize(from + 1, HashMap::new());
        }
        self.map[from].insert(symbol, to);
    }

    pub fn install_final(&mut self, state: State) {
        self.finals.insert(state);
    }

    pub fn init(&self) -> HashSet<State> {
        let mut state = HashSet::new();
        state.insert(0);
        self.extend_with_empty(&state)
    }

    pub fn next(&self, states: &HashSet<State>, symbol: char) -> HashSet<State> {
        let new_states = self.consume(states, Some(symbol));
        self.extend_with_empty(&new_states)
    }

    fn consume(&self, states: &HashSet<State>, symbol: Option<char>) -> HashSet<State> {
        let mut new_states: HashSet<State> = HashSet::new();

        for state in states.iter() {
            assert!(*state < self.map.len(), "Out of bound state");
            if let Some(tos) = self.map[*state].get(&symbol) {
                new_states.extend(tos);
            }
        }

        new_states
    }

    fn extend_with_empty(&self, states: &HashSet<State>) -> HashSet<State> {
        let mut empty_extended = states.clone();
        let mut size = empty_extended.len();

        // extend the states with ε transitions until the set of states no longer grow
        loop {
            let new_states = self.consume(&empty_extended, None);
            empty_extended.extend(new_states);
            if empty_extended.len() == size {
                break;
            } else {
                size = empty_extended.len();
            }
        }

        empty_extended
    }

    fn accepted(&self, states: &HashSet<State>) -> bool {
        states.is_disjoint(&self.finals)
    }

    pub fn accept(&self, string: &str) -> bool {
        let mut states = self.init();

        for c in string.chars() {
            states = self.next(&states, c);
        }

        self.accepted(&states)
    }

    pub fn all_paths(&self, string: &str) -> Vec<Vec<State>> {
        let mut paths = vec![vec![0]];

        for c in string.chars() {
            paths = paths
                .into_iter()
                .flat_map(|path| {
                    let last = *path.last().unwrap();
                    let mut states = HashSet::new();
                    states.insert(last);
                    self.next(&states, c).into_iter().map(move |state| {
                        let mut next = path.clone();
                        next.push(state);
                        next
                    })
                })
                .collect();
        }

        paths
    }

    pub fn is_dfa(&self) -> bool {
        self.map.iter().all(|trans| {
            trans
                .iter()
                .all(|(symbol, to)| symbol.is_some() && to.len() == 1)
        })
    }

    pub fn to_dfa(&self, alphabet: &str) -> (HashMap<Vec<State>, State>, NFA) {
        let init = self.init();
        let mut set_to_state: HashMap<Vec<State>, State> = HashMap::new();
        set_to_state.insert(sorted(&init), 0);
        let mut stack: Vec<HashSet<State>> = vec![self.init()];
        let mut max_state: State = 0;
        let mut dfa = NFA::new(0, &[]);

        while let Some(states) = stack.pop() {
            for c in alphabet.chars() {
                let from = set_to_state[&sorted(&states)];
                let next = self.next(&states, c);
                if !next.is_empty() {
                    let is_final = self.accepted(&next);
                    let to = *set_to_state.entry(sorted(&next)).or_insert_with(|| {
                        max_state += 1;
                        stack.push(next);
                        max_state
                    });
                    dfa.install_transition(from, Some(c), vec![to]);
                    if is_final {
                        dfa.install_final(to);
                    }
                }
            }
        }

        (set_to_state, dfa)
    }
}

fn sorted(states: &HashSet<State>) -> Vec<State> {
    let mut vec: Vec<State> = states.iter().cloned().collect();
    vec.sort();
    vec
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

#[test]
fn to_dfa_test() {
    let mut nfa = NFA::new(4, &[3]);
    nfa.install_transition(0, Some('a'), vec![1]);
    nfa.install_transition(0, None, vec![3]);
    nfa.install_transition(1, Some('b'), vec![2]);
    nfa.install_transition(1, None, vec![0]);
    nfa.install_transition(2, Some('b'), vec![3]);
    nfa.install_transition(2, None, vec![1]);
    nfa.install_transition(3, Some('a'), vec![0]);
    nfa.install_transition(3, None, vec![2]);

    let (mapping, dfa) = nfa.to_dfa("ab");
    assert!(dfa.is_dfa());
    assert_eq!(mapping.len(), 1);
    assert_eq!(mapping.keys().next().unwrap(), &[0, 1, 2, 3]);
}
