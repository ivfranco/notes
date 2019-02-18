use simple_regex::dfa::DFA;
use simple_regex::nfa::NFA;

fn main() {
    exercise_3_7_3();
    exercise_3_8_1();
    exercise_3_8_2();
    exercise_3_9_2();
    exercise_3_9_4();
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

fn exercise_3_9_2() {
    println!("Exercise 3.9.2:");

    println!("{:?}", DFA::parse("(a|b)*", "ab"));
    println!("{:?}", DFA::parse("(a*|b*)*", "ab"));
    println!("{:?}", DFA::parse("((ε|a)b*)*", "ab"));
    println!("{:?}", DFA::parse("(a|b)*abb(a|b)*", "ab"));
}

fn exercise_3_9_4() {
    println!("Exercise 3.9.4:");

    println!("{:?}", DFA::parse("(a|b)*a(a|b)", "ab").minimize("ab"));
    println!("{:?}", DFA::parse("(a|b)*a(a|b)(a|b)", "ab").minimize("ab"));
    println!(
        "{:?}",
        DFA::parse("(a|b)*a(a|b)(a|b)(a|b)", "ab").minimize("ab")
    );
}
