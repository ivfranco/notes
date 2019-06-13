use minimax::tic_tac_toe::TicTacToe;

fn main() {
    exercise_5_9();
}

fn exercise_5_9() {
    println!("5.9");

    for expr in &[
        "xo.......",
        "x.o......",
        "x...o....",
        "x....o...",
        "x.......o",
        "ox.......",
        ".x.o.....",
        ".x..o....",
        ".x....o..",
        ".x.....o.",
        "o...x....",
        ".o..x....",
    ] {
        let state = TicTacToe::parse(expr);
        println!("{:?}: {}", state, state.evaluate());
    }
}
