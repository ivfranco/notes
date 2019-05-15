use bdd::parse_expr;
use boolean_expression::BDD;

fn main() {
    exercise_12_7_1();
    exercise_12_7_2();
}

fn exercise_12_7_1() {
    println!("Exercise 12.7.1");
    //  a = (0, 0)
    //  b = (0, 1)
    //  c = (1, 0)
    let expr = parse_expr(
        "
        (~w & x & ~y & z) |
        (w & ~x & ~y & ~z) |
        (~w & x & ~y & ~z)
    ",
    )
    .unwrap();

    let mut bdd = BDD::new();
    let func = bdd.from_expr(&expr);
    println!("{}", bdd.to_dot(func));
}

fn exercise_12_7_2() {
    println!("Exercise 12.7.2");

    let expr = parse_expr("a ^ b ^ c ^ d").unwrap();
    let mut bdd = BDD::new();
    let func = bdd.from_expr(&expr);
    println!("{}", bdd.to_dot(func));
}
