use std::error::Error;
use std::fmt::{self, Debug, Formatter};

lalrpop_mod!(pub infix);

const LEVEL: usize = 4;

pub enum Expr {
    Mul(Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Lit(i32),
}

impl Expr {
    fn tag(&self) -> &'static str {
        match self {
            Expr::Mul(..) => "Mul",
            Expr::Add(..) => "Add",
            Expr::Lit(..) => "Lit",
        }
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Expr::Mul(lhs, rhs) => {
                lhs.format(indent + LEVEL, f)?;
                rhs.format(indent + LEVEL, f)?;
            }
            Expr::Add(lhs, rhs) => {
                lhs.format(indent + LEVEL, f)?;
                rhs.format(indent + LEVEL, f)?;
            }
            _ => (),
        }

        Ok(())
    }
}

pub struct Node {
    pub expr: Expr,
    pub val: i32,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            expr: Expr::Lit(val),
            val,
        }
    }

    pub fn add(self, rhs: Node) -> Self {
        add(self, rhs)
    }

    pub fn mul(self, rhs: Node) -> Self {
        mul(self, rhs)
    }

    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        infix::LParser::new().parse(s).map_err(|e| e.into())
    }

    pub fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:width$}", "", width = indent)?;
        writeln!(f, "{}.val = {}", self.expr.tag(), self.val)?;
        self.expr.format(indent, f)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.format(0, f)
    }
}

fn add(lhs: Node, rhs: Node) -> Node {
    let val = lhs.val + rhs.val;
    let expr = Expr::Add(Box::new(lhs), Box::new(rhs));
    Node { expr, val }
}

fn mul(lhs: Node, rhs: Node) -> Node {
    let val = lhs.val * rhs.val;
    let expr = Expr::Mul(Box::new(lhs), Box::new(rhs));
    Node { expr, val }
}

#[test]
fn calculator_test() {
    assert_eq!(Node::parse("(3 + 4) * (5 + 6) n").unwrap().val, 77);
    assert_eq!(Node::parse("1 * 2 * 3 * (4 * 5) n").unwrap().val, 120);
    assert_eq!(Node::parse("(9 + 8 * (7 + 6) + 5) n").unwrap().val, 118);
}
