use petgraph::dot::Dot;
use petgraph::prelude::*;
use rand::prelude::*;
use searching::eight_puzzle::Eight;
use searching::vaccum_cleaner::{Cleanliness, Room};
use searching::river_crossing::solve_river_crossing;
use pathfinding::prelude::astar;

fn main() {
    exercise_3_9();
    exercise_3_15();
    exercise_3_20();
    exercise_3_28();
    exercise_3_31();
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

fn exercise_3_20() {
    use Cleanliness::*;

    println!("3.20");

    let fixed = Room::new(
        (1, 1),
        3,
        vec![
            Dirty, Dirty, Dirty, Clean, Clean, Clean, Clean, Clean, Clean,
        ],
    );

    for room in fixed.solve().0 {
        print!("{:?}", room);
    }

    const SAMPLE: usize = 1000;

    let sum = std::iter::from_fn(|| Some(Room::new_random(3, 0.2)))
        .take(SAMPLE)
        .map(|room| room.solve().1)
        .sum::<usize>();

    println!(
        "avarage cost of {} samples is {}",
        SAMPLE,
        sum as f64 / SAMPLE as f64
    );
}

fn exercise_3_28() {
    println!("3.28");

    loop {
        let puzzle = random::<Eight>();
        let (_, optimal_cost) = astar(
            &puzzle,
            |puzzle| puzzle.successors().into_iter().map(|succ| (succ, 1)),
            Eight::heuristic,
            Eight::is_goal,
        ).unwrap();
        let (_, over_cost) = astar(
            &puzzle,
            |puzzle| puzzle.successors().into_iter().map(|succ| (succ, 1)),
            |_| random(),
            Eight::is_goal,
        ).unwrap();

        if over_cost > optimal_cost {
            print!("for start state\n{:?}", puzzle);
            println!("optimal cost is {}, with random heuristic {}", optimal_cost, over_cost);
            break;
        }
    }
}

fn exercise_3_31() {
    println!("3.31");

    let (_, cost) = Eight::new([4, 5, 3, 1, 2, 6, 7, 8, 0]).solve().unwrap();
    println!("{}", cost);
}