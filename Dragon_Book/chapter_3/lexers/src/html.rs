use pest::iterators::Pair;
use pest::Parser;
use std::fmt::{self, Debug, Formatter};

#[derive(Parser)]
#[grammar = "./html.pest"]
struct HTMLLexer;

#[derive(Debug)]
enum HTMLTag {
    Start(String),
    End(String),
    Text,
}

pub struct HTMLToken {
    tag: HTMLTag,
    lexeme: String,
}

impl HTMLToken {
    fn from_pair(pair: Pair<Rule>) -> Self {
        if pair.as_rule() == Rule::token {
            let inner = pair.into_inner().next().unwrap();
            return HTMLToken::from_pair(inner);
        }

        let tag = match pair.as_rule() {
            Rule::start => {
                let tag = pair
                    .clone()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_owned();
                HTMLTag::Start(tag)
            }
            Rule::end => {
                let tag = pair
                    .clone()
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_owned();
                HTMLTag::End(tag)
            }
            Rule::text => HTMLTag::Text,
            rule => unreachable!("{:?}", rule),
        };

        HTMLToken {
            tag,
            lexeme: pair.as_str().to_owned(),
        }
    }
}

impl Debug for HTMLToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{:?}, {:?}>", self.tag, self.lexeme)
    }
}

pub fn lex_html(input: &str) -> Vec<HTMLToken> {
    let pair = HTMLLexer::parse(Rule::tokens, input)
        .expect("Error: Invalid input string")
        .next()
        .unwrap();

    pair.into_inner().map(HTMLToken::from_pair).collect()
}

#[test]
fn lex_test() {
    let input = r#"Here is a photo of <B>my house</B>:
<P><IMG SRC = "house.gif"><BR>
See <A HREF = "morePix.html">More Pictures</A> if you
liked that one.<P>"#;

    let pair = HTMLLexer::parse(Rule::tokens, input)
        .expect("Error in grammar definition")
        .next()
        .unwrap();

    println!("{:?}", pair);
}
