use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Mul};

lalrpop_mod!(pub infix);

const LEVEL: usize = 4;

#[derive(Clone)]
pub enum Expr {
    Mul(Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Lit(i32),
    Id(String),
}

impl Expr {
    fn tag(&self) -> &'static str {
        match self {
            Expr::Mul(..) => "Mul",
            Expr::Add(..) => "Add",
            Expr::Lit(..) => "Lit",
            Expr::Id(..) => "Id",
        }
    }

    fn is_add(&self) -> bool {
        if let Expr::Add(..) = self {
            true
        } else {
            false
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

fn paren(expr: &Expr, f: &mut Formatter) -> Result<(), fmt::Error> {
    if expr.is_add() {
        write!(f, "({:?})", expr)
    } else {
        write!(f, "{:?}", expr)
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use Expr::*;
        match self {
            Mul(lhs, rhs) => {
                paren(&lhs.expr, f)?;
                write!(f, " * ")?;
                paren(&rhs.expr, f)
            }
            Add(lhs, rhs) => write!(f, "{:?} + {:?}", lhs.expr, rhs.expr),
            Lit(lit) => write!(f, "{}", lit),
            Id(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub expr: Expr,
    pub val: i32,
}

impl Node {
    pub fn lit(val: i32) -> Self {
        Node {
            expr: Expr::Lit(val),
            val,
        }
    }

    pub fn id(s: String) -> Self {
        Node {
            expr: Expr::Id(s),
            val: 0,
        }
    }

    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        infix::LParser::new().parse(s).map_err(|e| e.into())
    }

    pub fn diff(&self) -> Self {
        match &self.expr {
            Expr::Add(lhs, rhs) => lhs.diff() + rhs.diff(),
            Expr::Mul(lhs, rhs) => {
                let l: &Node = &*lhs;
                let r: &Node = &*rhs;
                l.clone() * r.diff() + l.diff() * r.clone()
            }
            Expr::Lit(_) => Node::lit(0),
            Expr::Id(_) => Node::lit(1),
        }
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

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Node) -> Node {
        let lhs = self;
        let val = lhs.val + rhs.val;
        let expr = Expr::Add(Box::new(lhs), Box::new(rhs));
        Node { expr, val }
    }
}

impl Mul for Node {
    type Output = Node;

    fn mul(self, rhs: Node) -> Node {
        let lhs = self;
        let val = lhs.val * rhs.val;
        let expr = Expr::Mul(Box::new(lhs), Box::new(rhs));
        Node { expr, val }
    }
}

#[test]
fn calculator_test() {
    assert_eq!(Node::parse("(3 + 4) * (5 + 6) n").unwrap().val, 77);
    assert_eq!(Node::parse("1 * 2 * 3 * (4 * 5) n").unwrap().val, 120);
    assert_eq!(Node::parse("(9 + 8 * (7 + 6) + 5) n").unwrap().val, 118);
}

#[test]
fn unnecessary_paren_test() {
    let expr = Node::parse("((a*(b+c))*(d)) n").unwrap().expr;
    assert_eq!(format!("{:?}", expr), "a * (b + c) * d");
}

#[test]
fn diff_test() {
    let node = Node::parse("3 * x n").unwrap();
    assert_eq!(format!("{:?}", node.diff().expr), "3 * 1 + 0 * x");
}
