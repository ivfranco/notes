#![feature(box_patterns)]

#[macro_use]
extern crate lalrpop_util;

pub mod machine_code;
pub mod rewrite;

use std::error::Error;
use std::fmt::{self, Debug, Formatter};

lalrpop_mod!(pub expr);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
    Mul,
}

impl BinOp {
    fn code(self) -> &'static str {
        use BinOp::*;

        match self {
            Add => "ADD",
            Mul => "MUL",
        }
    }
}

pub type Var = char;

#[derive(PartialEq, Clone, Copy)]
pub enum Reg {
    GP(u8),
    SP,
    NP,
}

impl Debug for Reg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Reg::GP(r) => write!(f, "R{}", r),
            Reg::SP => write!(f, "RSP"),
            Reg::NP => write!(f, "/* insert new register here */"),
        }
    }
}

pub type Mem = char;

#[derive(PartialEq, Clone, Copy)]
pub enum Cst {
    Var(Var),
    Lit(usize),
}

impl Debug for Cst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Cst::Var(var) => write!(f, "C{}", var),
            Cst::Lit(lit) => write!(f, "{}", lit),
        }
    }
}

#[derive(PartialEq)]
pub enum Node {
    Assign(Box<Node>, Box<Node>),
    Op(Box<Node>, BinOp, Box<Node>),
    Ind(Box<Node>),
    Mem(Mem),
    Cst(Cst),
    Reg(Reg),
    End,
}

impl Node {
    pub fn op(lhs: Node, op: BinOp, rhs: Node) -> Self {
        Node::Op(Box::new(lhs), op, Box::new(rhs))
    }

    fn ind(node: Node) -> Self {
        Node::Ind(Box::new(node))
    }

    pub fn assign(dst: Node, src: Node) -> Self {
        Node::Assign(Box::new(dst), Box::new(src))
    }

    pub fn access(array: Cst, idx: Cst) -> Self {
        Node::ind(Node::op(
            Node::op(Node::Cst(array), BinOp::Add, Node::Reg(Reg::SP)),
            BinOp::Add,
            Node::ind(Node::op(Node::Cst(idx), BinOp::Add, Node::Reg(Reg::SP))),
        ))
    }

    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        expr::AParser::new().parse(s).map_err(Box::from)
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Node::End | Node::Mem(..) | Node::Cst(..) | Node::Reg(..) => true,
            _ => false,
        }
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        use self::Node::*;
        const LEVEL: usize = 4;

        write!(f, "{:width$}", "", width = indent)?;
        let next = indent + LEVEL;
        match self {
            Assign(dst, src) => {
                writeln!(f, "=")?;
                dst.format(next, f)?;
                src.format(next, f)
            }
            Op(lhs, op, rhs) => {
                writeln!(f, "{:?}", op)?;
                lhs.format(next, f)?;
                rhs.format(next, f)
            }
            Ind(node) => {
                writeln!(f, "Ind")?;
                node.format(next, f)
            }
            Mem(mem) => writeln!(f, "M{}", mem),
            Cst(cst) => writeln!(f, "{:?}", cst),
            Reg(reg) => writeln!(f, "{:?}", reg),
            End => writeln!(f, "END"),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.format(0, f)
    }
}

#[test]
fn parse_test() {
    let expr = "x[i] = y[j] * z[k];";
    let _node = Node::parse(expr).unwrap();
    // println!("{:?}", _node);

    let tree = Node::assign(
        Node::Mem('x'),
        Node::op(Node::Mem('x'), BinOp::Add, Node::Cst(Cst::Lit(1))),
    );

    assert_eq!(Node::parse("x = x + 1;").unwrap(), tree);
}
