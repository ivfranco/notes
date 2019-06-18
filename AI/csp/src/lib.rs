pub mod colors;

use petgraph::prelude::*;

use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub trait Constraint<V> {
    fn relation(&self, x: &V, y: &V) -> bool;

    fn filter_forward(&self, x_set: &HashSet<V>, y_set: &HashSet<V>) -> HashSet<V>
    where
        V: Clone + Eq + Hash,
    {
        let mut filtered = HashSet::new();
        for x in x_set {
            for y in y_set {
                if self.relation(x, y) {
                    filtered.insert(y.clone());
                }
            }
        }
        filtered
    }
}

pub struct Diff;

impl<V> Constraint<V> for Diff
where
    V: PartialEq,
{
    fn relation(&self, x: &V, y: &V) -> bool {
        x != y
    }
}

#[derive(Debug)]
pub struct Inconsistency;

pub type Var<V> = HashSet<V>;
pub type Csp<C> = Graph<(), C, Directed>;

fn ac3<V, C>(
    edges: impl IntoIterator<Item = EdgeIndex>,
    variables: &mut [Var<V>],
    csp: &Csp<C>,
) -> Result<(), Inconsistency>
where
    V: Clone + Eq + Hash,
    C: Constraint<V>,
{
    let mut queue: VecDeque<EdgeIndex> = edges.into_iter().collect();

    while let Some(e) = queue.pop_front() {
        let (from, to) = csp.edge_endpoints(e).unwrap();
        let from_var = &variables[from.index()];
        let to_var = &variables[to.index()];
        let constraint = csp.edge_weight(e).unwrap();

        let orig_size = to_var.len();
        let filtered = constraint.filter_forward(from_var, to_var);
        let shrinked = filtered.len() < orig_size;

        if filtered.is_empty() {
            return Err(Inconsistency);
        }

        variables[to.index()] = filtered;

        if shrinked {
            queue.extend(csp.edges(to).map(|e| e.id()));
        }
    }

    Ok(())
}

pub fn backtracking_search<V, C>(
    mut variables: Vec<Var<V>>,
    csp: &mut Csp<C>,
) -> Result<Vec<V>, Inconsistency>
where
    V: Clone + Eq + Hash,
    C: Constraint<V>,
{
    let edges: Vec<_> = csp.edge_indices().collect();
    ac3(edges, &mut variables, csp)?;

    backtrack(variables, csp)
}

fn complete<V>(variables: &[Var<V>]) -> bool {
    variables.iter().all(|v| v.len() == 1)
}

fn select_unassigned<V>(variables: &[Var<V>]) -> usize {
    variables
        .iter()
        .enumerate()
        .filter(|(_, v)| v.len() > 1)
        .min_by_key(|(_, v)| v.len())
        .unwrap()
        .0
}

fn order_domain_values<V>(idx: usize, variables: &[Var<V>]) -> Vec<V>
where
    V: Clone,
{
    variables[idx].iter().cloned().collect()
}

fn backtrack<V, C>(mut variables: Vec<Var<V>>, csp: &mut Csp<C>) -> Result<Vec<V>, Inconsistency>
where
    V: Clone + Eq + Hash,
    C: Constraint<V>,
{
    if complete(&variables) {
        return Ok(variables
            .iter_mut()
            .map(|v| v.drain().next().unwrap())
            .collect());
    }

    let idx = select_unassigned(&variables);

    for value in order_domain_values(idx, &variables) {
        let mut updated = variables.to_vec();
        updated[idx] = Some(value).into_iter().collect();

        let edges = csp.edges(NodeIndex::new(idx)).map(|e| e.id());
        ac3(edges, &mut updated, csp)?;

        if let Ok(assignment) = backtrack(updated, csp) {
            return Ok(assignment);
        }
    }

    Err(Inconsistency)
}
