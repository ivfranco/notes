#![allow(dead_code)]

use mdp::{policy_iteration, worlds, zero_sum_value_iteration, Player, Prob, SoloMDP, Util, MDP};

fn main() {
    exercise_17_1();
    exercise_17_2();
    // takes a few seconds
    // exercise_17_5();
    exercise_17_7();
}

fn report_map_dist(dist: &[Prob]) {
    use worlds::two_terminals::*;

    for (i, p) in dist.iter().enumerate() {
        let pos = Pos::from_usize(i);
        println!("P(X = {:?}) = {}", pos, p);
    }
}

fn exercise_17_1() {
    use worlds::two_terminals::*;
    use Dir::*;

    println!("\n17.1");

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
    use worlds::two_terminals::*;
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

    for _ in 0..1000 {
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

fn exercise_17_5() {
    use worlds::two_terminals::*;

    println!("\n17.5");

    let mut start = -2.0;
    let mut policy = policy_iteration(&Map::new(start));
    let mut end = start;

    while end < 0.0 {
        end += 0.001;
        let next_policy = policy_iteration(&Map::new(end));
        if policy != next_policy {
            println!("[{}, {}]", start, end - 0.001);
            start = end;
            policy = next_policy;
        }
    }
}

fn exercise_17_7() {
    use worlds::simple_game::*;

    println!("\n17.7");

    fn report_board_utils(board: &Board, player: Player, utils: &[Util]) {
        let tag = if player == Player::Maxer { "A" } else { "B" };

        for (i, u) in utils.iter().enumerate() {
            let state = board.decode(i);
            if board.valid(state) {
                println!("U{}{:?} = {}", tag, state, u);
            }
        }
    }

    let board = Board::default();
    let (ua, ub) = zero_sum_value_iteration(&board, 0.001);
    println!("Utility of Player A:");
    report_board_utils(&board, Player::Maxer, &ua);
    println!("Utility of Player B:");
    report_board_utils(&board, Player::Miner, &ub);
}
