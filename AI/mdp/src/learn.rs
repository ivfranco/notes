use super::*;
use rand::prelude::*;

pub trait Simulate: SoloMDP {
    fn start(&self) -> Self::State;
}

pub struct Simulator<'a, M, S> {
    world: &'a M,
    state: S,
}

impl<'a, M> Simulator<'a, M, M::State>
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

    pub fn trial(&mut self, action: &M::Action) -> Option<(&M::State, Util)> {
        let mut rng = thread_rng();
        let mut successors = self.world.apply(&self.state, action);
        let indices: Vec<_> = (0..successors.len()).collect();
        let succ = indices
            .choose_weighted(&mut rng, |&i| successors[i].0)
            .map(|&i| successors.swap_remove(i).1)
            .ok()?;

        self.state = succ;
        Some((self.state(), self.world.reward(self.state())))
    }
}

struct Trials<'a, M, S> 
where
    M: Simulate,
{
    simulator: &'a mut Simulator<'a, M, S>,
    policy: &'a Policy<M>,
}

impl<'a, M, S> Trials<'a, M, S> 
where M: Simulate,
{
    fn into_inner(self) -> &'a mut Simulator<'a, M, S> {
        self.simulator
    }
}

impl<'a, M> Iterator for Trials<'a, M, M::State>
where
    M: Simulate,
    M::State: Clone,
{
    type Item = (M::State, Util);

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.policy[self.simulator.state_code()].as_ref()?;
        let (state, reward) = self.simulator.trial(action)?;
        Some((state.clone(), reward))
    }
}

fn trials<'a, M>(simulator: &'a mut Simulator<'a, M, M::State>, policy: &'a Policy<M>) -> Trials<'a, M, M::State> 
where
    M: Simulate,
{
    Trials { simulator, policy }
}

pub fn direct_utility_estimate<M>(mdp: &M, policy: &Policy<M>)
where
    M: Simulate,
{
    let mut simulator = Simulator::new(mdp);
    let mut utils = vec![0.0; mdp.states()];
    let mut utils_cnt = vec![0; mdp.states()];

    // when should it stop?
    unimplemented!()
}