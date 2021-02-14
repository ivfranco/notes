#![allow(non_snake_case)]

use functional_dependency::{chase::*, *};
use rand::{thread_rng, Rng};
use std::{convert::TryInto, iter::from_fn, ops::Not};

fn main() {
    exercise_3_2_1();
    exercise_3_2_2();
    exercise_3_2_10();
    exercise_3_3_1();
    exercise_3_4_1();
    exercise_3_5_1();
    exercise_3_5_2();
    exercise_3_5_3();
    exercise_3_5_4();
    exercise_3_5_5();
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

fn exercise_3_4_1() {
    println!("\nexercise 3.4.1");

    let mut reg = NameRegister::new();
    let A = reg.register("A");
    let B = reg.register("B");
    let C = reg.register("C");
    let D = reg.register("D");
    let E = reg.register("E");

    let decomposition = [attrs(&[A, B, C]), attrs(&[B, C, D]), attrs(&[A, C, E])];
    let rel = Relation::from_decomposition(&decomposition, reg.cnt);

    let print_fixpoint = move |FDs: &[&str]| {
        let mut origin = rel.clone();
        let FDs = parse_dependencies(&reg, FDs);
        origin.fixpoint(&FDs);
        if origin.contains_origin() {
            println!("Lossless");
        } else {
            print!("{}", origin);
        }

        if let Some(fd) = not_preserved(&decomposition, &FDs) {
            println!("{} is not preserved", fd.with_names(&reg));
        }

        println!();
    };

    print_fixpoint(&["B -> E", "C, E -> A"]);
    print_fixpoint(&["A, C -> E", "B, C -> D"]);
    print_fixpoint(&["A -> D", "D -> E", "B -> D"]);
    print_fixpoint(&["A -> D", "C, D -> E", "E -> D"]);
}

fn keys(attrs: &[&str], FDs: &[&str]) {
    let mut reg = NameRegister::new();
    for attr in attrs {
        reg.register(attr);
    }

    let FDs = parse_dependencies(&reg, FDs);

    assert!(is_minimal_basis(&FDs));

    for set in all_subsets_of(&*reg.attrs()) {
        if reg.categorize(&set, &FDs) == Category::Key {
            println!("{}", set.with_names(&reg));
        }
    }
}

fn exercise_3_5_1() {
    println!("\nexercise 3.5.1");

    keys(&["A", "B", "C", "D"], &["A, B -> C", "C -> D", "D -> A"]);
    println!();
    keys(&["A", "B", "C", "D"], &["B -> C", "B -> D"]);
    println!();
    keys(
        &["A", "B", "C", "D"],
        &["A, B -> C", "B, C -> D", "C, D -> A", "A, D -> B"],
    );
    println!();
    keys(
        &["A", "B", "C", "D"],
        &["A -> B", "B -> C", "C -> D", "D -> A"],
    );
    println!();
    keys(
        &["A", "B", "C", "D", "E"],
        &["A, B -> C", "D, E -> C", "B -> D"],
    );
    println!();
    keys(
        &["A", "B", "C", "D", "E"],
        &["A, B -> C", "C -> D", "D -> B", "D -> E"],
    );
}

fn key_3nf_violations(attrs: &[&str], FDs: &[&str]) {
    let mut reg = NameRegister::new();
    for attr in attrs {
        reg.register(attr);
    }
    let FDs = parse_dependencies(&reg, FDs);

    let keys = reg.attrs().keys(&FDs);
    for key in &keys {
        println!("{}: Key", key.with_names(&reg));
    }

    assert!(is_minimal_basis(&FDs));

    let mut decomposition: Vec<_> = FDs.iter().map(|fd| &fd.source | &fd.target).collect();

    let contains_superkey = decomposition
        .iter()
        .any(|rel| keys.iter().any(|key| rel.is_superset(key)));

    // add a key to the decomposition if there's no superkey in it
    if !contains_superkey {
        decomposition.extend(keys.into_iter().next());
    }

    // remove subsets in decomposition
    decomposition = decomposition
        .iter()
        .filter(|&rel_0| {
            decomposition
                .iter()
                .any(|rel_1| rel_0.len() < rel_1.len() && rel_0.is_subset(rel_1))
                .not()
        })
        .cloned()
        .collect();

    for rel in &decomposition {
        let label = if is_bcnf_violation(rel, &project_to(rel, &FDs)) {
            "BCNF violation"
        } else {
            "BCNF abiding"
        };

        println!("{}: {}", rel.with_names(&reg), label);
    }

    let mut chase = Relation::from_decomposition(&decomposition, reg.cnt);
    chase.fixpoint(&FDs);
    // 3NF decomposition has lossless join and dependency preservation
    assert!(chase.contains_origin());
    assert!(not_preserved(&decomposition, &FDs).is_none());
}

fn exercise_3_5_2() {
    println!("\nexercise 3.5.2");
    key_3nf_violations(
        &["C", "T", "H", "R", "S", "G"],
        &["C -> T", "H, R -> C", "H, T -> R", "H, S -> R", "C, S -> G"],
    );
}

fn exercise_3_5_3() {
    println!("\nexercise 3.5.3");
    key_3nf_violations(
        &["B", "O", "I", "S", "Q", "D"],
        &["S -> D", "I -> B", "I, S -> Q", "B -> O"],
    );
}

fn exercise_3_5_4() {
    println!("\nexercise 3.5.4");
    let mut reg = NameRegister::new();
    let A = reg.register("A");
    let B = reg.register("B");
    let C = reg.register("C");
    let D = reg.register("D");
    let E = reg.register("E");

    let FDs = parse_dependencies(&reg, &["A, B -> C", "C -> B", "A -> D"]);

    let mut chase = Relation::from_decomposition(
        &[attrs(&[A, B, C]), attrs(&[A, D]), attrs(&[A, B, E])],
        reg.cnt,
    );

    chase.fixpoint(&FDs);

    assert!(chase.contains_origin());
}

fn search_3nf_violation<'a>(rel: &Attrs, FDs: &'a [FD]) -> Option<&'a FD> {
    let keys = rel.keys(FDs);
    FDs.iter().find(|fd| {
        keys.iter().all(|key| fd.source.is_superset(key).not())
            && fd
                .target
                .iter()
                .any(|v| keys.iter().all(|key| key.contains(v).not()))
    })
}

fn mock_3nf_decomposition(rel: &Attrs, FDs: &[FD]) -> Vec<Attrs> {
    let mut decomposition = vec![];
    let mut stack = vec![(rel.clone(), FDs.to_vec())];

    while let Some((rel, FDs)) = stack.pop() {
        if let Some(fd) = search_3nf_violation(&rel, &FDs) {
            let rel_0 = closure_of(&fd.source, &FDs);
            let FDs_0 = project_to(&rel_0, &FDs);
            let rel_1 = &fd.source | &(&rel - &rel_0);
            let FDs_1 = project_to(&rel_1, &FDs);

            stack.push((rel_0, FDs_0));
            stack.push((rel_1, FDs_1));
        } else {
            decomposition.push(rel);
        }
    }

    decomposition
}

fn exercise_3_5_5() {
    // generate a random case then test for dependency preservation
    const ATTRS: u32 = 3;

    fn random_FD<R: Rng>(rng: &mut R) -> FD {
        let mut source = vec![];
        let mut target = vec![];
        for i in 0..ATTRS {
            if rng.gen_bool(0.5) {
                source.push(i);
            }
            if rng.gen_bool(0.5) {
                target.push(i);
            }
        }

        if source.is_empty() {
            source.push(rng.gen_range(0..ATTRS));
        }

        FD::new(source.into(), target.into())
    }

    let mut reg = NameRegister::new();
    (0..ATTRS)
        .map(|v| ('A' as u32 + v).try_into().unwrap())
        .for_each(|v: char| {
            reg.register(&v.to_string());
        });

    let mut rng = thread_rng();

    loop {
        let FDs: Vec<_> = minify(
            &from_fn(|| Some(random_FD(&mut rng)))
                .take(4) // maybe enough, maybe not
                .collect::<Vec<_>>(),
        );
        let decomposition = mock_3nf_decomposition(&reg.attrs(), &FDs);
        if not_preserved(&decomposition, &FDs).is_some() {
            for rel in decomposition {
                println!("{}", rel.with_names(&reg));
            }
            for fd in FDs {
                println!("{}", fd.with_names(&reg));
            }
            break;
        }
    }
}
