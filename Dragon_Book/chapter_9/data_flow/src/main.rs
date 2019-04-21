use data_flow::available_expr::available_expressions;
use data_flow::lazy_code_motion::{
    anticipates, availables, earliests, latests, postponables, used, where_to_compute,
    where_to_use, PairSlice,
};
use data_flow::live_var::live_variables;
use data_flow::reaching_def::reaching_definitions;
use data_flow::{Block, Expr, Program, Stmt};
use std::fmt::Debug;

fn main() {
    exercise_9_2_1();
    exercise_9_2_2();
    exercise_9_2_3();
    exercise_9_5_1();
    exercise_9_5_2();
}

fn figure_9_10() -> Program {
    let blocks = vec![
        Block::entry(), // ENTRY
        Block::parse(
            1,
            "a = 1
b = 2",
        ),
        Block::parse(
            3,
            "c = a+b
d = c-a",
        ),
        Block::parse(5, "d = b+d"),
        Block::parse(
            6,
            "d = a+b
e = e+1",
        ),
        Block::parse(
            8,
            "b = a+b
e = c-a",
        ),
        Block::parse(
            10,
            "a = b*d
b = a-d",
        ),
        Block::exit(), // EXIT
    ];

    let edges = &[
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (3, 5),
        (4, 3),
        (5, 2),
        (5, 6),
        (6, 7),
    ];

    Program::new(blocks, edges)
}

fn exercise_9_2_1() {
    println!("Exercise 9.2.1:");

    let program = figure_9_10();
    println!("{:?}", reaching_definitions(&program));
}

fn exercise_9_2_2() {
    println!("Exercise 9.2.2:");

    let program = figure_9_10();
    println!("{:?}", available_expressions(&program));
}

fn exercise_9_2_3() {
    println!("Exercise 9.2.3:");

    let program = figure_9_10();
    println!("{:?}", live_variables(&program));
}

fn figure_9_37() -> Program {
    Program::with_entry_exit(
        vec![
            Block::parse(0, "z = x + y"),
            Block::parse(1, "x = 1"),
            Block::parse(2, "z = x + y"),
            Block::parse(3, "z = x + y"),
        ],
        &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)],
    )
}

fn format_pairs<T: Debug>(tag: &str, pairs: &PairSlice<T>) {
    for (i, (in_value, out_value)) in pairs.iter().enumerate() {
        let name = if i == 0 {
            "ENTRY".into()
        } else if i == pairs.len() - 1 {
            "EXIT".into()
        } else {
            format!("B{}", i)
        };

        println!("{} of {}:", tag, name);
        println!("    IN:  {:?}", in_value);
        println!("    OUT: {:?}", out_value);
    }
}

fn format_sets<T: Debug>(tag: &str, sets: &[T]) {
    for (i, set) in sets.iter().enumerate() {
        let name = if i == 0 {
            "ENTRY".into()
        } else if i == sets.len() - 1 {
            "EXIT".into()
        } else {
            format!("B{}", i)
        };

        println!("{} of {}: {:?}", tag, name, set);
    }
}

fn report_lazy_code_motion<'a>(program: &'a Program, exprs: &[Expr<'a>]) {
    let anticipates = anticipates(program);
    format_pairs("Anticipated", &anticipates);
    let availables = availables(program, &anticipates);
    format_pairs("Available", &availables);
    let earliests = earliests(&anticipates, &availables);
    format_sets("Earliest", &earliests);
    let postponables = postponables(program, &earliests);
    format_pairs("Postponable", &postponables);
    let latests = latests(program, &earliests, &postponables);
    format_sets("Latest", &latests);
    let used = used(program, &latests);
    format_pairs("Used", &used);

    for expr in exprs {
        println!(
            "{:?} should be computed at: {:?}",
            expr,
            where_to_compute(expr, &latests, &used)
        );
        println!(
            "temporary of {:?} should be used at: {:?}",
            expr,
            where_to_use(expr, program, &latests, &used)
        );
    }
}

fn exercise_9_5_1() {
    println!("Exercise 9.5.1:");
    let program = figure_9_37();
    let stmt = Stmt::parse("z = x + y");
    let expr = stmt.as_expr().unwrap();

    report_lazy_code_motion(&program, &[expr]);
}

fn exercise_9_5_2() {
    println!("Exercise 9.5.2:");
    let program = figure_9_10();
    let stmts = vec![
        Stmt::parse("z = a + b"),
        Stmt::parse("z = c - a"),
        Stmt::parse("z = b * d"),
    ];
    let exprs: Vec<_> = stmts.iter().filter_map(Stmt::as_expr).collect();

    report_lazy_code_motion(&program, &exprs)
}
