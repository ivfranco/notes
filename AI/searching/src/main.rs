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

    #[derive(Debug)]
    enum Dir {
        Left,
        Right,
    }

    fn dir(mut n: u32) -> Vec<Dir> {
        assert!(n >= 1);
        let mut dirs: Vec<_> = std::iter::from_fn(|| {
            if n == 1 {
                None
            } else {
                let dir = if n % 2 == 0 { Dir::Left } else { Dir::Right };
                n /= 2;
                Some(dir)
            }
        })
        .collect();

        dirs.reverse();
        dirs
    }

    println!("{:?}", dir(11));
}

fn exercise_3_17() {
    println!("3.17");
}