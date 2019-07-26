use bayes::{
    examples,
    network::cpt::{Full, CPT, F, T},
    network::{evidence_from, Network, Value, Variable, Prob},
};

fn main() {
    exercise_14_1();
    exercise_14_4();
    exercise_14_14();
    exercise_14_18();
    exercise_14_21();
}

fn exercise_14_1() {
    println!("14.1");

    const H: Value = 1;
    const T: Value = 0;

    const A: Value = 0;
    const B: Value = 1;
    const C: Value = 2;

    let mut network = Network::new();

    let coin = network.add_node(Variable::new_const(vec![1.0 / 3.0; 3]));

    let mut flip = Full::new(&[coin]);
    flip.insert_binary_row(&[(coin, A)], 0.2);
    flip.insert_binary_row(&[(coin, B)], 0.6);
    flip.insert_binary_row(&[(coin, C)], 0.8);
    let flip_cpt = CPT::Full(flip);
    let x1 = network.add_node(Variable::new(flip_cpt.clone(), 2));
    let x2 = network.add_node(Variable::new(flip_cpt.clone(), 2));
    let x3 = network.add_node(Variable::new(flip_cpt, 2));

    // other cases would be symmetric
    let evidence = [(x1, H), (x2, H), (x3, T)].iter().cloned().collect();

    println!("{:?}", network.query(coin, &evidence));
}

fn exercise_14_4() {
    use examples::burglary::*;

    println!("14.4");

    let (network, nodes) = burglary_network();
    let [burglary, earthquake, alarm, _, _] = nodes;

    let evidence = [(alarm, T), (burglary, T)].iter().cloned().collect();
    println!(
        "P(Earthquake | alarm, burglary) = {:?}",
        network.query(earthquake, &evidence)
    );
    let evidence = [(alarm, T), (burglary, F)].iter().cloned().collect();
    println!(
        "P(Earthquake | alarm, ~burglary) = {:?}",
        network.query(earthquake, &evidence)
    );
    let evidence = [(alarm, T), (earthquake, T)].iter().cloned().collect();
    println!(
        "P(Burglary | alarm, earthquake) = {:?}",
        network.query(burglary, &evidence)
    );
    let evidence = [(alarm, T), (earthquake, F)].iter().cloned().collect();
    println!(
        "P(Burglary | alarm, ~earthquake) = {:?}",
        network.query(burglary, &evidence)
    );
}

fn exercise_14_14() {
    println!("14.14");

    let mut network = Network::new();
    let law = network.add_node(Variable::new_binary_const(0.9));
    let motivated = network.add_node(Variable::new_binary_const(0.1));

    let mut i_cpt = Full::new(&[law, motivated]);
    i_cpt.insert_in_binary_order(&[0.1, 0.5, 0.5, 0.9]);
    let indicted = network.add_node(Variable::new(i_cpt.into(), 2));

    let mut g_cpt = Full::new(&[law, indicted, motivated]);
    g_cpt.insert_in_binary_order(&[0.0, 0.0, 0.1, 0.2, 0.0, 0.0, 0.8, 0.9]);
    let guilty = network.add_node(Variable::new(g_cpt.into(), 2));

    let mut j_cpt = Full::new(&[guilty]);
    j_cpt.insert_in_binary_order(&[0.0, 0.9]);
    let jailed = network.add_node(Variable::new(j_cpt.into(), 2));

    let evidence = evidence_from(&[(law, T), (indicted, T), (motivated, T)]);
    println!("P(J | b, i, m) = {:?}", network.query(jailed, &evidence));
}

fn exercise_14_18() {
    println!("14.18");

    let mut network = Network::new();
    let cloudy = network.add_node(Variable::new_binary_const(0.5));

    let mut sprinker_cpt = Full::new(&[cloudy]);
    sprinker_cpt.insert_in_binary_order(&[0.5, 0.1]);
    let sprinker = network.add_node(Variable::new(sprinker_cpt.into(), 2));

    let mut rain_cpt = Full::new(&[cloudy]);
    rain_cpt.insert_in_binary_order(&[0.2, 0.8]);
    let rain = network.add_node(Variable::new(rain_cpt.into(), 2));

    let mut wet_grass_cpt = Full::new(&[sprinker, rain]);
    wet_grass_cpt.insert_in_binary_order(&[0.0, 0.9, 0.9, 0.99]);
    let wet_grass = network.add_node(Variable::new(wet_grass_cpt.into(), 2));

    let mut evidence = evidence_from(&[(sprinker, T), (wet_grass, T)]);
    for &(trans, fix) in &[(cloudy, rain), (rain, cloudy)] {
        for &fix_truth in &[T, F] {
            evidence.insert(fix, fix_truth);
            let (t_name, f_name) = if trans == cloudy {
                ("Cloudy", "rain")
            } else {
                ("Rain", "cloudy")
            };

            let unop = if fix_truth == T { "" } else { "~" };

            println!(
                "P({} | {}{}, sprinker, wet_grass) = {:?}",
                t_name,
                unop,
                f_name,
                network.query(trans, &evidence)
            );

            evidence.remove(&fix);
        }
    }
}

fn exercise_14_21() {
    println!("14.21");

    const WIN: Value = 0;
    const DRAW: Value = 1;
    // const LOSE: Value = 2;

    let mut network = Network::new();
    let a = network.add_node(Variable::new_const(vec![0.25, 0.25, 0.25, 0.25]));
    let b = network.add_node(Variable::new_const(vec![0.25, 0.25, 0.25, 0.25]));
    let c = network.add_node(Variable::new_const(vec![0.25, 0.25, 0.25, 0.25]));

    let mut matches = vec![];
    for &(home, away) in &[(a, b), (a, c), (b, c)] {
        let mut m_cpt = Full::new(&[home, away]);
        for home_quality in 0..=3 {
            for away_quality in 0..=3 {
                let diff = home_quality as f64 - away_quality as f64;
                let p_draw = 1.0 - (diff.abs() + 3.0) / 7.0;
                let p_win = (1.0 - p_draw) * (4.0 + diff) / 8.0;
                let p_lose = (1.0 - p_draw) * (4.0 - diff) / 8.0;
                m_cpt.insert_row(
                    &[(home, home_quality), (away, away_quality)],
                    &[p_win, p_draw, p_lose],
                );
            }
        }
        let m = network.add_node(Variable::new(m_cpt.into(), 3));
        matches.push(m);
    }
    let (mab, mac, mbc) = (matches[0], matches[1], matches[2]);

    let evidence = evidence_from(&[(mab, WIN), (mac, DRAW)]);
    let dist = network.query(mbc, &evidence);
    println!(
        "P(MBC | MAB = win, MAC = draw) = {:?}",
        dist,
    );

    let mut samples = 2;
    const E: Prob = 0.01;
    while (network.gibbs_sampling(mbc, WIN, &evidence, samples) - dist[WIN]) > E {
        samples *= 2;
    }
    println!("Converged with {} samples", samples);
}
