use grammars::slr::tokenize;
use grammars::Grammar;
use std::collections::HashSet;

fn main() {
    exercise_4_4_1();
    exercise_4_4_2();
    exercise_4_4_3();
    exercise_4_4_4();
    exercise_4_6_2();
    exercise_4_6_5();
    exercise_4_6_6();
    exercise_4_6_7();
    exercise_4_6_9();
    exercise_4_7_1();
    exercise_4_8_1();
}

fn print_parse_table(grammar: Grammar<String>) {
    println!("{:?}", grammar);
    if grammar.is_ll1() {
        let table = grammar.to_ll1();
        println!("{}", table.to_string(&grammar.rev_map()));
    } else {
        println!("Not LL(1)");
        let rev_map = grammar.rev_map();
        for (n, ps) in &grammar.prod_map {
            let nonterm = &rev_map[n];
            println!("{} follow: {:?}", nonterm, grammar.follow(nonterm));
            for p in ps {
                println!(
                    "{}: first {:?}",
                    p.to_string(&rev_map),
                    grammar.string_first(&p.body)
                );
            }
        }
        println!("");
    }
}

fn exercise_4_4_1() {
    println!("Exercise 4.4.1:");

    let mut grammar = Grammar::parse("S", &["S -> 0 S'", "S' -> 0 S' 1", "S' -> 1"]);
    grammar.eliminate_left_recursions();
    print_parse_table(grammar);

    let grammar = Grammar::parse("S", &["S -> + S S", "S -> * S S", "S -> a"]);
    print_parse_table(grammar);

    let mut grammar = Grammar::parse("S", &["S -> S ( S ) S", "S -> ε"]);
    grammar.eliminate_immediate_left_recursions();
    print_parse_table(grammar);

    let grammar = Grammar::parse(
        "S",
        &["S -> S + S", "S -> S S", "S -> ( S )", "S -> S *", "S -> a"],
    );
    print_parse_table(grammar);

    let mut grammar = Grammar::parse("S", &["S -> ( L )", "S -> a", "L -> L , S", "L -> S"]);
    grammar.eliminate_immediate_left_recursions();
    print_parse_table(grammar);

    let mut grammar = Grammar::parse(
        "bexpr",
        &[
            "bexpr -> bexpr or bterm",
            "bexpr -> bterm",
            "bterm -> bterm and bfactor",
            "bterm -> bfactor",
            "bfactor -> not bfactor",
            "bfactor -> ( bexpr )",
            "bfactor -> true",
            "bfactor -> false",
        ],
    );
    grammar.eliminate_immediate_left_recursions();
    print_parse_table(grammar);
}

fn exercise_4_4_2() {
    println!("Exercise 4.4.2:");

    let grammar = Grammar::parse(
        "S",
        &[
            "S -> a S'",
            "S' -> a S' OP S'",
            "S' -> ε",
            "OP -> *",
            "OP -> +",
        ],
    );
    print_parse_table(grammar);
}

fn print_first_and_follow(grammar: &Grammar<String>) {
    for nonterm in grammar.term_map.keys() {
        let first: HashSet<String> = grammar
            .first(nonterm)
            .iter()
            .map(|opt| opt.clone().unwrap_or_else(|| "ε".to_owned()))
            .collect();

        let follow: HashSet<String> = grammar
            .follow(nonterm)
            .iter()
            .map(|opt| opt.clone().unwrap_or_else(|| "$".to_owned()))
            .collect();

        println!("first of {}: {:?}", nonterm, first);
        println!("follow of {}: {:?}", nonterm, follow);
    }
    println!("");
}

fn exercise_4_4_3() {
    println!("Exercise 4.4.3:");

    let grammar = Grammar::parse("S", &["S -> S S +", "S -> S S -", "S -> a"]);
    print_first_and_follow(&grammar);
}

fn exercise_4_4_4() {
    println!("Exercise 4.4.4:");

    let grammar = Grammar::parse("S", &["S -> 0 S 1", "S -> 0 1"]);
    print_first_and_follow(&grammar);

    let grammar = Grammar::parse("S", &["S -> + S S", "S -> * S S", "S -> a"]);
    print_first_and_follow(&grammar);

    let grammar = Grammar::parse("S", &["S -> S ( S ) S", "S -> ε"]);
    print_first_and_follow(&grammar);

    let grammar = Grammar::parse(
        "S",
        &["S -> S + S", "S -> S S", "S -> ( S )", "S -> S *", "S -> a"],
    );
    print_first_and_follow(&grammar);

    let grammar = Grammar::parse("S", &["S -> ( L )", "S -> a", "L -> L , S", "L -> S"]);
    print_first_and_follow(&grammar);

    let grammar = Grammar::parse(
        "bexpr",
        &[
            "bexpr -> bexpr or bterm",
            "bexpr -> bterm",
            "bterm -> bterm and bfactor",
            "bterm -> bfactor",
            "bfactor -> not bfactor",
            "bfactor -> ( bexpr )",
            "bfactor -> true",
            "bfactor -> false",
        ],
    );
    print_first_and_follow(&grammar);
}

#[allow(dead_code)]
fn report_grammar(mut grammar: Grammar<String>) {
    let canonical = grammar.canonical();
    let slr = canonical.slr();

    println!("{:?}", slr);
    println!("Is SLR: {}", slr.is_slr());
}

fn exercise_4_6_2() {
    println!("Exercise 4.6.2:");
    let mut grammar = Grammar::parse("S", &["S -> S S +", "S -> S S *", "S -> a"]);
    let canonical = grammar.canonical();
    let slr = canonical.slr();

    println!("{:?}", slr);
    println!("Is SLR: {}", slr.is_slr());

    println!("Exercise 4.6.3:");
    assert!(slr.parse(&tokenize("a a * a +")));
}

fn exercise_4_6_5() {
    println!("Exercise 4.6.5:");
    let mut grammar = Grammar::parse("S", &["S -> A a A b", "S -> B b B a", "A -> ε", "B -> ε"]);

    assert!(grammar.is_ll1());
    let canonical = grammar.canonical();
    let slr = canonical.slr();
    assert!(!slr.is_slr());
    println!("{:?}", slr);
}

fn exercise_4_6_6() {
    println!("Exercise 4.6.6:");
    let mut grammar = Grammar::parse("S", &["S -> S A", "S -> A", "A -> a"]);

    assert!(!grammar.is_ll1());
    let canonical = grammar.canonical();
    let slr = canonical.slr();
    assert!(slr.is_slr());
}

fn exercise_4_6_7() {
    println!("Exercise 4.6.7:");
    let mut grammar = Grammar::parse(
        "S",
        &[
            "S -> A1 b1",
            "S -> A2 b2",
            "A1 -> a2 A1",
            "A1 -> a2",
            "A2 -> a1 A2",
            "A2 -> a1",
        ],
    );

    let canonical = grammar.canonical();
    let slr = canonical.slr();
    println!("{:?}", slr);
}

fn exercise_4_6_9() {
    println!("Exercise 4.6.9:");

    let mut grammar = Grammar::parse("S", &["S -> A S", "S -> b", "A -> S A", "A -> a"]);

    let canonical = grammar.canonical();
    let slr = canonical.slr();
    println!("{:?}", slr);
    if slr.parse(&tokenize("a b a b")) {
        println!("parse succeed");
    } else {
        println!("parse failed");
    }
}

fn exercise_4_7_1() {
    println!("Exercise 4.7.1:");

    let mut grammar = Grammar::parse("S", &["S -> S S +", "S -> S S *", "S -> a"]);

    println!("{:?}", grammar.canonical_lr());
}

fn exercise_4_8_1() {
    println!("Exercise 4.8.1:");

    println!("ambiguous:");
    for i in 2..10 {
        let mut rules = vec![];
        for j in 0..i {
            rules.push(format!("E -> E +{} E", j));
        }
        rules.push("E -> ( E )".to_owned());
        rules.push("E -> id".to_owned());

        let mut grammar =
            Grammar::parse("E", &rules.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let canonical = grammar.canonical();
        println!("{}, {}", i, canonical.size());
    }

    println!("unambiguous:");
    for i in 2..10 {
        let mut rules = vec![];
        for j in 0..i {
            rules.push(format!("E{} -> E{} +{} E{}", j, j, j, j + 1));
            rules.push(format!("E{} -> E{}", j, j + 1));
        }
        rules.push(format!("E{} -> ( E0 )", i));
        rules.push(format!("E{} -> id", i));

        let mut grammar =
            Grammar::parse("E0", &rules.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let canonical = grammar.canonical();
        println!("{}, {}", i, canonical.size());
    }
}
