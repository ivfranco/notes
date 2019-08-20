use crate::*;
use rand::prelude::*;

pub struct Trainer<'a> {
    examples: &'a [Example],
    weights: Vec<f64>,
    learning_rate: f64,
}

impl<'a> Trainer<'a> {
    pub fn new(examples: &'a [Example], learning_rate: f64) -> Self {
        assert!(!examples.is_empty()); let n = examples[0].input.len();
        Trainer {
            examples,
            weights: vec![0.0; n + 1],
            learning_rate,
        }
    }

    fn error(&self, example: &Example) -> f64 {
        let y = if example.output { 1.0 } else { 0.0 };
        let log_sum = log_sum(&self.weights, &example.input);
        (y - log_sum) * log_sum * (1.0 - log_sum)
    }

    fn update_weights(&mut self) {
        let mut rng = thread_rng();
        let example = self.examples.choose(&mut rng).unwrap();
        let learning_factor = self.error(example) * self.learning_rate;
        let updated_weights: Vec<_> = self.weights.iter()
            .zip(Some(&1).into_iter().chain(example.input.iter()))
            .map(|(w, &x)| w + learning_factor * f64::from(x))
            .collect();
        
        self.weights = updated_weights;
    }

    pub fn train(mut self, iteration: usize) -> LogisticClassifier {
        let mut i = 0;
        while i < iteration {
            self.update_weights();
            i += 1;
        }

        LogisticClassifier { weights: self.weights }
    }
}

fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn lin_sum(weights: &[f64], input: &[Value]) -> f64 {
    weights.iter()
        .zip(Some(1).iter().chain(input.iter()))
        .map(|(w, &x)| w * f64::from(x))
        .sum()
}

fn log_sum(weights: &[f64], input: &[Value]) -> f64 {
    logistic(lin_sum(weights, input))
}

// const THRESHOLD: f64 = 1e-5;

// fn l2(ia: &[f64], ib: &[f64]) -> f64 {
//     ia.iter().zip(ib)
//         .map(|(a, b)| (a - b).powi(2))
//         .sum()
// }

pub struct LogisticClassifier {
    weights: Vec<f64>,
}

impl LogisticClassifier {
    pub fn classify(&self, input: &[Value]) -> f64 {
        log_sum(&self.weights, input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn logistic_test() {
        // bitwise AND function
        let examples = vec![
            (vec![0, 0], false).into(),
            (vec![1, 0], false).into(),
            (vec![0, 1], false).into(),
            (vec![1, 1], true).into(),
        ];

        let trainer = Trainer::new(&examples, 0.5);
        let classifier = trainer.train(10000);
        for example in examples.iter() {
            let y = if example.output { 1.0 } else { 0.0 };
            assert!((classifier.classify(&example.input) - y).abs() < 0.1);
        }
    }
}