use tree_rewrite::rewrite::Rewriter;
use tree_rewrite::Node;

fn main() {
    exercise_8_9_1();
}

fn exercise_8_9_1() {
    println!("Exercise 8.9.1:");

    for expr in &["x = a * b + c * d;", "x[i] = y[j] * z[k];", "x = x + 1;"] {
        let node = Node::parse(expr).unwrap();
        println!("{:?}", node);

        let mut rewriter = Rewriter::new();
        let binary = rewriter.rewrite_root(node).unwrap();
        println!("{:?}", binary);
    }
}
