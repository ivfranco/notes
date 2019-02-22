use grammars::Grammar;

fn main() {
    let grammar = Grammar::parse("S", &["S -> S S +", "S -> S S -", "S -> a"]);

    println!("{:?}", grammar);
}
