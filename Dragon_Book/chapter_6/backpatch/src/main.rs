use backpatch::Boolean;

fn main() {
    exercise_6_7_1();
}

const START: usize = 100;

fn exercise_6_7_1() {
    println!("Exercise 6.7.1:");

    for boolean in &[
        "a==b && (c==d || e==f)",
        "(a==b || c==d) || e==f",
        "(a==b && c==d) && e==f",
    ] {
        println!("{}", boolean);
        println!("{:?}", Boolean::parse(START, boolean).unwrap());
    }
}
