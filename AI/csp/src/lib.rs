pub mod colors;
pub mod sudoku;

use petgraph::prelude::*;

use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub trait Constraint<V> {
    fn relation(&self, x: &V, y: &V) -> bool;

    fn filter_forward(&self, x_set: &Var<V>, y_set: &Var<V>) -> Var<V>
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

    fn filter_backward(&self, x_set: &HashSet<V>, y_set: &HashSet<V>) -> HashSet<V>
    where
        V: Clone + Eq + Hash,
    {
        let mut filtered = HashSet::new();
        for x in x_set {
            for y in y_set {
                if self.relation(x, y) {
                    filtered.insert(x.clone());
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

fn two_way_edges<'a, N, E>(
    node: NodeIndex,
    graph: &'a DiGraph<N, E>,
) -> impl Iterator<Item = (EdgeIndex, Direction)> + 'a {
    graph
        .edges_directed(node, Outgoing)
        .map(|e| (e.id(), Outgoing))
        .chain(
            graph
                .edges_directed(node, Incoming)
                .map(|e| (e.id(), Incoming)),
        )
}

fn ac3<V, C>(
    edges: impl IntoIterator<Item = (EdgeIndex, Direction)>,
    variables: &mut [Var<V>],
    csp: &Csp<C>,
) -> Result<(), Inconsistency>
where
    V: Clone + Eq + Hash,
    C: Constraint<V>,
{
    let mut queue: VecDeque<(EdgeIndex, Direction)> = edges.into_iter().collect();

    while let Some((e, d)) = queue.pop_front() {
        let (from, to) = csp.edge_endpoints(e).unwrap();
        let (from_var, to_var) = (&variables[from.index()], &variables[to.index()]);
        let constraint = csp.edge_weight(e).unwrap();

        let (target, filtered) = if d == Outgoing {
            let filtered = constraint.filter_forward(from_var, to_var);
            (to, filtered)
        } else {
            let filtered = constraint.filter_backward(from_var, to_var);
            (from, filtered)
        };

        if filtered.is_empty() {
            return Err(Inconsistency);
        }

        let shrinked = filtered.len() < variables[target.index()].len();

        variables[target.index()] = filtered;
        if shrinked {
            queue.extend(two_way_edges(target, csp));
        }
    }

    Ok(())
}

pub fn ac3_total<V, C>(
    variables: &mut [Var<V>],
    csp: &Csp<C>,
) -> Result<(), Inconsistency>
where
    V: Clone + Eq + Hash,
    C: Constraint<V>,
{
    let edges = csp.edge_indices().map(|e| (e, Outgoing)).chain(csp.edge_indices().map(|e| (e, Incoming)));
    ac3(edges, variables, csp)
}

pub fn backtracking_search<V, C>(
    mut variables: Vec<Var<V>>,
    csp: &Csp<C>,
) -> Result<Vec<V>, Inconsistency>
where
    V: Clone + Eq + Hash,
    C: Constraint<V>,
{
    ac3_total(&mut variables, csp)?;
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

fn backtrack<V, C>(mut variables: Vec<Var<V>>, csp: &Csp<C>) -> Result<Vec<V>, Inconsistency>
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

        if ac3(two_way_edges(NodeIndex::new(idx), csp), &mut updated, csp).is_err() {
            continue;
        }

        if let Ok(assignment) = backtrack(updated, csp) {
            return Ok(assignment);
        }
    }

    Err(Inconsistency)
}
