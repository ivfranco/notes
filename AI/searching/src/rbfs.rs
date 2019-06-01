use num_traits::identities::Zero;
use std::cell::RefCell;
use std::cmp::{self, Ordering};
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum OrInfinite<C> {
    Finite(C),
    Infinite,
}

use OrInfinite::*;

type SharedNode<N, C> = Rc<RefCell<Node<N, C>>>;

struct Node<N, C> {
    parent: Option<SharedNode<N, C>>,
    state: N,
    cost: C,
    memo: OrInfinite<C>,
}

impl<N, C> Node<N, C> {
    fn new_shared(
        parent: SharedNode<N, C>,
        state: N,
        cost: C,
        memo: OrInfinite<C>,
    ) -> SharedNode<N, C> {
        Rc::new(RefCell::new(Node {
            parent: Some(parent),
            state,
            cost,
            memo,
        }))
    }

    fn new_init_shared(state: N) -> SharedNode<N, C>
    where
        C: Zero,
    {
        Rc::new(RefCell::new(Node {
            parent: None,
            state,
            cost: C::zero(),
            memo: Finite(C::zero()),
        }))
    }

    fn solution(&self) -> Vec<N>
    where
        N: Clone,
    {
        let mut path = vec![self.state.clone()];
        let mut node = self.parent.clone();
        while let Some(next) = node {
            path.push(next.borrow().state.clone());
            node = next.borrow().parent.clone();
        }

        path.reverse();
        path
    }
}

impl<N, C> PartialEq for Node<N, C>
where
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.memo == other.memo
    }
}

impl<N, C> Eq for Node<N, C> where C: Eq {}

impl<N, C> PartialOrd for Node<N, C>
where
    C: Eq + PartialOrd,
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.memo.partial_cmp(&rhs.memo).map(Ordering::reverse)
    }
}

impl<N, C> Ord for Node<N, C>
where
    C: Eq + Ord,
{
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.memo.cmp(&rhs.memo).reverse()
    }
}

enum SearchResult<N, C> {
    Success(Vec<N>, C),
    Failed(OrInfinite<C>),
}

use SearchResult::*;

fn rbfs<N, C, FN, FH, FS, IN>(
    start: SharedNode<N, C>,
    successors: &mut FN,
    heuristic: &mut FH,
    successful: &mut FS,
    limit: OrInfinite<C>,
) -> SearchResult<N, C>
where
    N: Clone + Eq + Hash,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    if successful(&start.borrow().state) {
        let path = start.borrow().solution();
        return Success(path, start.borrow().cost);
    }

    let mut succs: BinaryHeap<SharedNode<N, C>> = successors(&start.borrow().state)
        .into_iter()
        .map(|(state, step_cost)| {
            let cost = start.borrow().cost + step_cost;
            let memo = cmp::max(Finite(cost + heuristic(&state)), start.borrow().memo);
            Node::new_shared(start.clone(), state, cost, memo)
        })
        .collect();

    while let Some(best) = succs.pop() {
        if best.borrow().memo > limit {
            return Failed(best.borrow().memo);
        }

        let alt_memo = succs.peek().map_or(Infinite, |node| node.borrow().memo);
        let next_limit = cmp::min(limit, alt_memo);
        match rbfs(best.clone(), successors, heuristic, successful, next_limit) {
            s @ Success(..) => return s,
            Failed(memo) => best.borrow_mut().memo = memo,
        }

        succs.push(best);
    }

    Failed(Infinite)
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
    match rbfs(
        Node::new_init_shared(start.clone()),
        &mut successors,
        &mut heuristic,
        &mut successful,
        Infinite,
    ) {
        Success(path, cost) => Some((path, cost)),
        _ => None,
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
