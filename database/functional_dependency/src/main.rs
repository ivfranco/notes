#![allow(non_snake_case)]

use functional_dependency::*;

fn main() {
    exercise_3_2_1();
    exercise_3_2_2();
    exercise_3_2_10();
    exercise_3_3_1();
}

fn dependencies_and_key(attrs: &[&str], FDs: &[&str]) {
    dependencies_and_key_selected(attrs, attrs, FDs);
}

fn dependencies_and_key_selected(selected: &[&str], attrs: &[&str], FDs: &[&str]) {
    let mut reg = NameRegister::new();
    for attr in attrs {
        reg.register(attr);
    }

    let FDs = parse_dependencies(&reg, FDs);
    let selected: Attrs = selected.iter().map(|v| reg.resolve(v).unwrap()).collect();

    for set in all_subsets_of(&selected) {
        let closure = closure_of(&set, &FDs);
        let fd = FD::new(set, &closure & &selected);
        if !fd.is_deformed() {
            println!("{}", fd.with_names(&reg));
        }
    }

    if selected.len() == attrs.len() {
        for set in all_subsets_of(&selected).filter(|set| !set.is_empty()) {
            println!("{}: {}", set.with_names(&reg), reg.categorize(&set, &FDs));
        }
    }
}

fn exercise_3_2_1() {
    println!("\nexercise 3.2.1");
    dependencies_and_key(&["A", "B", "C", "D"], &["A, B -> C", "C -> D", "D -> A"]);
}

fn exercise_3_2_2() {
    println!("\nexercise 3.2.2");
    dependencies_and_key(&["A", "B", "C", "D"], &["A -> B", "B -> C", "B -> D"]);
    println!();
    dependencies_and_key(
        &["A", "B", "C", "D"],
        &["A, B -> C", "B, C -> D", "C, D -> A", "A, D -> B"],
    );
    println!();
    dependencies_and_key(
        &["A", "B", "C", "D"],
        &["A -> B", "B -> C", "C -> D", "D -> A"],
    );
}

fn exercise_3_2_10() {
    println!("\nexercise 3.2.10");

    dependencies_and_key_selected(
        &["A", "B", "C"],
        &["A", "B", "C", "D", "E"],
        &["A, B -> D, E", "C -> E", "D -> C", "E -> A"],
    );

    println!();

    dependencies_and_key_selected(
        &["A", "B", "C"],
        &["A", "B", "C", "D", "E"],
        &["A -> D", "B, D -> E", "A, C -> E", "D, E -> B"],
    );

    println!();

    dependencies_and_key_selected(
        &["A", "B", "C"],
        &["A", "B", "C", "D", "E"],
        &["A, B -> D", "A, C -> E", "B, C -> D", "D -> A", "E -> B"],
    );

    println!();

    dependencies_and_key_selected(
        &["A", "B", "C"],
        &["A", "B", "C", "D", "E"],
        &["A -> B", "B -> C", "C -> D", "D -> E", "E -> A"],
    );
}

fn violations_and_decomposition(attrs: &[&str], FDs: &[&str]) {
    let mut reg = NameRegister::new();
    for attr in attrs {
        reg.register(attr);
    }
    let FDs = parse_dependencies(&reg, FDs);

    for fd in all_violations(&reg.attrs(), &FDs) {
        println!("{}", fd.with_names(&reg));
    }

    for rel in bcnf_decomposition(&reg.attrs(), &FDs) {
        println!("{}", rel.with_names(&reg));
    }
}

fn exercise_3_3_1() {
    println!("\nexercise 3.3.1");
    violations_and_decomposition(&["A", "B", "C", "D"], &["A, B -> C", "C -> D", "D -> A"]);
    println!();
    violations_and_decomposition(&["A", "B", "C", "D"], &["B -> C", "B -> D"]);
    println!();
    violations_and_decomposition(
        &["A", "B", "C", "D"],
        &["A, B -> C", "B, C -> D", "C, D -> A", "A, D -> B"],
    );
    println!();
    violations_and_decomposition(
        &["A", "B", "C", "D"],
        &["A -> B", "B -> C", "C -> D", "D -> A"],
    );
    println!();
    violations_and_decomposition(
        &["A", "B", "C", "D", "E"],
        &["A, B -> C", "D, E -> C", "B -> D"],
    );
    println!();
    violations_and_decomposition(
        &["A", "B", "C", "D", "E"],
        &["A, B -> C", "C -> D", "D -> B", "D -> E"],
    );
    println!();
}
