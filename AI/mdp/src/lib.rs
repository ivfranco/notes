pub mod worlds;

use rand::prelude::*;
use std::cmp::Ordering;

pub type Prob = f64;
pub type Util = f64;

pub trait MDP {
    type State;
    type Action;

    fn states(&self) -> usize;
    fn discount(&self) -> Util;
    fn encode(&self, state: &Self::State) -> usize;
    fn decode(&self, i: usize) -> Self::State;
    fn reward(&self, state: &Self::State) -> Util;
    fn apply(&self, state: &Self::State, action: &Self::Action) -> Vec<(Prob, Self::State)>;
}

pub trait SoloMDP: MDP {
    fn actions(&self, state: &Self::State) -> Vec<Self::Action>;
}

const ALMOST_ONE: f64 = 0.999_999_999;

pub fn value_iteration<M>(mdp: &M, error: Prob) -> Vec<Util>
where
    M: SoloMDP,
{
    let mut utils = vec![0.0; mdp.states()];

    loop {
        let next_utils = bellman_update(mdp, &utils);
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

fn bellman_update<M>(mdp: &M, utils: &[Util]) -> Vec<Util>
where
    M: SoloMDP,
{
    let mut next_utils = vec![0.0; utils.len()];
    for (i, u) in next_utils.iter_mut().enumerate() {
        let state = mdp.decode(i);
        let max_action = mdp
            .actions(&state)
            .into_iter()
            .map(|action| expected_util(mdp, &state, &action, utils))
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
        .map(|(p, s)| p * utils[mdp.encode(&s)])
        .sum()
}

fn best_action<M>(mdp: &M, state: &M::State, utils: &[Util]) -> Option<M::Action>
where
    M: SoloMDP,
{
    mdp.actions(&state).into_iter().max_by(|a, b| {
        let ua = expected_util(mdp, &state, a, utils);
        let ub = expected_util(mdp, &state, b, utils);
        cmp_f64(ua, ub)
    })
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

#[allow(type_alias_bounds)]
type Policy<M: SoloMDP> = Vec<Option<M::Action>>;

pub fn policy_from<M>(mdp: &M, utils: &[Util]) -> Policy<M>
where
    M: SoloMDP,
{
    (0..mdp.states())
        .map(|i| {
            let state = mdp.decode(i);
            best_action(mdp, &state, utils)
        })
        .collect()
}

const REPEAT: usize = 100;

pub fn policy_iteration<M>(mdp: &M) -> Policy<M>
where
    M: SoloMDP,
    M::Action: PartialEq + std::fmt::Debug,
{
    let mut rng = thread_rng();
    let mut policy: Policy<M> = (0..mdp.states())
        .map(|i| {
            let state = mdp.decode(i);
            mdp.actions(&state).into_iter().choose(&mut rng)
        })
        .collect();
    let mut utils = vec![0.0; mdp.states()];

    loop {
        utils = policy_evaluation(mdp, &policy, &utils, REPEAT);
        let mut next_policy = vec![];
        for i in 0..mdp.states() {
            let state = mdp.decode(i);
            let action = best_action(mdp, &state, &utils);
            next_policy.push(action);
        }

        if policy == next_policy {
            return policy;
        } else {
            policy = next_policy;
        }
    }
}

fn policy_evaluation<M>(
    mdp: &M,
    policy: &[Option<M::Action>],
    utils: &[Util],
    k: usize,
) -> Vec<Util>
where
    M: SoloMDP,
{
    let mut utils = utils.to_vec();

    for _ in 0..k {
        let next_utils = policy
            .iter()
            .enumerate()
            .map(|(i, paction)| {
                let state = mdp.decode(i);
                let mut u = match paction {
                    Some(action) => expected_util(mdp, &state, action, &utils),
                    _ => 0.0,
                };
                u = mdp.reward(&state) + mdp.discount() * u;
                u
            })
            .collect();

        utils = next_utils;
    }

    utils
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Maxer,
    Miner,
}

impl Player {
    fn turn(self) -> Self {
        match self {
            Maxer => Miner,
            Miner => Maxer,
        }
    }
}

use Player::*;

pub trait ZeroSumMDP: MDP {
    fn actions(&self, state: &Self::State, player: Player) -> Vec<Self::Action>;
}

pub fn zero_sum_value_iteration<M>(mdp: &M, error: Prob) -> (Vec<Util>, Vec<Util>)
where
    M: ZeroSumMDP,
{
    let mut maxer_utils = vec![0.0; mdp.states()];
    let mut miner_utils = vec![0.0; mdp.states()];
    let mut mover = Maxer;
    let mut maxer_stable = false;
    let mut miner_stable = false;

    loop {
        let (player_utils, opponent_utils) = if mover == Maxer {
            (&mut maxer_utils, &miner_utils)
        } else {
            (&mut miner_utils, &maxer_utils)
        };

        let next_utils = zero_sum_bellman_update(mdp, mover, opponent_utils);
        let max_norm = max_norm(player_utils, &next_utils);
        let gamma = if mdp.discount() >= 1.0 {
            ALMOST_ONE
        } else {
            mdp.discount()
        };

        let stable = match mover {
            Maxer => &mut maxer_stable,
            Miner => &mut miner_stable,
        };

        if max_norm < error * (1.0 - gamma) / gamma {
            *stable = true;
        } else {
            *stable = false;
        }

        *player_utils = next_utils;

        if maxer_stable && miner_stable {
            return (maxer_utils, miner_utils);
        }

        mover = mover.turn();
    }
}

fn zero_sum_bellman_update<M>(mdp: &M, player: Player, opponent_utils: &[Util]) -> Vec<Util>
where
    M: ZeroSumMDP,
{
    let mut player_utils = vec![0.0; opponent_utils.len()];
    for (i, u) in player_utils.iter_mut().enumerate() {
        let state = mdp.decode(i);
        let action_results = mdp
            .actions(&state, player)
            .into_iter()
            .map(|action| expected_util(mdp, &state, &action, opponent_utils));

        let best_result = if player == Maxer {
            action_results.max_by(|&a, &b| cmp_f64(a, b))
        } else {
            action_results.min_by(|&a, &b| cmp_f64(a, b))
        }
        .unwrap_or(0.0);

        *u = mdp.discount() * best_result + mdp.reward(&state);
    }
    player_utils
}
