use grammars::Grammar;
use std::collections::HashSet;

fn main() {
    exercise_4_4_1();
    exercise_4_4_2();
    exercise_4_4_3();
    exercise_4_4_4();
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
