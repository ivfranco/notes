use std::collections::HashMap;

fn main() {
    exercise_13_10();
    exercise_13_11();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Outcome {
    Coin(i32),
    Ruin,
}

const FEE: i32 = 1;

type Outcomes = HashMap<Outcome, f64>;

fn exercise_13_10() {
    println!("13.10");

    let paybacks = paybacks();

    println!("mean payback: {}", mean_of(paybacks.iter().cloned()));
    println!(
        "chance to win: {}",
        paybacks
            .iter()
            .filter_map(|(c, p)| if *c > FEE { Some(p) } else { None })
            .sum::<f64>()
    );

    println!("mean: {}", slot_machine_mean(10, &paybacks));
    println!("median: {}", slot_machine_median(10, &paybacks));
}

fn slot_machine_mean(start: i32, paybacks: &[(i32, f64)]) -> u64 {
    const TRIALS: u64 = 10000;

    let mut sum: u64 = 0;
    for _ in 0 .. TRIALS {
        sum += trial(start, paybacks);
    }

    sum / TRIALS
}

fn trial(start: i32, paybacks: &[(i32, f64)]) -> u64 {
    use rand::prelude::*;

    let mut coins = start;
    let mut rng = thread_rng();
    let mut iter = 0;

    while coins > 0 {
        let (c, _) = paybacks.choose_weighted(&mut rng, |(_, p)| *p).unwrap();
        coins += c - FEE;
        iter += 1;
    }

    iter
}

fn slot_machine_median(start: i32, paybacks: &[(i32, f64)]) -> i32 {
    let mut outcomes: Outcomes = Outcomes::new();
    outcomes.insert(Outcome::Coin(start), 1.0);
    let mut iter = 0;

    loop {
        iter += 1;

        let next_outcomes = evolve(&paybacks, &outcomes);
        outcomes = next_outcomes;

        if outcomes.get(&Outcome::Ruin) >= Some(&0.5) {
            return iter;
        }
    }
}

fn paybacks() -> Vec<(i32, f64)> {
    let mut paybacks = vec![
        (20, 0.25f64.powi(3)),
        (15, 0.25f64.powi(3)),
        (5, 0.25f64.powi(3)),
        (3, 0.25f64.powi(3)),
        (2, 0.25f64.powi(2) * (1.0 - 0.25)),
        (1, 0.25 * ((1.0f64 - 0.25).powi(2) + (1.0 - 0.25) * 0.25)),
    ];

    let p_lose = 1.0 - paybacks.iter().map(|(_, p)| p).sum::<f64>();
    paybacks.push((0, p_lose));
    paybacks
}

fn evolve(paybacks: &[(i32, f64)], outcomes: &HashMap<Outcome, f64>) -> Outcomes {
    let mut next_outcomes = Outcomes::new();

    for (o, p) in outcomes {
        if let Outcome::Coin(c) = o {
            for (pay, chance) in paybacks {
                let next_outcome = if c + pay - FEE > 0 {
                    Outcome::Coin(c + pay - FEE)
                } else {
                    Outcome::Ruin
                };

                *next_outcomes.entry(next_outcome).or_insert(0.0) += p * chance;
            }
        }
    }

    *next_outcomes.entry(Outcome::Ruin).or_insert(0.0) +=
        outcomes.get(&Outcome::Ruin).unwrap_or(&0.0);
    next_outcomes
}

fn mean_of<I>(outcomes: I) -> f64
where
    I: IntoIterator<Item = (i32, f64)>,
{
    outcomes.into_iter().map(|(c, p)| f64::from(c) * p).sum()
}

fn exercise_13_11() {
    println!("13.11");

    const E: f64 = 0.001;
    const D: f64 = 0.01;

    let max_n = (0 ..)
        .find(|&n| {
            (1.0 - E).powi(n + 1) + E * (1.0 - E).powi(n) * f64::from(n + 1) < 1.0 - D
        })
        .unwrap() - 1;
    
    println!("{}", max_n);
}