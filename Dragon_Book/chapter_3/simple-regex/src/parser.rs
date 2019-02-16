use pest::iterators::Pair;
use pest::Parser;

#[derive(Debug, PartialEq)]
pub enum Regex {
    Empty,
    Literal(char),
    Kleene(Box<Regex>),
    Union(Box<Regex>, Box<Regex>),
    Concat(Box<Regex>, Box<Regex>),
}

impl Regex {
    pub fn parse(input: &str) -> Self {
        let pair = RegexParser::parse(Rule::union, input)
            .expect("Error: Invalid regex expression")
            .next()
            .unwrap();

        Regex::from_pair(pair)
    }

    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::empty => Regex::Empty,
            Rule::factor => {
                let c = pair.as_str().chars().next().unwrap();
                if let Some(inner) = pair.into_inner().next() {
                    Regex::from_pair(inner)
                } else {
                    Regex::Literal(c)
                }
            }
            Rule::kleene => {
                let mut inners = pair.into_inner();

                let factor = Regex::from_pair(inners.next().unwrap());
                if inners.next().is_some() {
                    Regex::Kleene(Box::new(factor))
                } else {
                    factor
                }
            }
            Rule::concat => {
                let mut inners = pair.into_inner();

                let kleene = Regex::from_pair(inners.next().unwrap());
                if let Some(rest) = inners.next() {
                    let concat = Regex::from_pair(rest);
                    Regex::Concat(Box::new(kleene), Box::new(concat))
                } else {
                    kleene
                }
            }
            Rule::union => {
                let mut inners = pair.into_inner();

                let concat = Regex::from_pair(inners.next().unwrap());
                if let Some(rest) = inners.next() {
                    let union = Regex::from_pair(rest);
                    Regex::Union(Box::new(concat), Box::new(union))
                } else {
                    concat
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Parser)]
#[grammar = "./regex.pest"]
struct RegexParser;

#[test]
fn parse_test() {
    let regex = Regex::parse("((ε|a)b*)*");

    assert_eq!(
        regex,
        Regex::Kleene(Box::new(Regex::Concat(
            Box::new(Regex::Union(
                Box::new(Regex::Empty),
                Box::new(Regex::Literal('a')),
            )),
            Box::new(Regex::Kleene(Box::new(Regex::Literal('b'))))
        )))
    );
}
