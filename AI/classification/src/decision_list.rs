use super::*;

enum Outcome {
    Decision(bool),
    Undecided,
}

use Outcome::*;

struct Test {
    pairs: Vec<(Attr, Value)>,
    class: bool,
}

impl Test {
    fn new(pairs: Vec<(Attr, Value)>, class: bool) -> Self {
        Test { pairs, class }
    }

    fn classify(&self, input: &[Value]) -> Outcome {
        if self.pairs.iter().all(|&(attr, value)| input[attr] == value) {
            Decision(self.class)
        } else {
            Undecided
        }
    }
}

#[derive(Debug)]
pub enum TrainError {
    NonUniform,
}

pub struct Trainer<'a> {
    input_scheme: Vec<Value>,
    tests: Vec<Test>,
    examples: &'a [Example],
}

impl<'a> Trainer<'a> {
    pub fn new(input_scheme: Vec<Value>, examples: &'a [Example]) -> Self {
        Trainer {
            input_scheme,
            tests: vec![],
            examples,
        }
    }

    fn single_pair_train(&mut self) -> Result<(), TrainError> {
        let mut filtered: Vec<_> = self.examples.iter().collect();
        while !filtered.is_empty() {
            // iterate over all possible (Attr, Value) pairs
            // by no means optimized
            let triple = pairs(&self.input_scheme).find_map(|(attr, value)| {
                let (t, f) = class_count(
                    filtered
                        .iter()
                        .filter(|example| example.input[attr] == value)
                        .copied(),
                );
                if t > 0 && f == 0 {
                    Some((attr, value, true))
                } else if t == 0 && f > 0 {
                    Some((attr, value, false))
                } else {
                    None
                }
            });

            if let Some((attr, value, class)) = triple {
                self.tests.push(Test::new(vec![(attr, value)], class));
                filtered.retain(|example| example.input[attr] != value);
            } else {
                return Err(TrainError::NonUniform);
            }
        }
        Ok(())
    }

    pub fn train(mut self) -> Result<DecisionList, TrainError> {
        self.single_pair_train()?;
        Ok(DecisionList { tests: self.tests })
    }
}

fn pairs<'a>(input_scheme: &'a [Value]) -> impl Iterator<Item = (Attr, Value)> + 'a {
    input_scheme
        .iter()
        .enumerate()
        .flat_map(|(attr, &values)| (0..values).map(move |value| (attr, value)))
}

pub struct DecisionList {
    tests: Vec<Test>,
}

impl DecisionList {
    pub fn classify(&self, input: &[Value]) -> bool {
        for test in &self.tests {
            match test.classify(input) {
                Decision(output) => return output,
                Undecided => (),
            }
        }

        self.tests
            .last()
            .map(|test| !test.class)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    #[test]
    fn consistency_test() -> Result<(), TrainError> {
        let input_scheme = INPUT_SCHEME.to_vec();
        let examples = parse_examples(DATA);

        let trainer = Trainer::new(input_scheme, &examples);
        let decision_list = trainer.train()?;

        // as long as the training set is free of noise and error
        // trained decision list should be consistent to the training set
        for example in examples {
            assert_eq!(decision_list.classify(&example.input), example.output);
        }

        Ok(())
    }
}
