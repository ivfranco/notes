use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter, Write};
use std::hash::Hash;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Symbol<T> {
    N(usize),
    T(T),
}
use self::Symbol::*;

impl<T: Debug> Symbol<T> {
    pub fn to_string(&self, rev_map: &HashMap<usize, String>) -> String {
        match self {
            N(s) => rev_map[s].to_owned(),
            T(t) => format!("{:?}", t),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Production<T> {
    pub head: usize,
    pub body: Vec<Symbol<T>>,
}

impl<T: Debug> Production<T> {
    pub fn to_string(&self, rev_map: &HashMap<usize, String>) -> String {
        let mut symbols = self
            .body
            .iter()
            .map(|symbol| symbol.to_string(rev_map))
            .collect::<Vec<_>>()
            .join(" ");

        if symbols.is_empty() {
            symbols = "ε".to_owned();
        }

        format!("{} -> {}", N::<T>(self.head).to_string(rev_map), symbols)
    }
}

impl<T: PartialEq> Production<T> {
    pub fn is_left_recursive(&self) -> bool {
        self.body.first() == Some(&N(self.head))
    }
}

#[derive(Debug)]
enum ErrorKind {
    UnexpectedEof,
    UnexpectedTerminal,
}
use self::ErrorKind::*;

pub struct ParseError {
    kind: ErrorKind,
    pos: usize,
    msg: Option<String>,
}

impl ParseError {
    fn new(kind: ErrorKind, pos: usize) -> Self {
        ParseError {
            kind,
            pos,
            msg: None,
        }
    }

    fn with_msg(self, msg: String) -> Self {
        ParseError {
            kind: self.kind,
            pos: self.pos,
            msg: Some(msg),
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?} at position {}", self.kind, self.pos)?;
        if let Some(msg) = &self.msg {
            write!(f, ": {}", msg)?;
        }
        Ok(())
    }
}

type Tables<T> = Vec<HashMap<Option<T>, Production<T>>>;

pub struct ParseTable<T> {
    start: usize,
    tables: Tables<T>,
}

impl<T> ParseTable<T> {
    pub fn new(start: usize, tables: Tables<T>) -> Self {
        ParseTable { start, tables }
    }
}

impl<T: Eq + Hash + Debug> ParseTable<T> {
    pub fn to_string(&self, rev_map: &HashMap<usize, String>) -> String {
        let mut res = String::new();
        for (n, map) in self.tables.iter().enumerate() {
            let nonterm = &rev_map[&n];
            for (opt, p) in map.iter() {
                let term = if let Some(t) = opt {
                    format!("{:?}", t)
                } else {
                    "$".to_owned()
                };

                writeln!(res, "M[{}, {}]: {}", nonterm, term, p.to_string(rev_map)).unwrap();
            }
        }

        res
    }
}

impl<T: Debug + Clone + Eq + Hash> ParseTable<T> {
    pub fn parse(&self, input: &[T]) -> Result<Vec<&Production<T>>, ParseError> {
        let mut output = vec![];
        let mut stack: Vec<Symbol<T>> = vec![N(self.start)];
        let mut i = 0;

        while let Some(symbol) = stack.pop() {
            let next = input.get(i).cloned();

            match symbol {
                T(t) => {
                    if next.is_none() {
                        let err = ParseError::new(UnexpectedEof, i);
                        return Err(err);
                    } else if next.unwrap() != t {
                        let err = ParseError::new(UnexpectedTerminal, i)
                            .with_msg(format!("Expected {:?}", t));
                        return Err(err);
                    } else {
                        i += 1;
                    }
                }
                N(s) => {
                    let p = self.tables[s]
                        .get(&next)
                        .ok_or_else(|| ParseError::new(UnexpectedTerminal, i))?;

                    output.push(p);
                    stack.extend(p.body.iter().rev().cloned());
                }
            }
        }

        Ok(output)
    }
}
