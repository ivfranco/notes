use simple_regex::nfa::NFA;

fn main() {
    exercise_3_7_3();
    exercise_3_8_1();
    exercise_3_8_2();
}

fn exercise_3_7_3() {
    println!("Exercise 3.7.3:");

    println!("{:?}", NFA::parse("(a|b)*").to_dfa("ab").1);
    println!("{:?}", NFA::parse("(a*|b*)*").to_dfa("ab").1);
    println!("{:?}", NFA::parse("((ε|a)b*)*").to_dfa("ab").1);
    println!("{:?}", NFA::parse("(a|b)*abb(a|b)*").to_dfa("ab").1);
}

const DIGIT: &str = "0|1";

fn exercise_3_8_1() {
    println!("Exercise 3.8.1:");

    let nfa = NFA::multi_parse(&[("if", "IF"), ("(i|f)(i|f)*", "ID")]);
    println!("{:?}", nfa);
    println!("{:?}", nfa.to_dfa("abif").1);
}

fn exercise_3_8_2() {
    println!("Exercise 3.8.2:");
    const LETTER: &str = "(w|h|i|l|e|n)";

    let nfa = NFA::multi_parse(&[
        ("while", "WHILE"),
        ("when", "WHEN"),
        (&format!("{}({}|{})*", LETTER, LETTER, DIGIT), "ID"),
    ]);

    println!("{:?}", nfa);
    println!("{:?}", nfa.to_dfa("whilen01").1);
}
