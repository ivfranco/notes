use delays::{report, Instr::*};

fn main() {
    exercise_10_5_8();
}

fn exercise_10_5_8() {
    println!("Exercise 10.5.8:");

    let (fit_0, fit_1) = report(&mut [A, A, B, B, C, C], 2);
    println!("Requires no delay: {}", fit_0 / 8);
    println!("Requires one delay: {}", fit_1 / 8);
}
