use mdp::{
    Prob, Util, State, MDP, value_iteration, policy_from,
    worlds::two_terminals::*,
};

fn main() {
    exercise_17_1();
    exercise_17_2();
}

fn report_map_dist(dist: &[Prob]) {
    for (i, p) in dist.iter().enumerate() {
        let pos = Pos::from_usize(i);
        println!("P(X = {:?}) = {}", pos, p);
    }
}

fn exercise_17_1() {
    println!("\n17.1");
    use Dir::*;

    let map = Map::new(0.0);
    let mut dist = vec![0.0; map.states()];
    dist[Pos::new(0, 0).to_usize()] = 1.0;

    for dir in [N, N, E, E, E].iter() {
        let mut next_dist = vec![0.0; map.states()];
        for (i, p) in dist.into_iter().enumerate() {
            let state = Pos::from_usize(i);
            for (q, target) in map.apply(&state, dir) {
                next_dist[target.to_usize()] += p * q;
            }
        }
        dist = next_dist;
    }

    report_map_dist(&dist);
}

fn exercise_17_2() {
    use Dir::*;

    println!("\n17.2");

    let map = Map::new(0.1);
    #[rustfmt::skip]
    let policy = vec![
        W, W, W, S,
        W, W, W, W,
        W, W, W, W,
    ];

    let mut dist = vec![0.0; map.states()];
    dist[Pos::new(0, 0).to_usize()] = 1.0;

    for _ in 0 .. 1000 {
        let mut next_dist = vec![0.0; map.states()];
        for (i, p) in dist.into_iter().enumerate() {
            let state = Pos::from_usize(i);
            if map.actions(&state).is_empty() {
                next_dist[i] += p;
            } else {
                for (q, target) in map.apply(&state, &policy[i]) {
                    next_dist[target.to_usize()] += p * q;
                }
            }
        }
        dist = next_dist;
    }

    report_map_dist(&dist);
}