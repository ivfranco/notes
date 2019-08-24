use mdp::*;

fn main() {
    exercise_17_1();
    exercise_17_2();
    // takes a few seconds
    // exercise_17_5();
    exercise_17_7();
    exercise_17_8();
    exercise_17_10();
    exercise_17_13();
}

fn report_map_dist(map: &worlds::two_terminals::Map, dist: &[Prob]) {
    for (i, p) in dist.iter().enumerate() {
        let pos = map.decode(i);
        println!("P(X = {:?}) = {:.5}", pos, p);
    }
}

fn exercise_17_1() {
    use worlds::two_terminals::*;
    use Dir::*;

    println!("\n17.1");

    let map = Map::new(0.0);
    let mut dist = vec![0.0; map.states()];
    dist[map.encode(&Pos::new(0, 0))] = 1.0;

    for dir in [N, N, E, E, E].iter() {
        let mut next_dist = vec![0.0; map.states()];
        for (i, p) in dist.into_iter().enumerate() {
            let state = map.decode(i);
            for (q, target) in map.apply(&state, dir) {
                next_dist[map.encode(&target)] += p * q;
            }
        }
        dist = next_dist;
    }

    report_map_dist(&map, &dist);
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
    dist[map.encode(&Pos::new(0, 0))] = 1.0;

    for _ in 0..1000 {
        let mut next_dist = vec![0.0; map.states()];
        for (i, p) in dist.into_iter().enumerate() {
            let state = map.decode(i);
            if map.actions(&state).is_empty() {
                next_dist[i] += p;
            } else {
                for (q, target) in map.apply(&state, &policy[i]) {
                    next_dist[map.encode(&target)] += p * q;
                }
            }
        }
        dist = next_dist;
    }

    report_map_dist(&map, &dist);
}

#[allow(dead_code)]
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

fn exercise_17_8() {
    use worlds::two_terminals::*;

    fn report_map_policy(map: &Map, policy: &[Option<Dir>]) {
        for chunk in policy.chunks_exact(map.width()).rev() {
            println!("{:?}", chunk);
        }
    }

    println!("\n17.8");

    for &r in [100.0, -3.0, 0.0, 3.0].iter() {
        let width = 3;
        let pos_final = Pos::new(2, 2);
        let mut rewards = vec![-1.0; 9];
        rewards[Pos::new(0, 2).to_usize(width)] = r;
        rewards[pos_final.to_usize(width)] = 10.0;
        let map = Map::full(0.99, width, rewards, &[], &[pos_final]);

        println!("r = {}", r);
        let policy = policy_iteration(&map);
        report_map_policy(&map, &policy);
    }
}

fn exercise_17_10() {
    use worlds::three_states::*;
    use Move::*;

    println!("\n17.10");

    let init_policy = vec![Some(B), Some(B), None];
    let context = Context::new(1.0);
    println!(
        "Init = [B, B, _]: {:?}",
        policy_iteration_with_init(&context, init_policy)
    );

    let mut discount = 1.0;
    while discount > 0.0 {
        let init_policy = vec![Some(A), Some(A), None];
        let context = Context::new(discount);
        println!(
            "Init = [A, A, _], discount = {:.1}: {:?}",
            discount,
            policy_iteration_with_init(&context, init_policy)
        );
        discount -= 0.1;
    }
}

fn normalize(slice: &mut [Prob]) {
    let sum: f64 = slice.iter().sum();
    slice.iter_mut().for_each(|p| *p /= sum)
}

fn exercise_17_13() {
    use worlds::two_terminals::*;
    println!("\n17.13");

    let map = Map::default();
    let mut dist = vec![1.0; map.states()];
    for (i, p) in dist.iter_mut().enumerate() {
        let pos = map.decode(i);
        if map.trap(pos) {
            *p = 0.0;
        }
    }
    normalize(&mut dist);

    let sensor: Vec<Prob> = (0..map.states())
        .map(|i| {
            let pos = map.decode(i);
            let walls = map.walls(pos);
            if walls == 1 {
                0.9
            } else {
                0.1
            }
        })
        .collect();

    for b in 0..2 {
        let mut next_dist = vec![0.0; map.states()];
        for (i, p) in dist.into_iter().enumerate() {
            let pos = map.decode(i);
            if !map.trap(pos) {
                for (q, target) in map.apply(&pos, &Dir::W) {
                    next_dist[map.encode(&target)] += p * q;
                }
            } else {
                next_dist[i] += p;
            }
        }

        for (i, p) in next_dist.iter_mut().enumerate() {
            *p *= sensor[i];
        }
        normalize(&mut next_dist);

        dist = next_dist;
        println!("believe state at t = {}:", b + 1);
        report_map_dist(&map, &dist);
    }
}
