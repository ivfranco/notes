pub mod available_expr;
pub mod constant_propagation;
pub mod dominator;
pub(crate) mod framework;
pub mod lazy_code_motion;
pub mod live_var;
pub mod reaching_def;
pub mod utils;

use lazy_static::lazy_static;
use petgraph::prelude::*;
use petgraph::visit::{depth_first_search, Dfs, DfsEvent, VisitMap};
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};

lazy_static! {
    static ref OP: Regex =
        Regex::new(r"^(?P<dst>\w+)\s?=\s?(?P<lhs>\w+)\s?(?P<op>\+|-|\*)\s?(?P<rhs>\w+)$").unwrap();
    static ref COPY: Regex = Regex::new(r"^(?P<dst>\w+)\s?=\s?(?P<src>\w+)$").unwrap();
}

type Var = String;
type Lit = u32;
pub type BlockID = usize;
pub type StmtID = usize;

#[derive(PartialEq, Eq, Hash)]
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

    fn var(&self) -> Option<&str> {
        if let RValue::Var(var) = self {
            Some(var)
        } else {
            None
        }
    }
}

impl Debug for RValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            RValue::Lit(lit) => Debug::fmt(lit, f),
            RValue::Var(var) => write!(f, "{}", var),
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    fn apply(self, lhs: Lit, rhs: Lit) -> Lit {
        use BinOp::*;
        match self {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
        }
    }
}

impl Debug for BinOp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use BinOp::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Op(Var, RValue, BinOp, RValue),
    Copy(Var, RValue),
}

impl Stmt {
    pub fn parse(s: &str) -> Self {
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

    fn def(&self) -> Option<&str> {
        use Stmt::*;
        match self {
            Op(dst, ..) => Some(dst),
            Copy(dst, ..) => Some(dst),
        }
    }

    fn uses(&self) -> Vec<&str> {
        use Stmt::*;
        match self {
            Op(_, lhs, _, rhs) => vec![lhs, rhs].into_iter().flat_map(RValue::var).collect(),
            Copy(_, src) => src.var().into_iter().collect(),
        }
    }

    pub fn as_expr(&self) -> Option<Expr<'_>> {
        if let Stmt::Op(_, lhs, op, rhs) = self {
            Some(Expr { lhs, op: *op, rhs })
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Expr<'a> {
    lhs: &'a RValue,
    op: BinOp,
    rhs: &'a RValue,
}

impl<'a> Expr<'a> {
    fn uses(&self, var: &str) -> bool {
        self.lhs.var() == Some(var) || self.rhs.var() == Some(var)
    }
}

impl<'a> Debug for Expr<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?} {:?} {:?}", self.lhs, self.op, self.rhs)
    }
}

#[derive(PartialEq)]
pub enum BlockType {
    Entry,
    Basic,
    Exit,
}

pub struct Block {
    start: usize,
    stmts: Vec<Stmt>,
    btype: BlockType,
}

impl Block {
    pub fn entry() -> Self {
        Block {
            start: 0,
            btype: BlockType::Entry,
            stmts: vec![],
        }
    }

    pub fn exit() -> Self {
        Block {
            start: 0,
            btype: BlockType::Exit,
            stmts: vec![],
        }
    }

    pub fn empty() -> Self {
        Block {
            start: 0,
            btype: BlockType::Basic,
            stmts: vec![],
        }
    }

    pub fn parse(start: usize, s: &str) -> Self {
        let stmts = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(Stmt::parse)
            .collect();

        Block {
            start,
            stmts,
            btype: BlockType::Basic,
        }
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

    pub fn stmts(&self) -> impl DoubleEndedIterator<Item = &Stmt> {
        self.stmts.iter()
    }

    pub fn stmts_indices(&self) -> impl Iterator<Item = (usize, &Stmt)> {
        (self.start..).zip(self.stmts.iter())
    }

    pub fn exprs(&self) -> impl Iterator<Item = Expr<'_>> {
        self.stmts().filter_map(Stmt::as_expr)
    }
}

#[derive(Default)]
pub struct Program {
    blocks: Vec<Block>,
    graph: GraphMap<usize, (), Directed>,
}

impl Program {
    pub fn new(blocks: Vec<Block>, edges: &[(BlockID, BlockID)]) -> Self {
        let graph = GraphMap::from_edges(edges);
        Program { blocks, graph }
    }

    /// block id 1 still refers to the first non-ENTRY block
    pub fn with_entry_exit(mut blocks: Vec<Block>, edges: &[(BlockID, BlockID)]) -> Self {
        blocks.insert(0, Block::entry());
        blocks.push(Block::exit());
        Self::new(blocks, edges)
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn entry(&self) -> BlockID {
        0
    }

    pub fn exit(&self) -> Option<BlockID> {
        let id = self.len() - 1;
        if self.blocks[id].btype == BlockType::Exit {
            Some(id)
        } else {
            None
        }
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }

    pub fn block_range(&self) -> impl Iterator<Item = BlockID> {
        0..self.len()
    }

    pub fn stmts_indices(&self) -> impl Iterator<Item = (StmtID, &Stmt)> {
        self.blocks().flat_map(Block::stmts_indices)
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

    pub fn edges(&self) -> Vec<(BlockID, BlockID)> {
        self.graph
            .all_edges()
            .map(|(from, to, _)| (from, to))
            .collect()
    }

    pub fn insert_empty_between(&mut self, from: BlockID, to: BlockID) {
        assert!(
            self.graph.contains_edge(from, to),
            "Error: Insert along nonexist edge"
        );

        let empty = Block::empty();
        let empty_id = self.blocks.len();

        self.graph.remove_edge(from, to);
        self.blocks.push(empty);
        self.graph.add_edge(from, empty_id, ());
        self.graph.add_edge(empty_id, to, ());
    }

    pub fn exprs(&self) -> impl Iterator<Item = Expr<'_>> {
        self.blocks().flat_map(Block::exprs)
    }

    pub fn dfs_order<F>(&self, start: BlockID, mut f: F) -> Vec<BlockID>
    where
        F: FnMut(DfsEvent<BlockID>),
    {
        let mut nodes = vec![];

        depth_first_search(&self.graph, Some(start), |event| {
            if let DfsEvent::Discover(n, _) = event {
                nodes.push(n);
                f(event);
            } else {
                f(event);
            }
        });

        nodes
    }

    pub fn natural_loop(&self, from: BlockID, to: BlockID) -> HashSet<BlockID> {
        if from == to {
            // for some reason, dfs in petgraph will run over the start node
            // even when it is marked as visited
            return Some(from).into_iter().collect();
        }

        let reverse: GraphMap<BlockID, (), Directed> =
            GraphMap::from_edges(self.graph.all_edges().map(|(from, to, _)| (to, from)));

        let mut dfs = Dfs::new(&reverse, from);
        dfs.discovered.visit(to);
        while let Some(..) = dfs.next(&reverse) {}

        dfs.discovered
    }
}

#[cfg(test)]
fn s(s: &str) -> String {
    s.to_owned()
}

#[cfg(test)]
pub fn figure_9_13() -> Program {
    use crate::Block;

    let blocks = vec![
        Block::entry(), // ENTRY
        Block::parse(
            1,
            "i = m-1
j = n
a = u1",
        ),
        Block::parse(
            4,
            "i = i+1
j = j-1",
        ),
        Block::parse(6, "a = u2"),
        Block::parse(7, "i = u3"),
        Block::exit(), // EXIT
    ];

    let edges = &[(0, 1), (1, 2), (2, 3), (2, 4), (3, 4), (4, 2), (4, 5)];

    Program::new(blocks, edges)
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

#[test]
fn natural_loop_test() {
    let program = crate::dominator::figure_9_38();

    assert_eq!(
        program.natural_loop(10, 7),
        vec![7, 8, 10].into_iter().collect()
    );
    assert_eq!(
        program.natural_loop(7, 4),
        vec![4, 5, 6, 7, 8, 10].into_iter().collect()
    );
    assert_eq!(
        program.natural_loop(4, 3),
        vec![3, 4, 5, 6, 7, 8, 10].into_iter().collect()
    );
    assert_eq!(
        program.natural_loop(8, 3),
        vec![3, 4, 5, 6, 7, 8, 10].into_iter().collect()
    );
    assert_eq!(program.natural_loop(9, 1), (1..=10).collect());
}
