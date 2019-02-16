use nfa::NFA;

fn main() {
    exercise_3_6_3();
    exercise_3_6_4();
    exercise_3_6_5();
    exercise_3_7_1();
    exercise_3_7_2();
}

fn first_machine() -> NFA {
    let mut nfa = NFA::new(4, &[3]);
    nfa.install_transition(0, Some('a'), vec![0, 1]);
    nfa.install_transition(0, Some('b'), vec![0]);
    nfa.install_transition(1, Some('a'), vec![1, 2]);
    nfa.install_transition(1, Some('b'), vec![1]);
    nfa.install_transition(2, Some('a'), vec![2]);
    nfa.install_transition(2, Some('b'), vec![2, 3]);
    nfa.install_transition(2, None, vec![0]);

    nfa
}

fn second_machine() -> NFA {
    let mut nfa = NFA::new(4, &[3]);
    nfa.install_transition(0, Some('a'), vec![1]);
    nfa.install_transition(0, None, vec![3]);
    nfa.install_transition(1, Some('b'), vec![2]);
    nfa.install_transition(1, None, vec![0]);
    nfa.install_transition(2, Some('b'), vec![3]);
    nfa.install_transition(2, None, vec![1]);
    nfa.install_transition(3, Some('a'), vec![0]);
    nfa.install_transition(3, None, vec![2]);

    nfa
}

fn third_machine() -> NFA {
    let mut nfa = NFA::new(5, &[2, 4]);
    nfa.install_transition(0, None, vec![1, 3]);
    nfa.install_transition(1, Some('a'), vec![2]);
    nfa.install_transition(2, Some('a'), vec![2]);
    nfa.install_transition(3, Some('b'), vec![4]);
    nfa.install_transition(4, Some('b'), vec![4]);

    nfa
}

fn exercise_3_6_3() {
    println!("\nExercise 3.6.3:");

    let nfa = first_machine();
    let input = "aabb";

    println!("All paths:");
    for path in nfa.all_paths(input) {
        println!("{:?}", path);
    }
    if nfa.accept(input) {
        println!("{} is accepted", input);
    } else {
        println!("{} is rejected", input);
    }
}

fn exercise_3_6_4() {
    println!("\nExercise 3.6.4:");

    let nfa = second_machine();
    let input = "aabb";

    println!("All paths:");
    for path in nfa.all_paths(input) {
        println!("{:?}", path);
    }
    if nfa.accept(input) {
        println!("{} is accepted", input);
    } else {
        println!("{} is rejected", input);
    }
}

fn exercise_3_6_5() {
    println!("\nExercise 3.6.5:");

    println!("{:?}", first_machine());
    println!("{:?}", second_machine());
    println!("{:?}", third_machine());
}

fn exercise_3_7_1() {
    println!("\nExercise 3.7.1:");

    for nfa in &[third_machine(), first_machine(), second_machine()] {
        let (mapping, dfa) = nfa.to_dfa("ab");
        for (set, state) in mapping {
            println!("set {:?} in nfa corresponds to state {} in dfa", set, state);
        }
        println!("{:?}", dfa);
    }
}

fn exercise_3_7_2() {
    println!("\nExercise 3.7.2:");

    let input = "aabb";

    let mut nfa = first_machine();
    let mut states = nfa.init();
    println!("{:?}", states);
    for c in input.chars() {
        states = nfa.next(&states, c);
        println!("{:?}", states);
    }

    println!("");

    nfa = second_machine();
    let mut states = nfa.init();
    println!("{:?}", states);
    for c in input.chars() {
        states = nfa.next(&states, c);
        println!("{:?}", states);
    }
}
