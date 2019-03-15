use dag::env::Env;
use dag::symbolic;

fn main() {
    exercise_6_1_1();
    exercise_6_1_2();
    exercise_6_2_1();
    exercise_6_3_1();
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

fn exercise_6_2_1() {
    println!("Exercise 6.2.1:");

    println!("{:?}", symbolic::DAG::parse("a + -(b + c)").unwrap());
}

fn exercise_6_3_1() {
    println!("Exercise 6.3.1:");

    let decls = "float x;
record { float x; float y; } p;
record { int tag; float x; float y; } q;";

    println!("{:?}", Env::parse(decls).unwrap());
}
