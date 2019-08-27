use mdp::{learn::*, *};

fn main() {
    exercise_17_1();
    exercise_17_2();
    // takes a few seconds
    // exercise_17_5();
    exercise_17_7();
    exercise_17_8();
    exercise_17_10();
    exercise_17_13();
    exercise_21_5();
    exercise_21_8();
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

fn print_map_util(utils: &[Util], width: usize) {
    for row in utils.chunks_exact(width).rev() {
        for util in row {
            print!("{:<8.3}", util);
        }
        println!();
    }
}

fn exercise_21_5() {
    use mdp::worlds::two_terminals::*;

    println!("\n21.5");

    fn single_terminal(width: usize, height: usize, (tx, ty): (usize, usize)) -> Map {
        let mut rewards = vec![-0.04; width * height];
        let terminal = tx + ty * width;
        rewards[terminal] = 1.0;
        Map::full(1.0, width, rewards, &[], &[Pos::from_usize(terminal, width)])
    }

    for map in &[
        Map::default(),
        single_terminal(10, 10, (9, 9)),
        single_terminal(10, 10, (4, 4)),
    ] {
        let utils = value_iteration(map, 1e-5);
        let policy = policy_from(map, &utils);
        let tp = temporal_difference(map, &policy, 10000);
        // length of a trial will be propotional to the area of the map
        let linear = linear_temporal_difference(map, &policy, 100);

        println!("exact utils:");
        print_map_util(&utils, map.width());
        println!("tp utils:");
        print_map_util(&tp, map.width());
        println!("linear tp utils:");
        print_map_util(&linear, map.width());
    }
}

fn exercise_21_8() {
    use mdp::worlds::two_terminals::*;
    use rand::prelude::*;

    println!("\n21.8");

    let mut rng = thread_rng();

    fn single_terminal(pos_terms: &[Pos], neg_terms: &[Pos], blocks: &[Pos]) -> Map {
        let mut rewards = vec![-0.04; 100];
        for Pos { x, y } in pos_terms {
            rewards[(x + y * 10) as usize] = 1.0;
        }
        for Pos { x, y } in neg_terms {
            rewards[(x + y * 10) as usize] = -1.0;
        }
        for Pos { x, y } in blocks {
            rewards[(x + y * 10) as usize] = 0.0;
        }

        let mut terminals = pos_terms.to_vec();
        terminals.extend(neg_terms.iter().cloned());
        Map::full(1.0, 10, rewards, blocks, &terminals)
    }

    let mut maps = vec![];
    let pos_term = Pos::new(9, 9);
    let neg_term = Pos::new(9, 1);
    maps.push(single_terminal(&[pos_term], &[], &[]));
    maps.push(single_terminal(&[pos_term], &[neg_term], &[]));

    let mut blocks = vec![]; 
    for _ in 0 .. 10 {
        let pos = Pos::new(rng.gen_range(0, 10), rng.gen_range(0, 10));
        if pos != pos_term && pos != neg_term {
            blocks.push(pos);
        }
    }
    maps.push(single_terminal(&[pos_term], &[neg_term], &blocks));

    let blocks: Vec<_> = (1 ..= 8).map(|y| Pos::new(4, y)).collect();
    maps.push(single_terminal(&[pos_term], &[neg_term], &blocks));
    
    maps.push(single_terminal(&[Pos::new(4, 4)], &[], &[]));

    for (i, map) in maps.into_iter().enumerate() {
        println!("Environment {}", i + 1);
        let utils = value_iteration(&map, 1e-5);
        print_map_util(&utils, 10);

        let mut x_mean = 0.0;
        let mut y_mean = 0.0;
        let mut u_mean = 0.0;
        let mut xx = 0.0;
        let mut xy = 0.0;
        let mut yy = 0.0;
        let mut xu = 0.0;
        let mut yu = 0.0;

        for (y, row) in utils.chunks_exact(10).enumerate() {
            for (x, &util) in row.iter().enumerate() {
                let y = y as f64;
                let x = x as f64;

                x_mean += x;
                y_mean += y;
                u_mean += util;
                xx += x * x;
                yy += y * y;
                xy += x * y;
                xu += x * util;
                yu += y * util;
            }
        }

        x_mean /= 100.0;
        y_mean /= 100.0;
        u_mean /= 100.0;

        let theta_1 = (yy * xu - xy * yu) / (xx * yy - xy.powi(2));
        let theta_2 = (xx * yu - xy * xu) / (xx * yy - xy.powi(2));
        let theta_0 = u_mean - theta_1 * x_mean - theta_2 * y_mean;

        println!("U(x, y) = {:.4} + {:.4}x + {:.4}y", theta_0, theta_1, theta_2);
    }
}
