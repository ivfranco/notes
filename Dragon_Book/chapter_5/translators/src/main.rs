use translators::calculator::Node;
use translators::llcalculator::ExprNode;

fn main() {
    exercise_5_1_1();
    exercise_5_1_3();
}

fn exercise_5_1_1() {
    println!("Exercise 5.1.1:");

    println!("{:?}", Node::parse("(3 + 4) * (5 + 6) n").unwrap());
    println!("{:?}", Node::parse("1 * 2 * 3 * (4 * 5) n").unwrap());
    println!("{:?}", Node::parse("(9 + 8 * (7 + 6) + 5) n").unwrap());
}

fn exercise_5_1_3() {
    println!("Exercise 5.1.3:");

    println!("{:?}", ExprNode::parse("(3 + 4) * (5 + 6) n").unwrap());
    println!("{:?}", ExprNode::parse("1 * 2 * 3 * (4 * 5) n").unwrap());
    println!("{:?}", ExprNode::parse("(9 + 8 * (7 + 6) + 5) n").unwrap());
}
