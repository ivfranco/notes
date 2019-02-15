use pest::Parser;

#[derive(Parser)]
#[grammar = "./whitespace.pest"]
struct WSLexer;

pub fn translate(input: &str) -> String {
    WSLexer::parse(Rule::tokens, input)
        .expect("Grammar definition error")
        .next()
        .unwrap()
        .into_inner()
        .map(|token| match token.as_rule() {
            Rule::spaces => " ".to_owned(),
            Rule::other => token.as_str().to_owned(),
            _ => unreachable!(),
        })
        .fold(String::new(), |mut acc, token| {
            acc.push_str(token.as_str());
            acc
        })
}

#[test]
fn translate_test() {
    let input = "hello    world     !";
    assert_eq!(translate(input), "hello world !");
}
