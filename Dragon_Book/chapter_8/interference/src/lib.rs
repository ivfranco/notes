use lazy_static::lazy_static;
use petgraph::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug, Formatter};

lazy_static! {
    static ref STMT: Regex =
        Regex::new(r"(?P<dst>\w) = (?P<lhs>\w) (\+|\-|\*|/) (?P<rhs>\w)").unwrap();
}

type Var = char;
type Living = HashSet<Var>;

fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

struct Stmt {
    dst: Var,
    lhs: Var,
    rhs: Var,
}

impl Stmt {
    fn parse(s: &str) -> Self {
        let cap = STMT.captures(s).unwrap();

        Stmt {
            dst: first_char(&cap["dst"]),
            lhs: first_char(&cap["lhs"]),
            rhs: first_char(&cap["rhs"]),
        }
    }

    fn update_living(&self, living: &mut Living) {
        living.remove(&self.dst);
        living.insert(self.lhs);
        living.insert(self.rhs);
    }
}

pub struct Block {
    stmts: Vec<Stmt>,
    on_exit: Living,
}

impl Block {
    pub fn parse(stmts: &str, on_exit: &str) -> Self {
        let stmts = stmts.lines().map(|line| Stmt::parse(line)).collect();
        let on_exit = on_exit.chars().collect();

        Block { stmts, on_exit }
    }
}

#[derive(Default)]
pub struct Interference {
    graph: Graph<Var, (), Undirected>,
    nodes: HashMap<Var, NodeIndex>,
}

impl Interference {
    pub fn new() -> Self {
        Self::default()
    }

    fn add_node(&mut self, v: Var) -> NodeIndex {
        if let Some(node) = self.nodes.get(&v) {
            *node
        } else {
            let node = self.graph.add_node(v);
            self.nodes.insert(v, node);
            node
        }
    }

    fn add_edge(&mut self, v0: Var, v1: Var) {
        if v0 != v1 {
            let e0 = self.add_node(v0);
            let e1 = self.add_node(v1);
            self.graph.add_edge(e0, e1, ());
        }
    }

    pub fn update(&mut self, block: &Block) {
        let mut living = block.on_exit.clone();

        for stmt in block.stmts.iter().rev() {
            for v in &living {
                self.add_edge(stmt.dst, *v);
            }
            stmt.update_living(&mut living);
        }
    }
}

impl Debug for Interference {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Debug::fmt(&self.graph, f)
    }
}

#[test]
fn interference_test() {
    let block = Block::parse(
        "a = b + c
d = d - b
e = a + f",
        "acdef",
    );

    let mut graph = Interference::new();
    graph.update(&block);
    // println!("{:#?}", graph);
    assert_eq!(graph.graph.edge_count(), 11);
}
