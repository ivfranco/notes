use grammars::Grammar;

fn main() {
    let mut grammar = Grammar::parse("S", &["S -> S S +", "S -> S S -", "S -> a"]);
    grammar.update_first_and_follow();

    println!("{:?}", grammar);
}
