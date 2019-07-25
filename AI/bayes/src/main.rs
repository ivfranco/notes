use bayes::{
    examples,
    network::cpt::{Full, CPT, T},
    network::{evidence_from, Network, Value, Variable},
};

fn main() {
    exercise_14_1();
    exercise_14_4();
    exercise_14_14();
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

    println!("{:?}", network.query(coin, evidence));
}

fn exercise_14_4() {
    use examples::burglary::*;

    println!("14.4");

    let (network, nodes) = burglary_network();
    let [burglary, earthquake, alarm, _, _] = nodes;

    let evidence = [(alarm, T), (burglary, T)].iter().cloned().collect();
    println!(
        "P(Earthquake | alarm, burglary) = {:?}",
        network.query(earthquake, evidence)
    );
    let evidence = [(alarm, T), (burglary, F)].iter().cloned().collect();
    println!(
        "P(Earthquake | alarm, ~burglary) = {:?}",
        network.query(earthquake, evidence)
    );
    let evidence = [(alarm, T), (earthquake, T)].iter().cloned().collect();
    println!(
        "P(Burglary | alarm, earthquake) = {:?}",
        network.query(burglary, evidence)
    );
    let evidence = [(alarm, T), (earthquake, F)].iter().cloned().collect();
    println!(
        "P(Burglary | alarm, ~earthquake) = {:?}",
        network.query(burglary, evidence)
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
    println!("P(J | b, i, m) = {:?}", network.query(jailed, evidence));
}
