use petgraph::dot::Dot;
use petgraph::prelude::*;
use searching::river_crossing::solve_river_crossing;

fn main() {
    exercise_3_9();
    exercise_3_15();
}

fn exercise_3_9() {
    println!("3.9");

    for state in solve_river_crossing(3, 3).unwrap() {
        println!("{:?}", state);
    }
}

fn exercise_3_15() {
    println!("3.15");
    let mut graph: GraphMap<u32, (), Directed> = GraphMap::new();

    for i in 1..=15 {
        graph.add_node(i);
        let j = i * 2;
        if j <= 15 {
            graph.add_edge(i, j, ());
        }
        if j < 15 {
            graph.add_edge(i, j + 1, ());
        }
    }

    println!("{:?}", Dot::new(&graph));
}