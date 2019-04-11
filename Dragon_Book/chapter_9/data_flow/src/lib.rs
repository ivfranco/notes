pub mod reaching_def;

use lazy_static::lazy_static;
use petgraph::prelude::*;
use regex::Regex;

lazy_static! {
    static ref OP: Regex =
        Regex::new(r"^(?P<dst>\w+)\s?=\s?(?P<lhs>\w+)\s?(?P<op>\+|-|\*)\s?(?P<rhs>\w+)$").unwrap();
    static ref COPY: Regex = Regex::new(r"^(?P<dst>\w+)\s?=\s?(?P<src>\w+)$").unwrap();
}

type Var = String;
type Lit = u32;
pub type BlockID = usize;
pub type StmtID = usize;

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
            _ => panic!("Error: Invalid operator: {}", s),
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
            panic!("Error: Invalid Statement \"{}\"", s);
        }
    }

    fn dst(&self) -> Option<&str> {
        use Stmt::*;
        match self {
            Op(dst, ..) => Some(dst),
            Copy(dst, ..) => Some(dst),
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
            .map(Stmt::parse)
            .collect();

        Block { start, stmts }
    }

    pub fn in_range(&self, i: usize) -> bool {
        i >= self.start && i < self.start + self.len()
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

    pub fn stmts(&self) -> impl Iterator<Item = (usize, &Stmt)> {
        (self.start..).zip(self.stmts.iter())
    }
}

#[derive(Default)]
pub struct Program {
    blocks: Vec<Block>,
    graph: GraphMap<usize, (), Directed>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
        self.graph.add_node(self.blocks.len() - 1);
    }

    pub fn add_edge(&mut self, from: BlockID, to: BlockID) {
        self.graph.add_edge(from, to, ());
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }

    pub fn stmts(&self) -> impl Iterator<Item = (StmtID, &Stmt)> {
        self.blocks().flat_map(|b| b.stmts())
    }

    pub fn get_block(&self, i: BlockID) -> Option<&Block> {
        self.blocks.get(i)
    }

    pub fn get_stmt(&self, i: StmtID) -> Option<&Stmt> {
        self.blocks().find_map(|b| b.get(i))
    }

    pub fn predecessors(&self, block_id: BlockID) -> impl Iterator<Item = (BlockID, &Block)> {
        self.graph
            .neighbors_directed(block_id, Direction::Incoming)
            .map(move |i| (i, &self.blocks[i]))
    }

    pub fn successors(&self, block_id: BlockID) -> impl Iterator<Item = (BlockID, &Block)> {
        self.graph
            .neighbors_directed(block_id, Direction::Outgoing)
            .map(move |i| (i, &self.blocks[i]))
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
