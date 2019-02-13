use pest::iterators::Pair;
use pest::Parser;
use std::fmt::{self, Debug, Formatter};

#[derive(Parser)]
#[grammar = "./simple_c.pest"]
struct CLexer;

#[derive(Debug)]
enum CTag {
    Keyword,
    Ty,
    Punctuation,
    Operator,
    Constant(f64),
    Id,
}

pub struct CToken {
    tag: CTag,
    lexeme: String,
}

impl CToken {
    fn from_pair(pair: Pair<Rule>) -> Self {
        if pair.as_rule() == Rule::token {
            let inner = pair.into_inner().next().unwrap();
            return CToken::from_pair(inner);
        }

        let tag = match pair.as_rule() {
            Rule::keyword => CTag::Keyword,
            Rule::ty => CTag::Ty,
            Rule::punctuation => CTag::Punctuation,
            Rule::operator => CTag::Operator,
            Rule::constant => {
                let float = pair
                    .as_str()
                    .parse::<f64>()
                    .expect("Error: Invalid number constant");

                CTag::Constant(float)
            }
            Rule::id => CTag::Id,
            _ => unreachable!(),
        };

        CToken {
            tag,
            lexeme: pair.as_str().to_owned(),
        }
    }
}

impl Debug for CToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{:?}, {:?}>", self.tag, self.lexeme)
    }
}

pub fn lex_simple_c(input: &str) -> Vec<CToken> {
    let pair = CLexer::parse(Rule::tokens, input)
        .expect("Error: Invalid input string")
        .next()
        .unwrap();

    pair.into_inner().map(CToken::from_pair).collect()
}

#[test]
fn lex_test() {
    let input = "float limitedSquare(x) float x; {
/* returns x-squared, but never more than 100 */
return (x<=-10.0||x>=10.0)?100:x*x;
}";

    let pair = CLexer::parse(Rule::tokens, input).expect("Grammar definition error");
    println!("{:?}", pair);
}
