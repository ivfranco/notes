use and_or_search::vaccum_cleaner::{Erratic, explore};
use petgraph::dot::Dot;

fn main() {
    exercise_4_10();
}

fn exercise_4_10() {
    println!("4.10");

    let graph = explore(Erratic::enumerate());
    println!("{:?}", Dot::new(&graph));
}