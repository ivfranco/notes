pub mod othello;
pub mod tic_tac_toe;

use num_traits::identities::{One, Zero};
use ordered_float::NotNan;

use std::{cmp, f64};

type Util = NotNan<f64>;
type Prob = NotNan<f64>;

enum NodeType {
    Min,
    Max,
    Chance,
}

use NodeType::*;

pub struct Node<S> {
    node_type: NodeType,
    state: S,
    prob: Prob,
}

impl<S> Node<S> {
    fn new(node_type: NodeType, state: S, prob: Prob) -> Self {
        Node { node_type, state, prob }
    }

    pub fn min(state: S) -> Self {
        Node::new(Min, state, Prob::one())
    }

    pub fn max(state: S) -> Self {
        Node::new(Max, state, Prob::one())
    }

    pub fn chance(state: S) -> Self {
        Node::new(Chance, state, Prob::one())
    }

    pub fn set_prob(self, prob: f64) -> Self {
        Node {
            prob: Prob::new(prob).expect("ChanceNode initialization: probability is NaN"),
            ..self
        }
    }

    fn as_state(&self) -> &S {
        &self.state
    }
}

pub trait State: Sized {
    type Action;

    fn actions(&self) -> Vec<Self::Action>;
    fn result(&self, action: &Self::Action) -> Node<Self>;
    fn utility(&self) -> Option<f64>;
}

fn successors<'a, S>(state: &'a S) -> impl Iterator<Item = Node<S>> + 'a
where
    S: State,
{
    state
        .actions()
        .into_iter()
        .map(move |action| state.result(&action))
}

pub fn minimax<S>(state: S) -> S::Action
where
    S: State,
{
    let alpha = Util::new(f64::NEG_INFINITY).unwrap();
    let beta = Util::new(f64::INFINITY).unwrap();

    state
        .actions()
        .into_iter()
        .max_by_key(|action| value_of(&state.result(action), alpha, beta))
        .unwrap()
}

fn value_of<S>(node: &Node<S>, mut alpha: Util, mut beta: Util) -> Util
where
    S: State,
{
    if let Some(util) = node.as_state().utility() {
        return Util::new(util).expect("value_of: Utility value is NaN");
    }

    let state = node.as_state();

    match &node.node_type {
        Max => {
            let mut v = Util::new(f64::NEG_INFINITY).unwrap();

            for succ in successors(state) {
                v = cmp::max(v, value_of(&succ, alpha, beta));
                if v >= beta {
                    break;
                } else {
                    alpha = cmp::max(alpha, v);
                }
            }

            v
        }
        Min => {
            let mut v = Util::new(f64::INFINITY).unwrap();

            for succ in successors(state) {
                v = cmp::min(v, value_of(&succ, alpha, beta));
                if v <= alpha {
                    break;
                } else {
                    beta = cmp::min(beta, v);
                }
            }

            v
        }
        Chance => successors(state)
            .map(|node| value_of(&node, alpha, beta) * node.prob)
            .fold(Util::zero(), |sum, elem| sum + elem),
    }
}

#[test]
fn tic_tac_toe_test() {
    use crate::tic_tac_toe::TicTacToe;

    let init = TicTacToe::init();
    let node = Node::max(init);

    let alpha = Util::new(f64::NEG_INFINITY).unwrap();
    let beta = Util::new(f64::INFINITY).unwrap();

    assert!(
        value_of(&node, alpha, beta).is_zero(),
        "Tic-Tac-Toe should always end in a tie with two optimal players",
    );
}
