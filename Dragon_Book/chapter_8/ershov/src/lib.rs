pub mod builder;
pub mod machine_code;
pub mod utils;

use lalrpop_util::lalrpop_mod;
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

#[derive(Debug)]
pub struct Node {
    expr: Expr,
    pub label: u8,
}

impl Node {
    pub fn bin(lhs: Self, op: BinOp, rhs: Self) -> Self {
        use std::cmp;

        let label = if lhs.label == rhs.label {
            lhs.label + 1
        } else {
            cmp::max(lhs.label, rhs.label)
        };
        let expr = Expr::Bin(Box::new(lhs), op, Box::new(rhs));

        Node { expr, label }
    }

    pub fn un(op: UnOp, inner: Self) -> Self {
        let label = inner.label;
        let expr = Expr::Un(op, Box::new(inner));
        Node { expr, label }
    }

    pub fn var(var: Var) -> Self {
        Node {
            expr: Expr::Var(var),
            label: 1,
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
}
