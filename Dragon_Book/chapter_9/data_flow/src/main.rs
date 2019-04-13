use data_flow::available_expr::available_expressions;
use data_flow::live_var::live_variables;
use data_flow::reaching_def::reaching_definitions;
use data_flow::{Block, Program};

fn main() {
    exercise_9_2_1();
    exercise_9_2_2();
    exercise_9_2_3();
}

fn figure_9_10() -> Program {
    let blocks = vec![
        Block::parse(0, ""), // ENTRY
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
        Block::parse(10, ""), // EXIT
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
