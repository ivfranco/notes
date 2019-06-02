use crate::min_heap::MinHeap;
use indexmap::IndexSet;
use num_traits::identities::Zero;
use std::cmp;
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum OrInfinite<C> {
    Finite(C),
    Infinite,
}

use OrInfinite::*;

enum SearchResult<N, C> {
    Success(Vec<N>, C),
    Failed(OrInfinite<C>),
}

use SearchResult::*;

type NodeID = usize;
type StateID = usize;

#[derive(Clone, Copy)]
struct RBFSNode<C> {
    parent: Option<NodeID>,
    state: StateID,
    cost: C,
}

impl<C> RBFSNode<C>
where
    C: Zero,
{
    fn new_init(state: StateID) -> Self {
        RBFSNode {
            parent: None,
            state,
            cost: C::zero(),
        }
    }

    fn new(parent: NodeID, state: StateID, cost: C) -> Self {
        RBFSNode {
            parent: Some(parent),
            state,
            cost,
        }
    }
}

struct RBFS<N, C> {
    nodes: Vec<RBFSNode<C>>,
    states: IndexSet<N>,
}

impl<N, C> RBFS<N, C>
where
    N: Clone + Eq + Hash,
    C: Zero + Ord + Copy,
{
    fn new(start: N) -> Self {
        let mut states = IndexSet::new();
        let (id, _) = states.insert_full(start);

        RBFS {
            nodes: vec![RBFSNode::new_init(id)],
            states,
        }
    }

    fn push_node(&mut self, node: RBFSNode<C>) -> NodeID {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    fn state_id(&mut self, state: N) -> StateID {
        if let Some((id, _)) = self.states.get_full(&state) {
            id
        } else {
            let (id, _) = self.states.insert_full(state);
            id
        }
    }

    fn get_state(&self, id: StateID) -> &N {
        self.states.get_index(id).unwrap()
    }

    fn path(&self, i: NodeID) -> Vec<N> {
        let mut id = Some(i);
        let mut path = vec![];

        while let Some(i) = id {
            let node = &self.nodes[i];
            id = node.parent;
            path.push(self.get_state(node.state).clone());
        }

        path.reverse();
        path
    }

    fn search<FN, FH, FS, IN>(
        &mut self,
        start_id: NodeID,
        start_memo: OrInfinite<C>,
        successors: &mut FN,
        heuristic: &mut FH,
        successful: &mut FS,
        limit: OrInfinite<C>,
    ) -> SearchResult<N, C>
    where
        FN: FnMut(&N) -> IN,
        IN: IntoIterator<Item = (N, C)>,
        FH: FnMut(&N) -> C,
        FS: FnMut(&N) -> bool,
    {
        let node = self.nodes[start_id];
        let start = self.get_state(node.state).clone();
        if successful(&start) {
            return Success(self.path(start_id), node.cost);
        }

        let mut succs: MinHeap<OrInfinite<C>, NodeID> = MinHeap::new();

        for (state, step_cost) in successors(&start) {
            let cost = node.cost + step_cost;
            let memo = cmp::max(Finite(cost + heuristic(&state)), start_memo);
            let state_id = self.state_id(state);
            let succ = RBFSNode::new(start_id, state_id, cost);
            let succ_id = self.push_node(succ);
            succs.push(memo, succ_id);
        }

        while let Some((best_memo, best_id)) = succs.pop() {
            if best_memo > limit {
                return Failed(best_memo);
            }

            let alt_memo = succs.peek().map_or(Infinite, |(alt_memo, _)| *alt_memo);
            let next_limit = cmp::min(limit, alt_memo);
            match self.search(
                best_id, best_memo, successors, heuristic, successful, next_limit,
            ) {
                s @ Success(..) => return s,
                Failed(memo) => succs.push(memo, best_id),
            }
        }

        Failed(Infinite)
    }
}

pub fn recursive_best_first_search<N, C, FN, FH, FS, IN>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut successful: FS,
) -> Option<(Vec<N>, C)>
where
    N: Clone + Eq + Hash,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let mut rbfs_env = RBFS::new(start.clone());
    match rbfs_env.search(0, Finite(C::zero()), &mut successors, &mut heuristic, &mut successful, Infinite) {
        Success(path, cost) => Some((path, cost)),
        Failed(..) => None,
    }
}

#[test]
fn eight_puzzle_test() {
    use crate::eight_puzzle::Eight;
    use pathfinding::directed::astar::astar;
    use rand::prelude::*;

    let puzzle = random();

    let (_, rbfs_cost) = recursive_best_first_search(
        &puzzle,
        |state| state.successors().into_iter().map(|succ| (succ, 1)),
        Eight::heuristic,
        Eight::is_goal,
    )
    .unwrap();

    let (_, astar_cost) = astar(
        &puzzle,
        |state| state.successors().into_iter().map(|succ| (succ, 1)),
        Eight::heuristic,
        Eight::is_goal,
    )
    .unwrap();

    assert_eq!(rbfs_cost, astar_cost);
}
