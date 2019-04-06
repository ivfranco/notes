use ershov::builder::Builder;
use ershov::Node;

fn main() {
    exercise_8_10_1();
    exercise_8_10_2();
    exercise_8_10_3();
}

const EXPR_ONE: &str = "a/(b + c) - d * (e + f)";
const EXPR_TWO: &str = "a + b * (c * (d + e))";
const EXPR_THREE: &str = "(-a + *p) * ((b - *q)/(-c + *r))";

fn exercise_8_10_1() {
    println!("Exercise 8.10.1:");

    for expr in &[EXPR_ONE, EXPR_TWO, EXPR_THREE] {
        let node = Node::parse(expr).unwrap();
        println!("expr: {}", expr);
        println!("Ershov number: {}", node.label);
    }
}

fn exercise_8_10_2() {
    println!("Exercise 8.10.2:");

    for expr in &[EXPR_ONE, EXPR_TWO, EXPR_THREE] {
        let node = Node::parse(expr).unwrap();
        println!("{:?}", Builder::build(&node, 2));
    }
}

fn exercise_8_10_3() {
    println!("Exercise 8.10.3:");

    for expr in &[EXPR_ONE, EXPR_TWO, EXPR_THREE] {
        let node = Node::parse(expr).unwrap();
        println!("{:?}", Builder::build(&node, 3));
    }
}
