use petgraph::prelude::*;
use std::{
    collections::{HashSet, HashMap},
    hash::Hash,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cleanliness {
    Clean,
    Dirty,
}

use Cleanliness::*;

impl Cleanliness {
    fn label(self) -> u8 {
        match self {
            Clean => 1,
            Dirty => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Pos {
    Left,
    Right,
}

use Pos::*;

impl Pos {
    fn label(self) -> u8 {
        match self {
            Left => 0,
            Right => 1,
        }
    }

    fn other(self) -> Self {
        match self {
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Suck,
    Left,
    Right,
}

impl Action {
    fn enumerate() -> Vec<Action> {
        vec![Action::Suck, Action::Left, Action::Right]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Erratic {
    left: Cleanliness,
    right: Cleanliness,
    agent: Pos,
}

impl Erratic {
    pub fn enumerate() -> HashSet<Self> {
        let mut set = HashSet::new();

        for &left in &[Clean, Dirty] {
            for &right in &[Clean, Dirty] {
                for &agent in &[Left, Right] {
                    let state = Erratic {
                        left, right, agent,
                    };
                    set.insert(state);
                }
            }
        }

        set
    }

    pub fn label(self) -> u8 {
        self.agent.label()
            + (self.right.label() << 1)
            + (self.left.label() << 2)
            + 1
    }

    fn square_mut(&mut self, pos: Pos) -> &mut Cleanliness {
        match pos {
            Left => &mut self.left,
            Right => &mut self.right,
        }
    }

    fn square(self, pos: Pos) -> Cleanliness {
        match pos {
            Left => self.left,
            Right => self.right,
        }
    }

    fn clean(self) -> Vec<Self> {
        let pos = self.agent;

        if self.square(pos) == Clean {
            let mut dumped = self;
            *dumped.square_mut(pos) = Dirty;
            vec![self, dumped]
        } else {
            let mut results = vec![];

            let mut cleaned = self;
            *cleaned.square_mut(pos) = Clean;
            results.push(cleaned);

            if self.square(pos.other()) == Dirty {
                results.push(Erratic {
                    left: Clean,
                    right: Clean,
                    ..self
                });
            }

            results
        }
    }

    fn left(self) -> Self {
        Erratic {
            agent: Left,
            ..self
        }
    }

    fn right(self) -> Self {
        Erratic {
            agent: Right,
            ..self
        }
    }

    fn results(self, action: Action) -> Vec<Self> {
        match action {
            Action::Suck => self.clean(),
            Action::Left => vec![self.left()],
            Action::Right => vec![self.right()],
        }
    }
}

fn labels(believe: &HashSet<Erratic>) -> Vec<u8> {
    let mut labels: Vec<_> = believe.iter().map(|state| state.label()).collect();
    labels.sort();
    labels
}

fn touch<N, E>(node: N, graph: &mut Graph<N, E>, explored: &mut HashMap<N, NodeIndex>) -> NodeIndex 
where N: Clone + Eq + Hash,
{
    if let Some(idx) = explored.get(&node) {
        *idx
    } else {
        let idx = graph.add_node(node.clone());
        explored.insert(node, idx);
        idx
    }
}

pub fn explore(init: HashSet<Erratic>) -> Graph<Vec<u8>, Action, Directed> {
    let mut graph = Graph::new();
    let mut explored: HashMap<Vec<u8>, NodeIndex> = HashMap::new();

    let init_labels = labels(&init);
    let init_idx = touch(init_labels, &mut graph, &mut explored);

    let mut frontier = vec![(init_idx, init)];

    while let Some((from, believe)) = frontier.pop() {
        for action in Action::enumerate() {
            let succ: HashSet<_> = believe.iter().flat_map(|state| state.results(action)).collect();
            let labels = labels(&succ);
            let visited = explored.contains_key(&labels);

            let to = touch(labels, &mut graph, &mut explored);
            graph.add_edge(from, to, action);

            if !visited {
                frontier.push((to, succ));
            }
        }
    }
    
    graph
}