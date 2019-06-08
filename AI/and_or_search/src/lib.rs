use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait State: Sized {
    type Action;

    fn actions(&self) -> Vec<Self::Action>;
    fn results(&self, action: &Self::Action) -> Vec<Self>;
    fn successful(&self) -> bool;
}

pub enum Plan<S, A> {
    Goal,
    Or(A, Box<Plan<S, A>>),
    And(HashMap<S, Plan<S, A>>),
}

impl<S, A> Plan<S, A> {
    fn goal() -> Plan<S, A> {
        Plan::Goal
    }

    fn or(self, action: A) -> Self {
        Plan::Or(action, Box::new(self))
    }

    fn and(branches: HashMap<S, Plan<S, A>>) -> Self {
        Plan::And(branches)
    }
}

pub enum Error {
    Failure,
}

use Error::*;

fn or_search<S>(state: &S, path: &HashSet<S>) -> Result<Plan<S, S::Action>, Error>
where
    S: State + Clone + Eq + Hash,
    S::Action: Copy,
{
    if state.successful() {
        return Ok(Plan::goal());
    }

    if path.contains(state) {
        return Err(Failure);
    }

    let mut extended = path.clone();
    extended.insert(state.clone());

    for action in state.actions() {
        if let Ok(plan) = and_search(state, action, &extended) {
            return Ok(plan.or(action));
        }
    }

    Err(Failure)
}

fn and_search<S>(
    state: &S,
    action: S::Action,
    path: &HashSet<S>,
) -> Result<Plan<S, S::Action>, Error>
where
    S: State + Clone + Eq + Hash,
    S::Action: Copy,
{
    let mut branches = HashMap::new();

    for s in state.results(&action) {
        if let Ok(plan) = or_search(&s, path) {
            branches.insert(s, plan);
        } else {
            return Err(Failure);
        }
    }

    Ok(Plan::and(branches))
}

pub fn and_or_search<S>(start: &S) -> Result<Plan<S, S::Action>, Error>
where
    S: State + Clone + Eq + Hash,
    S::Action: Copy,
{
    or_search(start, &HashSet::new())
}