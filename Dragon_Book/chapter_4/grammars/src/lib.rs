use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone)]
pub enum Symbol {
    NonTerminal(String),
    Terminal(String),
}

impl Symbol {
    fn as_str(&self) -> &str {
        match self {
            Symbol::NonTerminal(s) => &s,
            Symbol::Terminal(s) => &s,
        }
    }
}

#[derive(Clone)]
pub struct Production {
    pub head: String,
    pub body: Vec<Symbol>,
}

impl Production {
    fn first_body_symbol(&self) -> Option<Symbol> {
        self.body.first().cloned()
    }
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

    fn eliminate_left_recursion(&self) -> Grammar {
        let map = self.group_by_head();
        let heads = map.keys().cloned().collect::<Vec<_>>();
        let mut productions: HashMap<String, Vec<Production>> = HashMap::new();

        for (i, head) in heads.iter().enumerate() {}

        Grammar {
            start: self.start.clone(),
            productions: productions.drain().flat_map(|(_, ps)| ps).collect(),
        }
    }
}
