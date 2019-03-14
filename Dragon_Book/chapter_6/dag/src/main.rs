use dag::symbolic;

fn main() {
    exercise_6_1_1();
    exercise_6_1_2();
}

fn exercise_6_1_1() {
    println!("Exercise 6.1.1:");

    println!(
        "{:?}",
        symbolic::DAG::parse("((x + y) - ((x + y) * (x - y))) + ((x + y) * (x - y))").unwrap()
    );
}

fn exercise_6_1_2() {
    println!("Exercise 6.1.2:");

    println!("{:?}", symbolic::DAG::parse("a + b + (a + b)").unwrap());
    println!("{:?}", symbolic::DAG::parse("a + b + a + b").unwrap());
    println!(
        "{:?}",
        symbolic::DAG::parse("a + a + (a + a + a + (a + a + a + a))").unwrap()
    );
}
