use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref OP: Regex =
        Regex::new(r"^(?P<dst>\w+) = (?P<lhs>\w+) (?P<op>\+|-|\*) (?P<rhs>\w+)$").unwrap();
    static ref COPY: Regex = Regex::new(r"^(?P<dst>\w+) = (?P<src>\w+)$").unwrap();
}

type Var = String;
type Lit = u32;

#[derive(Debug, PartialEq)]
pub enum RValue {
    Var(Var),
    Lit(Lit),
}

impl RValue {
    fn parse(s: &str) -> Self {
        if let Ok(lit) = s.parse::<Lit>() {
            lit.into()
        } else {
            s.to_string().into()
        }
    }
}

impl From<Var> for RValue {
    fn from(var: Var) -> RValue {
        RValue::Var(var)
    }
}

impl From<Lit> for RValue {
    fn from(lit: Lit) -> RValue {
        RValue::Lit(lit)
    }
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
}

impl BinOp {
    fn parse(s: &str) -> Self {
        use BinOp::*;
        match s {
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            _ => panic!("Error: Invalid operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Op(Var, RValue, BinOp, RValue),
    Copy(Var, RValue),
}

impl Stmt {
    fn parse(s: &str) -> Self {
        use Stmt::*;

        if let Some(cap) = OP.captures(s) {
            let dst = cap["dst"].to_string();
            let lhs = RValue::parse(&cap["lhs"]);
            let op = BinOp::parse(&cap["op"]);
            let rhs = RValue::parse(&cap["rhs"]);

            Op(dst, lhs, op, rhs)
        } else if let Some(cap) = COPY.captures(s) {
            let dst = cap["dst"].to_string();
            let src = RValue::parse(&cap["src"]);

            Copy(dst, src)
        } else {
            panic!("Error: Invalid Statement");
        }
    }
}

pub struct Block {
    start: usize,
    stmts: Vec<Stmt>,
}

impl Block {
    pub fn parse(start: usize, s: &str) -> Self {
        let stmts = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| Stmt::parse(l))
            .collect();

        Block { start, stmts }
    }

    pub fn is_empty(&self) -> bool {
        self.stmts.is_empty()
    }

    pub fn len(&self) -> usize {
        self.stmts.len()
    }

    pub fn get(&self, i: usize) -> Option<&Stmt> {
        if i < self.start {
            None
        } else {
            self.stmts.get(i - self.start)
        }
    }
}

#[cfg(test)]
fn s(s: &str) -> String {
    s.to_owned()
}

#[test]
fn parse_test() {
    let block = "c = a + b
d = c - a";

    let stmts = vec![
        Stmt::Op(s("c"), s("a").into(), BinOp::Add, s("b").into()),
        Stmt::Op(s("d"), s("c").into(), BinOp::Sub, s("a").into()),
    ];

    assert_eq!(Block::parse(0, block).stmts, stmts);
}
