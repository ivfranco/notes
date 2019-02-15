use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./pig_latin.pest"]
struct PigLexer;

fn latin(pair: Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::token => {
            let inner = pair.into_inner().next().unwrap();
            latin(inner)
        }
        Rule::vowel_word => {
            let mut word = pair.as_str().to_owned();
            word.push_str("ay");
            word
        }
        Rule::conso_word => {
            let mut word = pair.as_str().as_bytes().to_owned();
            let consonant = word.remove(0);
            word.push(consonant);
            word.extend_from_slice(b"ay");
            String::from_utf8(word).unwrap()
        }
        Rule::other => pair.as_str().to_owned(),
        _ => unreachable!(),
    }
}

pub fn translate(input: &str) -> String {
    PigLexer::parse(Rule::tokens, input)
        .expect("Grammar definition error")
        .next()
        .unwrap()
        .into_inner()
        .map(latin)
        .fold(String::new(), |mut acc, token| {
            acc.push_str(token.as_str());
            acc
        })
}

#[test]
fn pig_latin_test() {
    let input = "the quick brown fox jumps over the lazy dog.";
    assert_eq!(
        translate(input),
        "hetay uickqay rownbay oxfay umpsjay overay hetay azylay ogday."
    );
}
