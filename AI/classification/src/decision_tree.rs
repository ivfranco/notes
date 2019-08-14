use super::*;
use indextree::{Arena, NodeId};
use rand::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Class {
    True,
    False,
    Plural(Attr),
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

pub struct Trainer<'a> {
    arena: Arena<Tag>,
    input_scheme: Vec<Value>,
    examples: &'a [Example],
    leaves: Vec<NodeId>,
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
            leaves: vec![],
        }
    }

    fn select_examples(
        &'a self,
        examples: &'a HashSet<usize>,
    ) -> impl Iterator<Item = &'a Example> + 'a {
        examples.iter().map(move |&i| &self.examples[i])
    }

    fn output_count(&self, examples: &HashSet<usize>) -> (usize, usize) {
        let (mut t, mut f) = (0, 0);
        for example in self.select_examples(examples) {
            if example.output {
                t += 1;
            } else {
                f += 1;
            }
        }
        (t, f)
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

    fn remainder(&self, attr: Attr, examples: &HashSet<usize>) -> f64 {
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
            .into_iter()
            .map(|(p, n)| {
                if p + n == 0 {
                    0.0
                } else {
                    let p = f64::from(p);
                    let n = f64::from(n);
                    println!("{}, {}", p, n);
                    binary_entropy(p / (p + n)) * (p + n) / examples.len() as f64
                }
            })
            .inspect(|e| println!("{}", e))
            .sum()
    }

    fn new_leaf(&mut self, value: Value, class: Class) -> NodeId {
        let node_id = self.arena.new_node(Tag::new(value, class));
        self.leaves.push(node_id);
        node_id
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

        // select the attribute that minimizes the remaining entropy
        if let Some(attr) = attrs.iter().cloned().min_by(|&a0, &a1| {
            let i0 = self.remainder(a0, examples);
            let i1 = self.remainder(a1, examples);
            i0.partial_cmp(&i1).expect("Trainer::train_recur: NaN")
        }) {
            let node_id = self.arena.new_node(Tag::new(value, Class::Plural(attr)));
            attrs.remove(&attr);
            for value in 0..self.input_scheme[attr] {
                let filtered: HashSet<usize> = examples
                    .iter()
                    .cloned()
                    .filter(|&i| self.examples[i].input[attr] == value)
                    .collect();

                let child = if filtered.is_empty() {
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
            // select the majority of classification in the remaining examples
            let c = self.plurality_value(examples);
            self.new_leaf(value, c)
        }
    }

    pub fn train(mut self) -> DecisionTree {
        let root = self.train_root();
        DecisionTree {
            arena: self.arena,
            root,
        }
    }
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
    use regex::Regex;

    const DATA: &str = "\
x1 Yes No No Yes Some $$$ No Yes French 0–10 y1 = Yes
x2 Yes No No Yes Full $ No No Thai 30–60 y2 = No
x3 No Yes No No Some $ No No Burger 0–10 y3 = Yes
x4 Yes No Yes Yes Full $ Yes No Thai 10–30 y4 = Yes
x5 Yes No Yes No Full $$$ No Yes French >60 y5 = No
x6 No Yes No Yes Some $$ Yes Yes Italian 0–10 y6 = Yes
x7 No Yes No No None $ Yes No Burger 0–10 y7 = No
x8 No No No Yes Some $$ Yes Yes Thai 0–10 y8 = Yes
x9 No Yes Yes No Full $ Yes No Burger >60 y9 = No
x10 Yes Yes Yes Yes Full $$$ No Yes Italian 10–30 y10 = No
x11 No No No No None $ No No Thai 0–10 y11 = No
x12 Yes Yes Yes Yes Full $ No No Burger 30–60 y12 = Yes";

    fn yes_no(s: &str) -> Value {
        match s {
            "Yes" => 1,
            "No" => 0,
            _ => unreachable!(),
        }
    }

    fn parse_examples(data: &str) -> Vec<Example> {
        let regex = Regex::new(
            r"(?x)
            x\d+\s
            (?P<alt>Yes|No)\s
            (?P<bar>Yes|No)\s
            (?P<fri>Yes|No)\s
            (?P<hun>Yes|No)\s
            (?P<pat>None|Some|Full)\s
            (?P<price>\$|\$\$|\$\$\$)\s
            (?P<rain>Yes|No)\s
            (?P<res>Yes|No)\s
            (?P<type>French|Thai|Burger|Italian)\s
            (?P<est>0–10|10–30|30–60|>60)\s
            y\d+\s=\s
            (?P<willwait>Yes|No)
        ",
        )
        .unwrap();

        data.lines()
            .map(|line| {
                let cap = regex.captures(line).unwrap();
                let mut input = vec![];
                input.push(yes_no(&cap["alt"]));
                input.push(yes_no(&cap["bar"]));
                input.push(yes_no(&cap["fri"]));
                input.push(yes_no(&cap["hun"]));
                input.push(match &cap["pat"] {
                    "None" => 0,
                    "Some" => 1,
                    "Full" => 2,
                    _ => unreachable!(),
                });
                input.push(match &cap["price"] {
                    "$" => 0,
                    "$$" => 1,
                    "$$$" => 2,
                    _ => unreachable!(),
                });
                input.push(yes_no(&cap["rain"]));
                input.push(yes_no(&cap["res"]));
                input.push(match &cap["type"] {
                    "French" => 0,
                    "Thai" => 1,
                    "Burger" => 2,
                    "Italian" => 3,
                    _ => unreachable!(),
                });
                input.push(match &cap["est"] {
                    "0–10" => 0,
                    "10–30" => 1,
                    "30–60" => 2,
                    ">60" => 3,
                    _ => unreachable!(),
                });
                let output = yes_no(&cap["willwait"]) != 0;

                (input, output).into()
            })
            .collect()
    }

    #[test]
    fn train_test() {
        let input_scheme = vec![2, 2, 2, 2, 3, 3, 2, 2, 4, 4];
        let examples = parse_examples(DATA);
        let trainer = Trainer::new(input_scheme, &examples);
        let tree = trainer.train();

        // trained decision tree should be consistent to the training set
        for example in examples {
            assert_eq!(tree.classify(&example.input), example.output);
        }
    }
}
