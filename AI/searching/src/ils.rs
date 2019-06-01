use num_traits::identities::Zero;
use std::cmp::{self, Ordering};
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone)]
struct SmallestCost<N, C> {
    parent: Option<Rc<Self>>,
    state: N,
    cost: C,
}

impl<N, C> SmallestCost<N, C> {
    fn new_shared(parent: Rc<Self>, state: N, cost: C) -> Rc<Self> {
        Rc::new(SmallestCost {
            parent: Some(parent),
            state,
            cost,
        })
    }

    fn new_init_shared(state: N, cost: C) -> Rc<Self> {
        Rc::new(SmallestCost {
            parent: None,
            state,
            cost,
        })
    }

    fn path(&self) -> Vec<N>
    where
        N: Clone,
    {
        let mut path = vec![];
        let mut node = self;
        path.push(node.state.clone());

        while let Some(rc) = node.parent.as_ref() {
            node = rc;
            path.push(node.state.clone());
        }

        path.reverse();
        path
    }
}

impl<N, C> PartialEq for SmallestCost<N, C>
where
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl<N, C> Eq for SmallestCost<N, C> where C: Eq {}

impl<N, C> PartialOrd for SmallestCost<N, C>
where
    C: Eq + PartialOrd,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&rhs.cost).map(Ordering::reverse)
    }
}

impl<N, C> Ord for SmallestCost<N, C>
where
    C: Eq + Ord,
{
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.cost.cmp(&rhs.cost).reverse()
    }
}

#[derive(Debug)]
enum SearchResult<N, C> {
    Success(Vec<N>, C),
    Limited(C),
    Failed,
}

use SearchResult::*;

// for non-negative costs
fn uniform_cost_search<N, C, FN, FS, IN>(
    start: &N,
    mut successors: FN,
    mut successful: FS,
    limit: C,
) -> SearchResult<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut explored: HashSet<N> = HashSet::new();
    let mut frontier: BinaryHeap<Rc<SmallestCost<N, C>>> = BinaryHeap::new();
    let mut next_limit: Option<C> = None;

    frontier.push(SmallestCost::new_init_shared(start.clone(), C::zero()));

    while let Some(top) = frontier.pop() {
        if successful(&top.state) {
            return Success(top.path(), top.cost);
        }

        explored.insert(top.state.clone());

        for (succ, step_cost) in successors(&top.state) {
            if !explored.contains(&succ) {
                let cost = top.cost + step_cost;
                if cost > limit {
                    let c = *next_limit.get_or_insert(cost);
                    next_limit = Some(cmp::min(cost, c));
                } else {
                    frontier.push(SmallestCost::new_shared(top.clone(), succ, cost));
                }
            }
        }
    }

    if let Some(limit) = next_limit {
        Limited(limit)
    } else {
        Failed
    }
}

pub fn iterative_lengthening_search<N, C, FN, FS, IN>(
    start: &N,
    mut successors: FN,
    mut successful: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut limit: C = C::zero();

    loop {
        match uniform_cost_search(start, &mut successors, &mut successful, limit) {
            Success(path, cost) => break Some((path, cost)),
            Limited(next_limit) => limit = next_limit,
            Failed => break None,
        }
    }
}

#[test]
fn eight_puzzle_test() {
    use crate::eight_puzzle::Eight;
    use pathfinding::directed::astar::astar;
    use rand::prelude::*;

    let puzzle = random();

    let (_, ils_cost) = iterative_lengthening_search(
        &puzzle,
        |state| state.successors().into_iter().map(|succ| (succ, 1)),
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

    assert_eq!(ils_cost, astar_cost);
}
