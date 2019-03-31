use lazy_static::lazy_static;
use petgraph::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<dst>\w) = (?P<lhs>\w) (?P<op>\+|\-|\*) (?P<rhs>\w)").unwrap();
}

type Var = char;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Add,
    Sub,
    Mul,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            _ => unreachable!(),
        }
    }
}

impl Debug for Op {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
        }
    }
}

fn fc(s: &str) -> char {
    s.chars().next().unwrap()
}

#[derive(Clone)]
struct Stmt {
    dst: Var,
    lhs: Var,
    op: Op,
    rhs: Var,
}

impl Stmt {
    fn parse(s: &str) -> Self {
        let cap = RE.captures(s).unwrap();
        Stmt {
            dst: fc(&cap["dst"]),
            lhs: fc(&cap["lhs"]),
            op: Op::parse(&cap["op"]),
            rhs: fc(&cap["rhs"]),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExprNode {
    lhs: NodeIndex,
    op: Op,
    rhs: NodeIndex,
}

impl ExprNode {
    fn new(lhs: NodeIndex, op: Op, rhs: NodeIndex) -> Self {
        ExprNode { lhs, op, rhs }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StmtNode {
    expr: ExprNode,
    defs: Vec<Var>,
}

impl StmtNode {
    fn new(expr: ExprNode, def: Var) -> Self {
        StmtNode {
            expr,
            defs: vec![def],
        }
    }
}

impl Debug for StmtNode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}: {:?}", self.expr.op, self.defs)
    }
}

impl StmtNode {
    fn push_def(&mut self, def: Var) {
        self.defs.push(def);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Stmt(StmtNode),
    Init(Var),
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Node::Stmt(stmt) => Debug::fmt(stmt, f),
            Node::Init(var) => write!(f, "{}0", var),
        }
    }
}

impl Node {
    fn push_def(&mut self, def: Var) {
        if let Node::Stmt(stmt) = self {
            stmt.push_def(def);
        }
    }
}

type DAG = Graph<Node, ()>;

#[derive(Default)]
pub struct Builder {
    dag: DAG,
    last: HashMap<Var, NodeIndex>,
    cache: HashMap<ExprNode, NodeIndex>,
}

impl Builder {
    fn new() -> Self {
        Self::default()
    }

    fn last_def(&mut self, var: Var) -> NodeIndex {
        if let Some(idx) = self.last.get(&var) {
            *idx
        } else {
            let node = Node::Init(var);
            let idx = self.dag.add_node(node);
            self.last.insert(var, idx);
            idx
        }
    }

    fn add_stmt(&mut self, stmt: &Stmt) {
        let l = self.last_def(stmt.lhs);
        let r = self.last_def(stmt.rhs);
        let expr_node = ExprNode::new(l, stmt.op, r);

        let idx = if let Some(idx) = self.cache.get(&expr_node) {
            self.dag[*idx].push_def(stmt.dst);
            *idx
        } else {
            let stmt_node = StmtNode::new(expr_node.clone(), stmt.dst);
            let idx = self.dag.add_node(Node::Stmt(stmt_node));
            self.dag.add_edge(idx, l, ());
            self.dag.add_edge(idx, r, ());
            self.cache.insert(expr_node, idx);
            idx
        };

        self.last.insert(stmt.dst, idx);
    }

    fn build(&mut self) -> DAG {
        use std::mem;
        mem::replace(&mut self.dag, DAG::new())
    }

    pub fn parse(s: &str) -> DAG {
        let mut builder = Self::new();

        for stmt in s.lines().map(|l| Stmt::parse(l)) {
            builder.add_stmt(&stmt);
        }

        builder.build()
    }
}

#[test]
fn build_test() {
    let stmts = "a = b + c
b = a - d
c = b + c
d = a - d";

    let dag = Builder::parse(stmts);
    // println!("{:#?}", dag);
    assert_eq!(dag.node_count(), 6);
}
