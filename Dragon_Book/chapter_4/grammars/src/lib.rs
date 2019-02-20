use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub enum Symbol {
    NonTerminal(String),
    Terminal(String),
    Empty,
}

use self::Symbol::*;

impl Symbol {
    fn as_str(&self) -> &str {
        match self {
            Symbol::NonTerminal(s) => &s,
            Symbol::Terminal(s) => &s,
            Symbol::Empty => "ε",
        }
    }
}

#[derive(Clone)]
pub struct Production {
    pub head: String,
    pub body: Vec<Symbol>,
}

impl Production {
    fn new(head: &str, body: Vec<Symbol>) -> Self {
        Production {
            head: head.to_owned(),
            body,
        }
    }

    fn prefix_nonterm(&self, nonterm: &str) -> bool {
        self.body
            .first()
            .map(|s| match s {
                NonTerminal(..) => s.as_str() == nonterm,
                _ => false,
            })
            .unwrap_or(false)
    }

    fn prefix(&self) -> Symbol {
        self.body.first().cloned().unwrap()
    }
}

fn eliminate_immediate_left_recursion(productions: &[Production]) -> Vec<Production> {
    assert!(
        !productions.is_empty(),
        "Error: Empty set of productions cannot be recursive",
    );

    assert!(
        productions.iter().all(|p| p.head == productions[0].head),
        "Error: Inconsistent head detected when eliminating immediate left recursions",
    );

    let head = productions[0].head.clone();
    let interm = format!("{}'", head);

    let mut ps: Vec<Production> = productions
        .iter()
        .map(|p| {
            if p.prefix_nonterm(&head) {
                let mut body = (&p.body[1..]).to_owned();
                body.push(NonTerminal(interm.clone()));
                Production::new(&interm, body)
            } else {
                let mut body = p.body.to_owned();
                body.push(NonTerminal(interm.clone()));
                Production::new(&head, body)
            }
        })
        .collect();

    ps.push(Production::new(&interm, vec![Empty]));

    ps
}

impl Debug for Production {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let symbols = self
            .body
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{} -> {}", self.head, symbols)
    }
}

pub struct Grammar {
    pub start: String,
    productions: Vec<Production>,
}

impl Grammar {
    fn group_by_head(&self) -> HashMap<&str, Vec<&Production>> {
        let mut map = HashMap::new();

        for production in &self.productions {
            map.entry(production.head.as_str())
                .or_insert_with(|| vec![])
                .push(production);
        }

        map
    }
}
