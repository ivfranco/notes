use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};

pub type State = usize;

pub struct DFA {
    map: Vec<HashMap<char, State>>,
    finals: HashSet<State>,
}

impl DFA {
    pub fn new(n_state: usize, finals: &[State]) -> Self {
        DFA {
            map: vec![HashMap::new(); n_state],
            finals: finals.iter().cloned().collect(),
        }
    }

    pub fn init(&self) -> DFAState {
        DFAState::new(self)
    }

    pub fn install_transition(&mut self, from: State, symbol: char, to: State) {
        if self.map.len() <= from {
            self.map.resize(from + 1, HashMap::new());
        }
        self.map[from].insert(symbol, to);
    }

    pub fn install_final(&mut self, state: State) {
        self.finals.insert(state);
    }

    fn goto(&self, state: State, symbol: char) -> Option<State> {
        assert!(self.map.len() > state, "Out of bound state");

        self.map[state].get(&symbol).cloned()
    }

    pub fn accept(&self, string: &str) -> bool {
        let mut state = self.init();

        for symbol in string.chars() {
            state = state.next(symbol);
        }

        state.accepted()
    }
}

impl Debug for DFA {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Transitions:")?;

        for (from, trans) in self.map.iter().enumerate() {
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
            state: Some(0),
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
