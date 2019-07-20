use petgraph::{
    prelude::*,
    algo::toposort,
};

use std::collections::HashMap;
use cpt::*;

pub type Prob = f64;
pub type Evidence = HashMap<NodeIndex, bool>;

pub mod cpt {
    use super::*;
    use fixedbitset::FixedBitSet;

    pub struct Full {
        /// mapping NodeIndex of
        causes: HashMap<NodeIndex, usize>,
        /// probability of p(X = true) given a set of evidences
        probs: HashMap<FixedBitSet, Prob>,
    }

    impl Full {
        pub fn new(parents: &[NodeIndex]) -> Self {
            let mut sorted_causes = parents.to_vec();
            sorted_causes.sort();
            sorted_causes.dedup();

            let causes = sorted_causes
                .into_iter()
                .enumerate()
                .map(|(i, c)| (c, i))
                .collect();

            Full {
                causes,
                probs: HashMap::new(),
            }
        }

        pub fn single_parent(parent: NodeIndex, p_true: Prob, p_false: Prob) -> Self {
            let mut full = Full::new(&[parent]);
            full.insert_prob(&[parent], p_true);
            full.insert_prob(&[], p_false);
            full
        }

        fn truth_set<'a, I>(&self, truths: I) -> FixedBitSet 
        where
            I: IntoIterator<Item = &'a NodeIndex>,
        {
            truths
                .into_iter()
                .filter_map(|n| self.causes.get(n))
                .cloned()
                .collect()
        }

        pub fn insert_prob(&mut self, truths: &[NodeIndex], prob: Prob) {
            let set = self.truth_set(truths);
            self.probs.insert(set, prob);
        }

        fn get_prob(&self, evidence: &Evidence) -> Prob {
            let set = self.truth_set(evidence.iter().filter(|(_, t)| **t).map(|(n, _)| n));
            *self
                .probs
                .get(&set)
                .expect("Full::get_prob: conditional probability table should be complete")
        }
    }

    pub enum CPT {
        Const(Prob),
        Full(Full),
    }

    use CPT::*;

    impl CPT {
        pub fn new_const(prob: Prob) -> Self {
            Const(prob)
        }

        pub fn new_full(parents: &[NodeIndex]) -> Self {
            Full(Full::new(parents))
        }

        pub fn get_prob(&self, evidence: &Evidence) -> Prob {
            match self {
                Full(full) => full.get_prob(evidence),
                Const(prob) => *prob,
            }
        }
    }
}

/// binary-valued Bayesian network
#[derive(Default)]
pub struct Network {
    graph: Graph<CPT, (), Directed>,
}

impl Network {
    pub fn new() -> Self {
        Network::default()
    }

    pub fn add_node(&mut self, cpt: CPT) -> NodeIndex {
        self.graph.add_node(cpt)
    }

    pub fn add_edge(&mut self, parent: NodeIndex, child: NodeIndex) {
        self.graph.add_edge(parent, child, ());
    }

    fn get_prob(&self, var: NodeIndex, evidence: &Evidence) -> Prob {
        self.graph.node_weight(var).expect("Network::get_prob: NodeIndex out of bound")
            .get_prob(evidence)
    }

    /// return P(x = true | evidence)
    pub fn query(&self, x: NodeIndex, mut evidence: Evidence) -> Prob {
        let vars = toposort(&self.graph, None).expect("Network::query: Bayesian network should be acyclic");
        evidence.insert(x, true);
        let p_true = self.enumerate_all(&vars, &mut evidence);
        evidence.insert(x, false);
        let p_false = self.enumerate_all(&vars, &mut evidence);

        normalize(p_true, p_false)
    }

    fn enumerate_all(&self, vars: &[NodeIndex], evidence: &mut Evidence) -> Prob {
        let var = if let Some(var) = vars.first() {
            *var
        } else {
            return 1.0;
        };

        let rest = &vars[1..];
        let p_true = self.get_prob(var, evidence);
        let p_false = 1.0 - p_true;

        match evidence.get(&var) {
            Some(true) => p_true * self.enumerate_all(rest, evidence),
            Some(false) => p_false * self.enumerate_all(rest, evidence),
            None => {
                evidence.insert(var, true);
                let mut p = p_true * self.enumerate_all(rest, evidence);
                evidence.insert(var, false);
                p += p_false * self.enumerate_all(rest, evidence);
                evidence.remove(&var);
                p
            }
        }
    }
}

fn normalize(p_true: Prob, p_false: Prob) -> Prob {
    p_true / (p_true + p_false)
}

#[test]
fn burglary_test() {
    let mut network = Network::new();
    let burglary = network.add_node(CPT::new_const(0.001));
    let earthquake = network.add_node(CPT::new_const(0.002));

    let mut alarm_cpt = Full::new(&[burglary, earthquake]);
    alarm_cpt.insert_prob(&[burglary, earthquake], 0.95);
    alarm_cpt.insert_prob(&[burglary], 0.94);
    alarm_cpt.insert_prob(&[earthquake], 0.29);
    alarm_cpt.insert_prob(&[], 0.001);
    let alarm = network.add_node(CPT::Full(alarm_cpt));

    let john_calls_cpt = Full::single_parent(alarm, 0.9, 0.05);
    let john_calls = network.add_node(CPT::Full(john_calls_cpt));
    let mary_calls_cpt = Full::single_parent(alarm, 0.7, 0.01);
    let mary_calls = network.add_node(CPT::Full(mary_calls_cpt));

    network.add_edge(burglary, alarm);
    network.add_edge(earthquake, alarm);
    network.add_edge(alarm, john_calls);
    network.add_edge(alarm, mary_calls);

    let evidence = [(john_calls, true), (mary_calls, true)].iter()
        .cloned()
        .collect();

    assert!((0.284 - network.query(burglary, evidence)).abs() <= 0.001);
}