use classification::decision_tree::*;

fn main() {
    exercise_18_6();
}

fn exercise_18_6() {
    println!("\n18.6");

    let examples = vec![
        (vec![1, 0, 0], false).into(),
        (vec![1, 0, 1], false).into(),
        (vec![0, 1, 0], false).into(),
        (vec![1, 1, 1], true).into(),
        (vec![1, 1, 0], true).into(),
    ];

    let input_scheme = vec![2; 3];

    let trainer = Trainer::new(input_scheme, &examples);
    let _ = trainer.train(TrainOption::Full);
}
