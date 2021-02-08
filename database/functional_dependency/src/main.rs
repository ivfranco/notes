#![allow(non_snake_case)]

use functional_dependency::*;
use itertools::Itertools;

fn main() {
    exercise_2_2_1();
    exercise_2_2_2();
}

fn all_subsets_of(attrs: &[u32]) -> impl Iterator<Item = Vec<u32>> + '_ {
    (0..=attrs.len()).flat_map(move |k| attrs.iter().copied().combinations(k))
}

fn key_listing(attrs: &[&str], FDs: &[&str]) {
    let mut reg = NameRegister::new();
    for attr in attrs {
        reg.register(attr);
    }

    let FDs: Vec<_> = FDs.iter().map(|fd| reg.parse(fd).unwrap()).collect();

    for set in all_subsets_of(&reg.sorted_attrs()) {
        let closure = closure_of(&set, &FDs);

        let fd = FD::new(&set, closure);
        if !fd.is_deformed() {
            println!("{}", fd.with_names(&reg));
        }
    }

    for set in all_subsets_of(&reg.sorted_attrs()).filter(|set| !set.is_empty()) {
        println!("{}: {}", reg.with_names(&set), reg.categorize(&set, &FDs));
    }
}

fn exercise_2_2_1() {
    println!("\nexercise 2.2.1");
    key_listing(&["A", "B", "C", "D"], &["A, B -> C", "C -> D", "D -> A"]);
}

fn exercise_2_2_2() {
    println!("\nexercise 2.2.2");
    key_listing(&["A", "B", "C", "D"], &["A -> B", "B -> C", "B -> D"]);
    println!();
    key_listing(
        &["A", "B", "C", "D"],
        &["A, B -> C", "B, C -> D", "C, D -> A", "A, D -> B"],
    );
    println!();
    key_listing(
        &["A", "B", "C", "D"],
        &["A -> B", "B -> C", "C -> D", "D -> A"],
    );
}
