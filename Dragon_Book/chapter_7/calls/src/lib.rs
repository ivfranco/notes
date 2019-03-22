pub mod fibonacci;
pub mod quicksort;

use std::fmt::{self, Debug, Formatter};

const LEVEL: usize = 4;

fn pad(indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:width$}", "", width = indent)
}

pub struct Activation {
    name: String,
    subs: Vec<Activation>,
}

impl Activation {
    fn new(name: String, subs: Vec<Activation>) -> Self {
        Activation { name, subs }
    }

    pub fn depth(&self) -> usize {
        self.subs.iter().map(|sub| sub.depth()).max().unwrap_or(0) + 1
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        pad(indent, f)?;
        writeln!(f, "{}", self.name)?;
        for sub in &self.subs {
            sub.format(indent + LEVEL, f)?;
        }
        Ok(())
    }
}

impl Debug for Activation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.format(0, f)
    }
}
