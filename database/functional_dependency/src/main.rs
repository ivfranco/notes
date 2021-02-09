#![allow(non_snake_case)]

use functional_dependency::*;
use itertools::Itertools;

fn main() {
    exercise_2_2_1();
    exercise_2_2_2();
    exercise_2_2_10();
}

fn all_subsets_of(attrs: &[u32]) -> impl Iterator<Item = Vec<u32>> + '_ {
    (0..=attrs.len()).flat_map(move |k| attrs.iter().copied().combinations(k))
}

fn key_listing(attrs: &[&str], FDs: &[&str]) {
    selected_key_listing(attrs, attrs, FDs);
}

fn selected_key_listing(selected: &[&str], attrs: &[&str], FDs: &[&str]) {
    let mut reg = NameRegister::new();
    for attr in attrs {
        reg.register(attr);
    }

    let FDs: Vec<_> = FDs.iter().map(|fd| reg.parse(fd).unwrap()).collect();
    let selected: Vec<_> = selected.iter().map(|v| reg.resolve(v).unwrap()).collect();

    for set in all_subsets_of(&selected) {
        let mut closure = closure_of(&set, &FDs);
        closure.retain(|v| selected.contains(v));

        let fd = FD::new(&set, closure);
        if !fd.is_deformed() {
            println!("{}", fd.with_names(&reg));
        }
    }

    if selected.len() == attrs.len() {
        for set in all_subsets_of(&selected).filter(|set| !set.is_empty()) {
            println!("{}: {}", reg.with_names(&set), reg.categorize(&set, &FDs));
        }
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

fn exercise_2_2_10() {
    println!("\nexercise 2.2.10");

    selected_key_listing(
        &["A", "B", "C"],
        &["A", "B", "C", "D", "E"],
        &["A, B -> D, E", "C -> E", "D -> C", "E -> A"],
    );

    println!();

    selected_key_listing(
        &["A", "B", "C"],
        &["A", "B", "C", "D", "E"],
        &["A -> D", "B, D -> E", "A, C -> E", "D, E -> B"],
    );

    println!();

    selected_key_listing(
        &["A", "B", "C"],
        &["A", "B", "C", "D", "E"],
        &["A, B -> D", "A, C -> E", "B, C -> D", "D -> A", "E -> B"],
    );

    println!();

    selected_key_listing(
        &["A", "B", "C"],
        &["A", "B", "C", "D", "E"],
        &["A -> B", "B -> C", "C -> D", "D -> E", "E -> A"],
    );
}
