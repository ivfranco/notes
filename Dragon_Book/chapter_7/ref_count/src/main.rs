use ref_count::Network;

fn main() {
    exercise_7_5_1();
    exercise_7_5_2();
}

fn network_one() -> Network {
    let mut network = Network::new();
    for object in &["X", "A", "B", "C", "D", "E", "F", "G", "H", "I"] {
        network.create_object(object, 0);
    }
    for (source, dest) in &[
        ("X", "A"),
        ("A", "B"),
        ("A", "C"),
        ("B", "D"),
        ("B", "E"),
        ("C", "F"),
        ("D", "G"),
        ("E", "C"),
        ("F", "H"),
        ("G", "E"),
        ("G", "H"),
        ("H", "I"),
        ("I", "G"),
    ] {
        network.create_reference(source, dest);
    }

    network
}

fn exercise_7_5_1() {
    println!("Exercise 7.5.1:");

    let mut network = network_one();
    network.remove_reference("A", "B");
    println!("{:?}", network);

    let mut network = network_one();
    network.remove_reference("X", "A");
    println!("{:?}", network);

    let mut network = network_one();
    network.remove_object("C");
    println!("{:?}", network);
}

fn network_two() -> Network {
    let mut network = Network::new();
    for object in &["X", "Y", "A", "B", "C", "D", "E", "F", "G", "H", "I"] {
        network.create_object(object, 0);
    }
    for (source, dest) in &[
        ("X", "A"),
        ("Y", "B"),
        ("A", "D"),
        ("A", "E"),
        ("B", "C"),
        ("B", "E"),
        ("C", "I"),
        ("D", "G"),
        ("D", "H"),
        ("D", "F"),
        ("E", "H"),
        ("F", "I"),
        ("G", "H"),
        ("H", "I"),
        ("I", "E"),
    ] {
        network.create_reference(source, dest);
    }

    network
}

fn exercise_7_5_2() {
    println!("Exercise 7.5.2:");

    let mut network = network_two();
    network.remove_reference("A", "D");
    println!("{:?}", network);
}
