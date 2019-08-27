use super::*;
use rand::prelude::*;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub enum SimulateError {
    Trapped,
    InvalidAction,
}

use SimulateError::*;

pub trait Simulate: SoloMDP {
    fn start(&self) -> Self::State;
}

pub struct Simulator<'a, M>
where
    M: Simulate,
{
    world: &'a M,
    state: M::State,
}

impl<'a, M> Simulator<'a, M>
where
    M: Simulate,
{
    pub fn new(world: &'a M) -> Self {
        Simulator {
            world,
            state: world.start(),
        }
    }

    pub fn restart(&mut self) {
        self.state = self.world.start();
    }

    pub fn actions(&self) -> Vec<M::Action> {
        self.world.actions(&self.state)
    }

    pub fn state(&self) -> &M::State {
        &self.state
    }

    pub fn state_code(&self) -> usize {
        self.world.encode(self.state())
    }

    pub fn reward(&self) -> Util {
        self.world.reward(self.state())
    }

    pub fn trial(&mut self, action: &M::Action) -> Result<(), SimulateError> {
        let mut rng = thread_rng();
        let mut successors = self.world.apply(&self.state, action);
        let indices: Vec<_> = (0..successors.len()).collect();
        let succ = indices
            .choose_weighted(&mut rng, |&i| successors[i].0)
            .map(|&i| successors.swap_remove(i).1)
            .map_err(|_| InvalidAction)?;

        self.state = succ;
        Ok(())
    }

    pub fn trial_on_policy(&mut self, policy: &Policy<M>) -> Result<(), SimulateError> {
        let code = self.state_code();
        let action = policy[code].as_ref().ok_or(Trapped)?;
        self.trial(action)
    }
}

pub struct Trials<'a, M>
where
    M: Simulate,
{
    simulator: &'a mut Simulator<'a, M>,
    policy: &'a Policy<M>,
    trapped: bool,
}

impl<'a, M> Trials<'a, M>
where
    M: Simulate,
{
    pub fn into_inner(self) -> &'a mut Simulator<'a, M> {
        self.simulator
    }
}

impl<'a, M> Iterator for Trials<'a, M>
where
    M: Simulate,
    M::State: Clone,
{
    type Item = (M::State, Util);

    fn next(&mut self) -> Option<Self::Item> {
        if self.trapped {
            None
        } else {
            let state = self.simulator.state.clone();
            let reward = self.simulator.reward();
            if self.simulator.trial_on_policy(self.policy).is_err() {
                self.trapped = true;
            }
            Some((state, reward))
        }
    }
}

pub fn trials<'a, M>(simulator: &'a mut Simulator<'a, M>, policy: &'a Policy<M>) -> Trials<'a, M>
where
    M: Simulate,
{
    Trials {
        simulator,
        policy,
        trapped: false,
    }
}

/// n here determines the number of trial sequences
/// (i.e. the number of restarts of the simulator upon being trapped inside terminal states),
/// not number of individual trials
pub fn direct_utility_estimate<M>(mdp: &M, policy: &Policy<M>, n: u32) -> Vec<Util>
where
    M: Simulate,
    M::State: Clone,
{
    let mut simulator = Simulator::new(mdp);
    let mut utils = vec![0.0; mdp.states()];
    let mut utils_cnt = vec![0; mdp.states()];

    // why is this necessary
    let mut simu_mut = &mut simulator;
    for _ in 0..n {
        let mut trials = trials(simu_mut, policy);
        let trial_seq: Vec<_> = (&mut trials).collect();
        let mut acc = 0.0;
        for (state, reward) in trial_seq.into_iter().rev() {
            acc += reward;
            let code = mdp.encode(&state);
            utils[code] += acc;
            utils_cnt[code] += 1;
        }

        simu_mut = trials.into_inner();
        simu_mut.restart();
    }

    for (u, cnt) in utils.iter_mut().zip(utils_cnt) {
        if cnt > 0 {
            *u /= f64::from(cnt);
        }
    }
    utils
}

pub fn learn_factor(n: u32) -> f64 {
    1.0 / f64::from(n).sqrt()
}

pub fn temporal_difference<M>(mdp: &M, policy: &Policy<M>, n: u32) -> Vec<Util>
where
    M: Simulate,
{
    let mut simulator = Simulator::new(mdp);
    let mut utils = vec![0.0; mdp.states()];
    let mut util_cnt = vec![0; mdp.states()];

    for _ in 0..n {
        let prev_code = simulator.state_code();
        let prev_reward = simulator.reward();

        if util_cnt[prev_code] == 0 {
            utils[prev_code] = prev_reward;
        }

        if simulator.trial_on_policy(policy).is_err() {
            simulator.restart();
        } else {
            util_cnt[prev_code] += 1;
            let succ_code = simulator.state_code();
            let succ_reward = simulator.reward();

            if util_cnt[succ_code] == 0 {
                utils[succ_code] = succ_reward;
            }

            utils[prev_code] += learn_factor(util_cnt[prev_code])
                * (prev_reward + mdp.discount() * utils[succ_code] - utils[prev_code]);
        }
    }

    utils
}

struct DummyMDP<A> {
    discount: f64,
    rewards: Vec<Util>,
    outcome: HashMap<(usize, A), Vec<(usize, u32)>>,
}

impl<A> DummyMDP<A>
where
    A: Copy + Eq + Hash,
{
    fn update_reward(&mut self, state: usize, reward: Util) {
        self.rewards[state] = reward;
    }

    fn increment_outcome(&mut self, prev: usize, action: A, succ: usize) {
        let succs = self.outcome.entry((prev, action)).or_insert_with(|| vec![]);
        if let Some((_, cnt)) = succs.iter_mut().find(|(state, _)| *state == succ) {
            *cnt += 1;
        } else {
            succs.push((succ, 1));
        }
    }
}

impl<A> MDP for DummyMDP<A>
where
    A: Copy + Eq + Hash,
{
    type State = usize;
    type Action = A;

    fn states(&self) -> usize {
        self.rewards.len()
    }

    fn discount(&self) -> f64 {
        self.discount
    }

    fn encode(&self, state: &usize) -> usize {
        *state
    }

    fn decode(&self, code: usize) -> usize {
        code
    }

    fn reward(&self, state: &usize) -> Util {
        self.rewards[*state]
    }

    fn apply(&self, state: &usize, action: &A) -> Vec<(Prob, usize)> {
        let succs = self
            .outcome
            .get(&(*state, *action))
            .map(|succs| succs.as_slice())
            .unwrap_or(&[]);

        let sum: u32 = succs.iter().map(|(_, cnt)| *cnt).sum();
        succs
            .iter()
            .map(|(state, cnt)| (f64::from(*cnt) / f64::from(sum), *state))
            .collect()
    }
}

fn dummy<M>(mdp: &M) -> DummyMDP<M::Action>
where
    M: MDP,
    M::Action: Copy + Eq + Hash,
{
    DummyMDP {
        discount: mdp.discount(),
        rewards: vec![0.0; mdp.states()],
        outcome: HashMap::new(),
    }
}

pub fn adaptive_dynamic_program<M>(mdp: &M, policy: &Policy<M>, n: u32) -> Vec<Util>
where
    M: Simulate,
    M::Action: Copy + Eq + Hash,
{
    let mut visited = vec![false; mdp.states()];
    let mut dummy = dummy(mdp);
    let mut utils = vec![0.0; mdp.states()];
    let mut simulator = Simulator::new(mdp);

    for _ in 0..n {
        let prev_code = simulator.state_code();
        let prev_reward = simulator.reward();

        if !visited[prev_code] {
            visited[prev_code] = true;
            dummy.update_reward(prev_code, prev_reward);
            utils[prev_code] = prev_reward;
        }

        if simulator.trial_on_policy(policy).is_ok() {
            let succ_code = simulator.state_code();
            let succ_reward = simulator.reward();
            let action = policy[prev_code].unwrap();

            if !visited[succ_code] {
                visited[succ_code] = true;
                dummy.update_reward(succ_code, succ_reward);
                utils[succ_code] = succ_reward;
            }

            dummy.increment_outcome(prev_code, action, succ_code);
            utils = policy_evaluation(&dummy, policy, &utils, 10);
        } else {
            simulator.restart();
        }
    }

    utils
}

#[cfg(test)]
mod test {
    use crate::{learn::*, worlds::two_terminals::*};

    #[test]
    fn direct_utility_estimate_test() {
        let map = Map::default();
        let utils = value_iteration(&map, 1e-5);
        let policy = policy_from(&map, &utils);
        let estimated = direct_utility_estimate(&map, &policy, 100_000);
        // println!("{:?}", utils);
        // println!("{:?}", estimated);
        assert!(max_norm(&utils, &estimated) <= 0.01);
    }

    #[test]
    fn td_test() {
        let map = Map::default();
        let utils = value_iteration(&map, 1e-5);
        let policy = policy_from(&map, &utils);
        let estimated = temporal_difference(&map, &policy, 100_000);
        // println!("{:?}", utils);
        // println!("{:?}", estimated);
        assert!(max_norm(&utils, &estimated) <= 0.1);
    }

    #[test]
    fn adp_test() {
        let map = Map::default();
        let utils = value_iteration(&map, 1e-5);
        let policy = policy_from(&map, &utils);
        let estimated = adaptive_dynamic_program(&map, &policy, 100_000);
        // println!("{:?}", utils);
        // println!("{:?}", estimated);
        assert!(max_norm(&utils, &estimated) <= 0.1);
    }
}
