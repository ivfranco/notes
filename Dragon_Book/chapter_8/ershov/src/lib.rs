pub mod builder;
pub mod machine_code;
pub mod utils;

use lalrpop_util::lalrpop_mod;
use std::cmp;
use std::error::Error;

lalrpop_mod!(pub grammar);

pub type Var = char;

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    fn code(self) -> &'static str {
        use BinOp::*;
        match self {
            Add => "ADD",
            Sub => "SUB",
            Mul => "MUL",
            Div => "DIV",
        }
    }
}

#[derive(Debug)]
pub enum UnOp {
    Neg,
    Deref,
}

#[derive(Debug)]
pub enum Expr {
    Bin(Box<Node>, BinOp, Box<Node>),
    Un(UnOp, Box<Node>),
    Var(Var),
}

#[derive(Debug, PartialEq)]
pub struct Cost {
    m0: u32,
    r1: u32,
    r2: u32,
}

impl Cost {
    fn var() -> Self {
        Cost {
            m0: 0,
            r1: 1,
            r2: 1,
        }
    }

    fn un(inner: &Self) -> Self {
        // assume the operands of unary operators must be loaded into register
        let r1 = inner.r1 + 1;
        let r2 = inner.r2 + 1;
        let m0 = cmp::min(r1, r2) + 1;

        Cost { m0, r1, r2 }
    }

    fn bin(lhs: &Self, rhs: &Self) -> Self {
        // assume the right operand of binary operators can be in memory
        let r1 = lhs.r1 + rhs.m0 + 1;
        let r2 = *[lhs.r2 + rhs.r1, lhs.r1 + rhs.r2, lhs.r2 + rhs.m0]
            .iter()
            .min()
            .unwrap()
            + 1;
        let m0 = cmp::min(r1, r2) + 1;

        Cost { m0, r1, r2 }
    }
}

#[derive(Debug)]
pub struct Node {
    pub label: u8,
    pub cost: Cost,
    expr: Expr,
}

impl Node {
    pub fn bin(lhs: Self, op: BinOp, rhs: Self) -> Self {
        let label = if lhs.label == rhs.label {
            lhs.label + 1
        } else {
            cmp::max(lhs.label, rhs.label)
        };
        let cost = Cost::bin(&lhs.cost, &rhs.cost);
        let expr = Expr::Bin(Box::new(lhs), op, Box::new(rhs));

        Node { expr, label, cost }
    }

    pub fn un(op: UnOp, inner: Self) -> Self {
        let label = inner.label;
        let cost = Cost::un(&inner.cost);
        let expr = Expr::Un(op, Box::new(inner));
        Node { expr, label, cost }
    }

    pub fn var(var: Var) -> Self {
        Node {
            expr: Expr::Var(var),
            label: 1,
            cost: Cost::var(),
        }
    }

    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        grammar::EParser::new().parse(s).map_err(Box::from)
    }
}

#[test]
fn parse_test() {
    let expr = "(-a + *p) * ((b - *q)/(-c + *r))";
    let node = Node::parse(expr).unwrap();
    // println!("{:#?}", node);
    assert_eq!(node.label, 3);

    let expr = "y[j] * z[k]";
    let node = Node::parse(expr).unwrap();
    assert_eq!(node.label, 3);
}

#[test]
fn cost_test() {
    let expr = "(a-b)+c*(d/e)";
    assert_eq!(
        Node::parse(expr).unwrap().cost,
        Cost {
            m0: 8,
            r1: 8,
            r2: 7,
        }
    );
}
