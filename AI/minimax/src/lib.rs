use num_traits::identities::{One, Zero};
use ordered_float::NotNan;

pub type Util = NotNan<f64>;
type Prob = NotNan<f64>;

enum Node<S> {
    Min(S),
    Max(S),
    Chance(S),
}

use Node::*;

impl<S> Node<S> {
    fn as_state(&self) -> &S {
        match self {
            Min(s) => &s,
            Max(s) => &s,
            Chance(s) => &s,
        }
    }
}

pub struct ChanceNode<S> {
    node: Node<S>,
    prob: Prob,
}

impl<S> ChanceNode<S> {
    fn new(node: Node<S>, prob: Prob) -> Self {
        ChanceNode { node, prob }
    }

    pub fn min(state: S) -> Self {
        ChanceNode::new(Min(state), Prob::one())
    }

    pub fn max(state: S) -> Self {
        ChanceNode::new(Max(state), Prob::one())
    }

    pub fn chance(state: S) -> Self {
        ChanceNode::new(Chance(state), Prob::one())
    }

    pub fn prob(self, prob: f64) -> Self {
        let node = self.node;
        ChanceNode {
            node,
            prob: Prob::new(prob).expect("ChanceNode initialization: probability is NaN"),
        }
    }

    fn as_state(&self) -> &S {
        self.node.as_state()
    }
}

pub trait State: Sized {
    type Action;

    fn actions(&self) -> Vec<Self::Action>;
    fn result(&self, action: &Self::Action) -> ChanceNode<Self>;
    fn utility(&self) -> Option<f64>;
}

fn results<'a, S>(state: &'a S) -> impl Iterator<Item = ChanceNode<S>> + 'a
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
    state
        .actions()
        .into_iter()
        .max_by_key(|action| value_of(&state.result(action)))
        .unwrap()
}

fn value_of<S>(chance_node: &ChanceNode<S>) -> Util
where
    S: State,
{
    if let Some(util) = chance_node.as_state().utility() {
        return NotNan::new(util).expect("value_of: Utility value is NaN");
    }

    match &chance_node.node {
        Min(state) => results(state).map(|node| value_of(&node)).min().unwrap(),
        Max(state) => results(state).map(|node| value_of(&node)).max().unwrap(),
        Chance(state) => results(state)
            .map(|node| value_of(&node) * node.prob)
            .fold(NotNan::zero(), |sum, elem| sum + elem),
    }
}
