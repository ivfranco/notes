#![allow(dead_code)]

use pathfinding::prelude::astar;
use petgraph::{dot::Dot, prelude::*};
use rand::prelude::*;
use std::time::{Duration, Instant};

use searching::{
    eight_puzzle::Eight,
    eight_queens::Queens,
    local_search::{first_choice, random_restart, simulated_annealing, steepest_ascent},
    river_crossing::solve_river_crossing,
    tsp::{self, TSP},
    vaccum_cleaner::{Cleanliness, Room},
};

fn main() {
    exercise_3_9();
    exercise_3_15();
    exercise_3_20();
    exercise_3_28();
    exercise_3_31();
    exercise_4_3();
    exercise_4_4();
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
        )
        .unwrap();
        let (_, over_cost) = astar(
            &puzzle,
            |puzzle| puzzle.successors().into_iter().map(|succ| (succ, 1)),
            |_| random(),
            Eight::is_goal,
        )
        .unwrap();

        if over_cost > optimal_cost {
            print!("for start state\n{:?}", puzzle);
            println!(
                "optimal cost is {}, with random heuristic {}",
                optimal_cost, over_cost
            );
            break;
        }
    }
}

fn exercise_3_31() {
    println!("3.31");

    let (_, cost) = Eight::new([4, 5, 3, 1, 2, 6, 7, 8, 0]).solve().unwrap();
    println!("{}", cost);
}

fn exercise_4_3() {
    println!("4.3");

    const SAMPLE: usize = 100;

    let avg_ratio: f64 = (0..SAMPLE)
        .map(|_| {
            let map: tsp::Map = random();
            let start = TSP::new(0, &map);

            let optimal_cost = start.solve().cost();
            let local_maxima_cost = steepest_ascent(&start).cost();

            (local_maxima_cost / optimal_cost).into_inner()
        })
        .sum::<f64>()
        / (SAMPLE as f64);

    println!(
        "average cost ratio over {} samples is {}",
        SAMPLE, avg_ratio
    );
}

fn elapsed<F, R>(mut f: F) -> (R, Duration)
where
    F: FnMut() -> R,
{
    let now = Instant::now();
    let r = f();
    (r, now.elapsed())
}

fn exercise_4_4() {
    println!("4.4");

    const SAMPLE: usize = 1000;

    let mut steepest_sum = Duration::default();
    let mut steepest_success = 0;

    let mut first_choice_sum = Duration::default();
    let mut first_choice_success = 0;

    let mut restart_sum = Duration::default();

    let mut annealing_sum = Duration::default();
    let mut annealing_success = 0;

    for _ in 0..SAMPLE {
        let start = random::<Queens>();

        let (ret, cost) = elapsed(|| steepest_ascent(&start));
        steepest_sum += cost;
        if ret.is_goal() {
            steepest_success += 1;
        }

        let (ret, cost) = elapsed(|| first_choice(&start));
        first_choice_sum += cost;
        if ret.is_goal() {
            first_choice_success += 1;
        }

        let (ret, cost) = elapsed(|| simulated_annealing(&start, |t| (1000 - 10 * t) as f64));
        annealing_sum += cost;
        if ret.is_goal() {
            annealing_success += 1;
        }

        let (_, cost) = elapsed(|| random_restart(random::<Queens>));
        restart_sum += cost;
    }

    println!("over {} samples of eight queens:", SAMPLE);

    println!("Steepest ascent cost: {:?}", steepest_sum);
    println!(
        "Steepest ascent success ratio: {}",
        f64::from(steepest_success) / SAMPLE as f64
    );

    println!("First choice cost: {:?}", first_choice_sum);
    println!(
        "First choice success ratio: {}",
        f64::from(first_choice_success) / SAMPLE as f64
    );

    println!("Simulated annealing cost: {:?}", annealing_sum);
    println!(
        "Simulated annealing success ratio: {}",
        f64::from(annealing_success) / SAMPLE as f64
    );

    println!("Random restart cost: {:?}", restart_sum);
}
