use super::*;
use indextree::{Arena, NodeId};
use rand::prelude::*;
use statrs::distribution::{ChiSquared, Univariate};
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Class {
    True,
    False,
    Plural(Attr),
}

impl Class {
    fn attr(self) -> Option<Attr> {
        match self {
            Class::Plural(attr) => Some(attr),
            _ => None,
        }
    }
}

impl From<bool> for Class {
    fn from(b: bool) -> Self {
        if b {
            Class::True
        } else {
            Class::False
        }
    }
}

struct Tag {
    value: Value,
    class: Class,
}

impl Tag {
    fn new(value: Value, class: Class) -> Self {
        Tag { value, class }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TrainOption {
    Full,
    X2Prune,
}

pub struct Trainer<'a> {
    arena: Arena<Tag>,
    input_scheme: Vec<Value>,
    examples: &'a [Example],
}

impl<'a> Trainer<'a> {
    pub fn new(input_scheme: Vec<Value>, examples: &'a [Example]) -> Self {
        assert!(
            !examples.is_empty(),
            "Trainer::new: Cannot train against empty example set"
        );

        Trainer {
            arena: Arena::new(),
            input_scheme,
            examples,
        }
    }

    fn select_examples(
        &'a self,
        examples: &'a HashSet<usize>,
    ) -> impl Iterator<Item = &'a Example> + 'a {
        examples.iter().map(move |&i| &self.examples[i])
    }

    fn output_count(&self, examples: &HashSet<usize>) -> (usize, usize) {
        class_count(self.select_examples(examples))
    }

    fn uniform_class(&self, examples: &HashSet<usize>) -> Option<Class> {
        let (p, n) = self.output_count(examples);
        if p == 0 {
            Some(Class::False)
        } else if n == 0 {
            Some(Class::True)
        } else {
            None
        }
    }

    fn plurality_value(&self, examples: &HashSet<usize>) -> Class {
        let (t, f) = self.output_count(examples);
        if t > f {
            Class::True
        } else if t < f {
            Class::False
        } else {
            random::<bool>().into()
        }
    }

    fn subsets(&self, attr: Attr, examples: &HashSet<usize>) -> Vec<(u32, u32)> {
        let mut values = vec![(0, 0); self.input_scheme[attr] as usize];
        for example in self.select_examples(examples) {
            let v = example.input[attr] as usize;
            let (p, n) = &mut values[v];
            if example.output {
                *p += 1;
            } else {
                *n += 1;
            }
        }
        values
    }

    fn remainder(&self, attr: Attr, examples: &HashSet<usize>) -> f64 {
        self.subsets(attr, examples)
            .into_iter()
            .map(|(p, n)| {
                if p + n == 0 {
                    0.0
                } else {
                    let p = f64::from(p);
                    let n = f64::from(n);
                    binary_entropy(p / (p + n)) * (p + n) / examples.len() as f64
                }
            })
            .sum()
    }

    fn new_leaf(&mut self, value: Value, class: Class) -> NodeId {
        self.arena.new_node(Tag::new(value, class))
    }

    fn train_root(&mut self) -> NodeId {
        let mut attrs = (0..self.input_scheme.len()).collect();
        let examples = (0..self.examples.len()).collect();
        self.train_recur(0 /* dummy value for root */, &examples, &mut attrs)
    }

    fn train_recur(
        &mut self,
        value: Value,
        examples: &HashSet<usize>,
        attrs: &mut HashSet<Attr>,
    ) -> NodeId {
        // the remaining examples have the same classification
        if let Some(c) = self.uniform_class(examples) {
            return self.new_leaf(value, c);
        }

        let (p, n) = class_count(self.select_examples(examples));
        let entropy = binary_entropy(p as f64 / (p + n) as f64);

        // select the attribute that minimizes the remaining entropy
        if let Some(attr) = attrs.iter().copied().max_by(|&a0, &a1| {
            let i0 = gain_ratio(entropy, self.remainder(a0, examples), self.input_scheme[a0]);
            let i1 = gain_ratio(entropy, self.remainder(a1, examples), self.input_scheme[a1]);
            i0.partial_cmp(&i1).expect("Trainer::train_recur: NaN")
        }) {
            // println!(
            //     "In examples {:?}",
            //     examples
            //         .iter()
            //         .map(|x| format!("x{}", x + 1))
            //         .collect::<Vec<_>>()
            // );
            // println!(
            //     "Attribute A{} has minimum remaining entropy {}",
            //     attr + 1,
            //     self.remainder(attr, examples)
            // );
            let node_id = self.arena.new_node(Tag::new(value, Class::Plural(attr)));
            attrs.remove(&attr);
            for value in 0..self.input_scheme[attr] {
                let filtered: HashSet<usize> = examples
                    .iter()
                    .copied()
                    .filter(|&i| self.examples[i].input[attr] == value)
                    .collect();

                let child = if filtered.is_empty() {
                    // the path of values to this leaf node filtered out all examples
                    // the only rational decision here is to return the majority of classification
                    // among examples of its parent
                    let c = self.plurality_value(examples);
                    self.new_leaf(value, c)
                } else {
                    self.train_recur(value, &filtered, attrs)
                };
                node_id.append(child, &mut self.arena);
            }
            attrs.insert(attr);
            node_id
        } else {
            // all attributes are examined but the examples are not classified yet
            // select the majority of classification among the remaining examples
            let c = self.plurality_value(examples);
            self.new_leaf(value, c)
        }
    }

    fn significance(&self, attr: Attr, examples: &HashSet<usize>) -> f64 {
        let (p, n) = class_count(self.select_examples(examples));
        let (p, n) = (p as f64, n as f64);
        self.subsets(attr, examples)
            .into_iter()
            .map(|(pk, nk)| {
                let pk = f64::from(pk);
                let nk = f64::from(nk);
                let epk = p * (pk + nk) / (p + n);
                let enk = n * (pk + nk) / (p + n);
                if epk == 0.0 || enk == 0.0 {
                    0.0
                } else {
                    (pk - epk).powi(2) / epk + (nk - enk).powi(2) / enk
                }
            })
            // .inspect(|delta| assert!(!delta.is_nan()))
            .sum()
    }

    fn prune(&mut self, node_id: NodeId, examples: &HashSet<usize>) {
        const CONFIDENCE_LEVEL: f64 = 0.95;

        let attr = if let Some(attr) = self.arena[node_id].get().class.attr() {
            attr
        } else {
            // nothing to do with leaf nodes
            return;
        };

        let children: Vec<_> = node_id.children(&self.arena).collect();
        for &child_id in children.iter() {
            // recursively prune children
            let value = self.arena[child_id].get().value;
            let filtered = examples
                .iter()
                .copied()
                .filter(|&i| self.examples[i].input[attr] == value)
                .collect();
            self.prune(child_id, &filtered);
        }

        if children
            .iter()
            .all(|&child_id| is_leaf(&self.arena, child_id))
        {
            // degree of freedom = possible values - 1
            let freedom = f64::from(self.input_scheme[attr] - 1);
            let dist =
                ChiSquared::new(freedom).expect("Trainer::prune: degree of freedom is nonpositive");
            if dist.cdf(self.significance(attr, examples)) <= CONFIDENCE_LEVEL {
                for child_id in children {
                    child_id.remove(&mut self.arena);
                }
                // after pruning, node becomes a leaf returning majority of output among examples
                let c = self.plurality_value(examples);
                self.arena[node_id].get_mut().class = c;
            }
        }
    }

    pub fn train(mut self, option: TrainOption) -> DecisionTree {
        let root = self.train_root();
        if option == TrainOption::X2Prune {
            self.prune(root, &(0..self.examples.len()).collect());
        }
        DecisionTree {
            arena: self.arena,
            root,
        }
    }
}

fn gain_ratio(entropy: f64, remainder: f64, values: Value) -> f64 {
    assert!(values > 1);
    (entropy - remainder) / f64::from(values).log2()
}

fn is_leaf<T>(arena: &Arena<T>, node_id: NodeId) -> bool {
    node_id.children(arena).count() == 0
}

pub struct DecisionTree {
    arena: Arena<Tag>,
    root: NodeId,
}

impl DecisionTree {
    fn get_class(&self, node_id: NodeId) -> Class {
        self.arena
            .get(node_id)
            .expect("DecisionTree::get_class: node id should always be valid")
            .get()
            .class
    }

    fn select_child(&self, node_id: NodeId, value: Value) -> NodeId {
        node_id.children(&self.arena)
            .find(|&child_id| self.arena.get(child_id).map(|node| node.get().value) == Some(value))
            .expect("DecisionTree::select_child: Non-leaf node should have child for each possible value")
    }

    pub fn classify(&self, input: &[Value]) -> bool {
        let mut node_id = self.root;
        loop {
            match self.get_class(node_id) {
                Class::True => return true,
                Class::False => return false,
                Class::Plural(attr) => {
                    node_id = self.select_child(node_id, input[attr]);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    #[test]
    fn consistency_test() {
        let examples = parse_examples(DATA);
        let trainer = Trainer::new(INPUT_SCHEME.to_vec(), &examples);
        let tree = trainer.train(TrainOption::Full);

        // as long as the training set is free of noise and error
        // trained decision tree should be consistent to the training set
        for example in examples {
            assert_eq!(tree.classify(&example.input), example.output);
        }
    }

    #[test]
    fn pruning_test() {
        let examples = parse_examples(DATA);
        let trainer = Trainer::new(INPUT_SCHEME.to_vec(), &examples);
        let tree = trainer.train(TrainOption::X2Prune);
        let error = examples
            .iter()
            .filter(|example| tree.classify(&example.input) != example.output)
            .count();

        assert!(error <= 2);
    }
}
