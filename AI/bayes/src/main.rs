use bayes::{
    network::cpt::{Full, CPT},
    network::{Network, Value, Variable},
};

fn main() {
    exercise_14_1();
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
