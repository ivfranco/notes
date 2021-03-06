use bayes::{
    examples,
    network::cpt::{Full, CPT, F, T},
    network::{evidence_from, Network, Prob, Value, Variable},
};

fn main() {
    exercise_14_1();
    exercise_14_4();
    exercise_14_14();
    exercise_14_18();
    exercise_14_21();
    exercise_16_5();
    exercise_16_15();
    exercise_16_17();
    exercise_20_10();
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
    println!("P(MBC | MAB = win, MAC = draw) = {:?}", dist,);

    let mut samples = 2;
    const E: Prob = 0.01;
    while (network.gibbs_sampling(mbc, WIN, &evidence, samples) - dist[WIN]) > E {
        samples *= 2;
    }
    println!("Converged with {} samples", samples);
}

fn exercise_16_5() {
    println!("16.5");

    const ROUND: Value = 0;
    // const SQUARE: Value = 1;
    const RED: Value = 0;
    // const BROWN: Value = 1;
    const STRAWBERRY: Value = 0;
    // const ANCHOVY: Value = 1;

    let mut network = Network::new();
    let flavor = network.add_node(Variable::new_const(vec![0.7, 0.3]));
    let wrapper = network.add_node(Variable::binary_single_parent(flavor, 0.2, 0.9));
    let shape = network.add_node(Variable::binary_single_parent(flavor, 0.2, 0.9));

    println!(
        "p(red) = {}",
        network.query(wrapper, &evidence_from(&[]))[RED]
    );
    println!(
        "p(strawberry | round, red) = {}",
        network.query(flavor, &evidence_from(&[(shape, ROUND), (wrapper, RED)]))[STRAWBERRY]
    );
}

fn exercise_16_15() {
    println!("16.15");

    let mut network = Network::new();
    let b = network.add_node(Variable::new_const(vec![0.5, 0.5]));
    let m = network.add_node(Variable::binary_single_parent(b, 0.7, 0.9));
    let mut p_cpt = Full::new(&[b, m]);
    p_cpt.insert_in_binary_order(&[0.3, 0.8, 0.5, 0.9]);
    let p = network.add_node(Variable::new(p_cpt.into(), 2));

    let no_buy_pass = network.query(p, &evidence_from(&[(b, F)]));
    println!("EU(~b) = {}", no_buy_pass[T] * 2000.0);
    let buy_pass = network.query(p, &evidence_from(&[(b, T)]));
    println!("EU(b) = {}", buy_pass[T] * 2000.0 - 100.0);
}

fn exercise_16_17() {
    println!("16.17");

    const GOOD_GAIN: f64 = 2000.0 - 1500.0;
    const BAD_GAIN: f64 = GOOD_GAIN - 700.0;
    const FAIL: Value = 0;
    const PASS: Value = 1;

    fn utility(q_dist: &[f64]) -> f64 {
        q_dist[F] * BAD_GAIN + q_dist[T] * GOOD_GAIN
    }

    let mut network = Network::new();
    let q = network.add_node(Variable::new_const(vec![0.3, 0.7]));
    let t = network.add_node(Variable::binary_single_parent(q, 0.35, 0.8));

    let eu = utility(&network.query(q, &evidence_from(&[])));
    println!("EU(c) = {}", eu);

    let pt = network.query(t, &evidence_from(&[]));
    println!("P(Test) = {:?}", pt);
    println!(
        "P(Quality | pass) = {:?}",
        network.query(q, &evidence_from(&[(t, T)]))
    );
    println!(
        "P(Quality | fail) = {:?}",
        network.query(q, &evidence_from(&[(t, F)]))
    );

    let pass_q = network.query(q, &evidence_from(&[(t, T)]));
    println!("EU(c | pass) = {}", utility(&pass_q));
    let fail_q = network.query(q, &evidence_from(&[(t, F)]));
    println!("EU(c | fail) = {}", utility(&fail_q));

    println!(
        "VPI(Test) = {}",
        utility(&pass_q) * pt[PASS] + utility(&fail_q) * pt[FAIL] - eu
    );
}

mod candy {
    use super::*;
    use petgraph::prelude::*;

    // value order:
    // Bag = [1, 2]
    // Flavor = [cherry, lime]
    // Wrapper = [red, green]
    // Hold = [no, has]

    pub type Nodes = [NodeIndex; 4];
    pub type Params = [Prob; 7];
    pub type Sample = [[[usize; 2]; 2]; 2];

    pub const CHERRY: Value = 0;
    pub const LIME: Value = 1;
    pub const RED: Value = 0;
    pub const GREEN: Value = 1;
    pub const NOHOLE: Value = 0;
    pub const HASHOLE: Value = 1;

    pub fn candy_network([theta, f1, w1, h1, f2, w2, h2]: Params) -> (Network, Nodes) {
        let mut network = Network::new();
        let bag = network.add_node(Variable::new_const(vec![theta, 1.0 - theta]));
        let flavor = network.add_node(Variable::binary_single_parent(bag, 1.0 - f1, 1.0 - f2));
        let wrapper = network.add_node(Variable::binary_single_parent(bag, 1.0 - w1, 1.0 - w2));
        let hole = network.add_node(Variable::binary_single_parent(bag, h1, h2));

        (network, [bag, flavor, wrapper, hole])
    }

    pub fn update_on_sample(
        network: &Network,
        [bag, flavor, wrapper, hole]: Nodes,
        sample: &Sample,
    ) -> Params {
        let mut bag_1 = 0.0;
        let (mut f1, mut w1, mut h1) = (0.0, 0.0, 0.0);
        let (mut f2, mut w2, mut h2) = (0.0, 0.0, 0.0);

        for &f in [CHERRY, LIME].iter() {
            for &w in [RED, GREEN].iter() {
                for &h in [NOHOLE, HASHOLE].iter() {
                    let b =
                        network.query(bag, &evidence_from(&[(flavor, f), (wrapper, w), (hole, h)]));
                    let cnt = sample[f][w][h] as f64;
                    let bag_one_weight = b[0] * cnt;
                    let bag_two_weight = b[1] * cnt;
                    bag_1 += bag_one_weight;
                    if f == CHERRY {
                        f1 += bag_one_weight;
                        f2 += bag_two_weight;
                    }
                    if w == RED {
                        w1 += bag_one_weight;
                        w2 += bag_two_weight;
                    }
                    if h == HASHOLE {
                        h1 += bag_one_weight;
                        h2 += bag_two_weight;
                    }
                }
            }
        }

        let sample_cnt = sample
            .iter()
            .flat_map(|wrappers| wrappers.iter())
            .flat_map(|holes| holes.iter())
            .sum::<usize>() as f64;

        let bag_2 = sample_cnt - bag_1;
        [
            bag_1 / sample_cnt,
            f1 / bag_1,
            w1 / bag_1,
            h1 / bag_1,
            f2 / bag_2,
            w2 / bag_2,
            h2 / bag_2,
        ]
    }
}

fn exercise_20_10() {
    use candy::*;

    println!("\n20.10");

    const SAMPLE: Sample = [[[93, 273], [90, 104]], [[100, 79], [167, 94]]];

    let (network, nodes) = candy_network([0.6, 0.6, 0.6, 0.6, 0.4, 0.4, 0.4]);
    let [theta, f1, w1, h1, f2, w2, h2] = update_on_sample(&network, nodes, &SAMPLE);

    println!("θ(1) = {}", theta);
    println!("θF1(1) = {}", f1);
    println!("θW1(1) = {}", w1);
    println!("θH1(1) = {}", h1);
    println!("θF2(1) = {}", f2);
    println!("θW2(1) = {}", w2);
    println!("θH2(1) = {}", h2);

    let mut params = [0.6; 7];
    for _ in 0..10 {
        println!("{:?}", params);
        let (network, nodes) = candy_network(params);
        params = update_on_sample(&network, nodes, &SAMPLE);
    }
}
