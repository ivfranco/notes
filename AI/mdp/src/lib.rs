pub mod worlds;

use std::cmp::Ordering;

pub type Prob = f64;
pub type Util = f64;

pub trait State: Sized {
    fn to_usize(&self) -> usize;
    fn from_usize(i: usize) -> Self;
}

pub trait MDP {
    type State: State;
    type Action;

    fn states(&self) -> usize;
    fn discount(&self) -> Util;
    fn reward(&self, state: &Self::State) -> Util;
    fn actions(&self, state: &Self::State) -> Vec<Self::Action>;
    fn apply(&self, state: &Self::State, action: &Self::Action) -> Vec<(Prob, Self::State)>;
}

const ALMOST_ONE: f64 = 0.999_999_999;

pub fn value_iteration<M>(mdp: &M, error: Prob) -> Vec<Util>
where
    M: MDP,
{
    let mut utils = vec![0.0; mdp.states()];

    loop {
        let next_utils = next_utils(mdp, &utils);
        let max_norm = max_norm(&utils, &next_utils);
        // when discount >= 1, the loop will not terminate
        // the error threshold is loosened a little bit in that case
        let gamma = if mdp.discount() >= 1.0 {
            ALMOST_ONE
        } else {
            mdp.discount()
        };

        if max_norm < error * (1.0 - gamma) / gamma {
            return next_utils;
        } else {
            utils = next_utils;
        }
    }
}

fn next_utils<M>(mdp: &M, utils: &[Util]) -> Vec<Util>
where
    M: MDP,
{
    let mut next_utils = vec![0.0; utils.len()];
    for (i, u) in next_utils.iter_mut().enumerate() {
        let state = M::State::from_usize(i);
        let max_action = mdp
            .actions(&state)
            .into_iter()
            .map(|action| {
                expected_util(mdp, &state, &action, utils)
            })
            .max_by(|&a, &b| cmp_f64(a, b))
            .unwrap_or(0.0);

        *u = mdp.discount() * max_action + mdp.reward(&state);
    }
    next_utils
}

fn expected_util<M>(mdp: &M, state: &M::State, action: &M::Action, utils: &[Util]) -> Util
where
    M: MDP,
{
    mdp.apply(state, action)
        .into_iter()
        .map(|(p, s)| p * utils[s.to_usize()])
        .sum()
}

fn max_norm(utils: &[Util], next_utils: &[Util]) -> Util {
    utils
        .iter()
        .zip(next_utils)
        .map(|(a, b)| (a - b).abs())
        .max_by(|&a, &b| cmp_f64(a, b))
        .unwrap_or(0.0)
}

fn cmp_f64(a: f64, b: f64) -> Ordering {
    assert!(!a.is_nan());
    assert!(!b.is_nan());

    a.partial_cmp(&b).unwrap()
}

pub fn policy_from<M>(mdp: &M, utils: &[Util]) -> Vec<Option<M::Action>>
where
    M: MDP,
{
    (0 .. mdp.states())
        .map(|i| {
            let state = M::State::from_usize(i);
            mdp.actions(&state)
                .into_iter()
                .max_by(|a, b| {
                    let ua = expected_util(mdp, &state, a, utils);
                    let ub = expected_util(mdp, &state, b, utils);
                    ua.partial_cmp(&ub).unwrap()
                })
        })
        .collect()
}