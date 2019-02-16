use simple_regex::nfa::NFA;

fn main() {
    exercise_3_7_3();
}

fn exercise_3_7_3() {
    println!("Exercise 3.7.3:");

    println!("{:?}", NFA::parse("(a|b)*").to_dfa("ab").1);
    println!("{:?}", NFA::parse("(a*|b*)*").to_dfa("ab").1);
    println!("{:?}", NFA::parse("((ε|a)b*)*").to_dfa("ab").1);
    println!("{:?}", NFA::parse("(a|b)*abb(a|b)*").to_dfa("ab").1);
}
