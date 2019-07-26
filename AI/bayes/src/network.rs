use petgraph::{
    algo::toposort,
    prelude::*,
    visit::{IntoNodeReferences, Topo, Walker},
};

pub use cpt::*;
use rand::prelude::*;

use std::{borrow::Borrow, collections::HashMap};

pub type Prob = f64;
pub type Value = usize;
pub type Evidence = HashMap<NodeIndex, Value>;
pub type Event = Evidence;

pub mod cpt {
    use super::*;
    use std::borrow::Borrow;

    pub const T: Value = 1;
    pub const F: Value = 0;

    #[derive(Clone)]
    /// a fully specified CPT table
    pub struct Full {
        /// mapping NodeIndex of parents to indices in conditioned probability
        causes: HashMap<NodeIndex, usize>,
        /// P(X | Parents(X)) for each combination of parents
        probs: HashMap<Vec<Value>, Vec<Prob>>,
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

        fn causes(&self) -> Vec<NodeIndex> {
            self.causes.keys().cloned().collect()
        }

        fn to_key<I, P>(&self, iter: I) -> Vec<Value>
        where
            I: IntoIterator<Item = P>,
            P: Borrow<(NodeIndex, Value)>,
        {
            let mut key = vec![0; self.causes.len()];

            for p in iter.into_iter() {
                let (n, v) = p.borrow();
                if let Some(p) = self.causes.get(n) {
                    key[*p] = *v;
                }
            }

            key
        }

        fn query_row(&self, evidence: &Evidence) -> &[Prob] {
            assert!(
                self.causes.keys().all(|n| evidence.contains_key(n)),
                "Full::query_row: unspecified parent"
            );

            let key = self.to_key(evidence.iter().map(|(k, v)| (*k, *v)));
            self.probs
                .get(&key)
                .expect("Full::query_row: Full CPT must be complete")
        }

        /// insert P(X | parents) to the CPT
        pub fn insert_row(&mut self, parents: &[(NodeIndex, Value)], probs: &[Prob]) {
            let key = self.to_key(parents);
            self.probs.insert(key, probs.to_vec());
        }

        /// p_one would be the p(X = 1 | parents), for whatever 1 stands for in the context
        pub fn insert_binary_row(&mut self, parents: &[(NodeIndex, Value)], p_one: Prob) {
            self.insert_row(parents, &[1.0 - p_one, p_one])
        }

        /// insert cpt rows in increasing binary number order
        ///
        /// i.e. for a binary node with 3 binary parents, insert probabilities in order:
        ///
        /// P(X = True | Parents(X) = [F, F, F]) = probs[0]
        ///
        /// P(X = True | Parents(X) = [F, F, T]) = probs[1]
        ///
        /// P(X = True | Parents(X) = [F, T, F]) = probs[2]
        ///
        /// P(X = True | Parents(X) = [F, T, T]) = probs[3]
        ///
        /// ...
        ///
        /// P(X = True | Parents(X) = [T, T, T]) = probs[7]
        pub fn insert_in_binary_order(&mut self, probs: &[Prob]) {
            assert_eq!(probs.len(), 2usize.pow(self.causes.len() as u32));

            let mut key = vec![F; self.causes.len()];
            for &p in probs {
                self.probs.insert(key.clone(), vec![1.0 - p, p]);
                increment(&mut key);
            }
        }

        fn query(&self, value: Value, evidence: &Evidence) -> Prob {
            *self
                .query_row(evidence)
                .get(value)
                .expect("Full::query: Categorical value out of bound")
        }

        fn random_sample(&self, evidence: &Evidence) -> Value {
            let row = self.query_row(evidence);
            random_idx_sample(row)
        }
    }

    fn increment(key: &mut [Value]) {
        let mut carry = true;
        for bit in key.iter_mut().rev() {
            if carry {
                if *bit == 0 {
                    *bit = 1;
                    carry = false;
                } else {
                    *bit = 0;
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct NoisyOr {
        causes: HashMap<NodeIndex, Prob>,
    }

    impl NoisyOr {
        fn new(parents: &[(NodeIndex, Prob)]) -> Self {
            let causes = parents.iter().cloned().collect();
            NoisyOr { causes }
        }

        fn causes(&self) -> Vec<NodeIndex> {
            self.causes.keys().cloned().collect()
        }

        fn query(&self, value: Value, evidence: &Evidence) -> Prob {
            let p_false = evidence
                .iter()
                .filter_map(|(n, v)| if *v == T { self.causes.get(n) } else { None })
                .product();

            if value == T {
                1.0 - p_false
            } else {
                p_false
            }
        }

        fn random_sample(&self, evidence: &Evidence) -> Value {
            let p_true = self.query(T, evidence);
            if random::<f64>() <= p_true {
                T
            } else {
                F
            }
        }
    }

    #[derive(Clone)]
    pub enum CPT {
        Const(Vec<Prob>),
        Full(Full),
        NoisyOr(NoisyOr),
    }

    use CPT::*;

    impl CPT {
        pub(super) fn new_const(probs: Vec<Prob>) -> Self {
            Const(probs)
        }

        pub(super) fn new_noisy_or(parents: &[(NodeIndex, Prob)]) -> Self {
            NoisyOr(NoisyOr::new(parents))
        }

        pub(super) fn query(&self, value: Value, evidence: &Evidence) -> Prob {
            match self {
                Full(full) => full.query(value, evidence),
                Const(probs) => *probs
                    .get(value)
                    .expect("CPT::get_prob: Categorical value out of bound"),
                NoisyOr(or) => or.query(value, evidence),
            }
        }

        pub(super) fn random_sample(&self, evidence: &Evidence) -> Value {
            match self {
                Const(probs) => random_idx_sample(probs),
                Full(full) => full.random_sample(evidence),
                NoisyOr(or) => or.random_sample(evidence),
            }
        }

        pub(super) fn causes(&self) -> Vec<NodeIndex> {
            match self {
                Const(..) => vec![],
                Full(full) => full.causes(),
                NoisyOr(or) => or.causes(),
            }
        }
    }

    impl From<Full> for CPT {
        fn from(full: Full) -> Self {
            CPT::Full(full)
        }
    }


    #[test]
    fn noisy_or_test() {
        let (cold, flu, malaria) = (NodeIndex::from(0), NodeIndex::from(1), NodeIndex::from(2));
        let fever = CPT::new_noisy_or(&[(cold, 0.6), (flu, 0.2), (malaria, 0.1)]);
        const E: Prob = 0.0001;

        let evidence = evidence_from([(cold, F), (flu, F), (malaria, F)].iter());
        assert!((fever.query(T, &evidence) - 0.0).abs() <= E);
        let evidence = evidence_from([(cold, F), (flu, F), (malaria, T)].iter());
        assert!((fever.query(T, &evidence) - 0.9).abs() <= E);
        let evidence = evidence_from([(cold, T), (flu, F), (malaria, T)].iter());
        assert!((fever.query(T, &evidence) - 0.94).abs() <= E);
        let evidence = evidence_from([(cold, T), (flu, T), (malaria, T)].iter());
        assert!((fever.query(T, &evidence) - 0.988).abs() <= E);
    }
}

/// a discrete random variable with constant probability distribution or fully specified CPT
pub struct Variable {
    cpt: CPT,
    values: usize,
}

impl Variable {
    pub fn new(cpt: CPT, values: usize) -> Self {
        Variable { cpt, values }
    }

    pub fn new_const(probs: Vec<Prob>) -> Self {
        Variable {
            values: probs.len(),
            cpt: CPT::new_const(probs),
        }
    }

    pub fn new_noisy_or(parents: &[(NodeIndex, Prob)]) -> Self {
        Variable {
            values: 2,
            cpt: CPT::new_noisy_or(parents),
        }
    }

    pub fn new_binary_const(p_true: Prob) -> Self {
        Variable::new_const(vec![1.0 - p_true, p_true])
    }

    pub fn binary_single_parent(parent: NodeIndex, p_true: Prob, p_false: Prob) -> Self {
        let mut cpt = Full::new(&[parent]);
        cpt.insert_row(&[(parent, 0)], &[1.0 - p_false, p_false]);
        cpt.insert_row(&[(parent, 1)], &[1.0 - p_true, p_true]);
        Variable::new(CPT::Full(cpt), 2)
    }

    pub fn values(&self) -> usize {
        self.values
    }

    pub fn causes(&self) -> Vec<NodeIndex> {
        self.cpt.causes()
    }

    pub fn query(&self, value: Value, evidence: &Evidence) -> Prob {
        assert!(
            value < self.values,
            "Variable::query: Categorical value out of bound"
        );
        self.cpt.query(value, evidence)
    }

    pub fn random_sample(&self, evidence: &Evidence) -> Value {
        self.cpt.random_sample(evidence)
    }
}

/// Bayesian network with discrete variables
#[derive(Default)]
pub struct Network {
    graph: Graph<Variable, (), Directed>,
}

impl Network {
    pub fn new() -> Self {
        Network::default()
    }

    pub fn add_node(&mut self, var: Variable) -> NodeIndex {
        let causes = var.causes();
        let node = self.graph.add_node(var);
        for parent in causes {
            self.graph.add_edge(parent, node, ());
        }
        node
    }

    // pub fn add_edge(&mut self, parent: NodeIndex, child: NodeIndex) {
    //     self.graph.add_edge(parent, child, ());
    // }

    fn get(&self, x: NodeIndex) -> &Variable {
        self.graph
            .node_weight(x)
            .expect("Network::query: NodeIndex out of bound")
    }

    fn query_cpt(&self, var: NodeIndex, value: Value, evidence: &Evidence) -> Prob {
        self.graph
            .node_weight(var)
            .expect("Network::get_prob: NodeIndex out of bound")
            .query(value, evidence)
    }

    /// calculates P(X | evidence) from the Bayesian network
    pub fn query(&self, x: NodeIndex, evidence: &Evidence) -> Vec<Prob> {
        let topo = toposort(&self.graph, None)
            .expect("Network::query: Bayesian network should be acyclic");
        let var = self.get(x);

        if let Some(v) = evidence.get(&x) {
            let mut dist = vec![0.0; var.values()];
            dist[*v] = 1.0;
            dist
        } else {
            let mut dist = Vec::with_capacity(var.values());
            let mut event = evidence.clone();

            for v in 0..var.values() {
                event.insert(x, v);
                dist.push(self.enumerate_all(&topo, &mut event));
                event.remove(&x);
            }

            normalize(dist)
        }
    }

    fn enumerate_all(&self, vars: &[NodeIndex], evidence: &mut Evidence) -> Prob {
        let x = if let Some(var) = vars.first() {
            *var
        } else {
            return 1.0;
        };

        let rest = &vars[1..];

        match evidence.get(&x) {
            Some(v) => self.query_cpt(x, *v, evidence) * self.enumerate_all(rest, evidence),
            None => {
                let var = self.get(x);
                let mut sum = 0.0;
                for v in 0..var.values() {
                    let p = self.query_cpt(x, v, evidence);
                    evidence.insert(x, v);
                    sum += p * self.enumerate_all(rest, evidence);
                    evidence.remove(&x);
                }
                sum
            }
        }
    }

    /// returns an estimate of P(X = value | evidence) calculated from n samples
    pub fn likelihood_weighting(
        &self,
        x: NodeIndex,
        value: Value,
        evidence: &Evidence,
        n: u32,
    ) -> Prob {
        let var = self.get(x);
        let mut dist = vec![0.0; var.values()];

        for _ in 0..n {
            let (event, w) = self.weighted_sample(evidence);
            let &v = event
                .get(&x)
                .expect("Network::likelihood_weighting: Event must be complete");
            dist[v] += w;
        }

        normalize(dist)[value]
    }

    fn weighted_sample(&self, evidence: &Evidence) -> (Event, Prob) {
        let mut w = 1.0;
        let mut event = evidence.clone();

        for x in Topo::new(&self.graph).iter(&self.graph) {
            match event.get(&x) {
                Some(v) => w *= self.query_cpt(x, *v, &event),
                None => {
                    let var = self.get(x);
                    event.insert(x, var.random_sample(&event));
                }
            }
        }

        (event, w)
    }

    pub fn gibbs_sampling(
        &self,
        x: NodeIndex,
        value: Value,
        evidence: &Evidence,
        samples: u32,
    ) -> Prob {
        let mut rng = thread_rng();

        let mut event = evidence.clone();
        for (n, var) in self.graph.node_references() {
            event
                .entry(n)
                .or_insert_with(|| (0..var.values()).choose(&mut rng).unwrap());
        }

        let mut non_evidence: Vec<_> = self
            .graph
            .node_indices()
            .filter(|n| !evidence.contains_key(&n))
            .collect();
        non_evidence.shuffle(&mut rng);
        let mut non_evidence_iter = non_evidence.into_iter().cycle();

        let mut hits = 0;
        for _ in 0 .. samples {
            let node = non_evidence_iter.next().unwrap();
            event.remove(&node);
            let dist = self.query(node, &event);
            let v = random_idx_sample(&dist);
            event.insert(node, v);


            if event.get(&x) == Some(&value) {
                hits += 1;
            }
        }

        f64::from(hits) / f64::from(samples)
    }
}

fn normalize(mut probs: Vec<Prob>) -> Vec<Prob> {
    let sum: Prob = probs.iter().sum();
    for p in probs.iter_mut() {
        *p /= sum;
    }
    probs
}

pub fn evidence_from<I, P>(iter: I) -> Evidence
where
    I: IntoIterator<Item = P>,
    P: Borrow<(NodeIndex, Value)>,
{
    iter.into_iter().map(|p| *p.borrow()).collect()
}

fn random_idx_sample(probs: &[Prob]) -> Value {
    let mut rng = thread_rng();
    let indices: Vec<_> = (0..probs.len()).collect();
    *indices
        .choose_weighted(&mut rng, |&i| probs[i])
        .expect("random_idx_sample: probability table rows should be nonempty")
}

#[cfg(test)]
mod test {
    use crate::examples::burglary::*;

    #[test]
    fn burglary_test() {
        let (network, nodes) = burglary_network();
        let [burglary, _, _, john_calls, mary_calls] = nodes;

        let evidence = [(john_calls, T), (mary_calls, T)].iter().cloned().collect();

        let dist = network.query(burglary, &evidence);
        assert!((0.284 - dist[T]).abs() <= 0.001);

        let evidence = [(burglary, T)].iter().cloned().collect();
        assert_eq!(network.query(burglary, &evidence), &[0.0, 1.0]);
    }

    #[test]
    fn sampling_test() {
        let (network, nodes) = burglary_network();
        let [burglary, _, _, john_calls, mary_calls] = nodes;

        let evidence = [(john_calls, T), (mary_calls, T)].iter().cloned().collect();

        let estimate = network.likelihood_weighting(burglary, T, &evidence, 100_000);
        assert!((estimate - 0.284).abs() <= 0.05, "{}", estimate);

        let estimate = network.gibbs_sampling(burglary, T, &evidence, 10000);
        assert!((estimate - 0.284).abs() <= 0.02, "{}", estimate);
    }
}
