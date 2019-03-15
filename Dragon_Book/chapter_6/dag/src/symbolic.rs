use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

// would be unnecessary if the syntax of LALRPOP is more flexible
thread_local! {
    static EXPRS: RefCell<HashSet<Rc<Expr>>> = RefCell::new(HashSet::new());
}

lalrpop_mod!(pub infix);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum UnOp {
    Neg,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Expr {
    Bin(BinOp, Rc<Expr>, Rc<Expr>),
    Un(UnOp, Rc<Expr>),
    Var(String),
}

impl Expr {
    fn dedup(self) -> Rc<Self> {
        let expr = Rc::new(self);
        EXPRS.with(move |exprs| {
            let mut borrowed = exprs.borrow_mut();
            if let Some(v) = borrowed.get(&expr) {
                v.clone()
            } else {
                borrowed.insert(expr.clone());
                expr
            }
        })
    }

    pub fn bin(op: BinOp, lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Self> {
        Expr::Bin(op, lhs, rhs).dedup()
    }

    pub fn un(op: UnOp, inner: Rc<Expr>) -> Rc<Self> {
        Expr::Un(op, inner).dedup()
    }

    pub fn var(s: String) -> Rc<Self> {
        Expr::Var(s).dedup()
    }

    fn format(&self, map: &ExprMap, f: &mut Formatter) -> Result<(), fmt::Error> {
        let id = map[self];
        match self {
            Expr::Bin(op, lhs, rhs) => {
                let lhs_id = map[lhs];
                let rhs_id = map[rhs];
                writeln!(f, "{}: {:?}({}, {})", id, op, lhs_id, rhs_id)
            }
            Expr::Un(op, inner) => {
                let inner_id = map[inner];
                writeln!(f, "{}: {:?}({})", id, op, inner_id)
            }
            Expr::Var(var) => writeln!(f, "{}: {}", id, var),
        }
    }
}

type ExprMap = HashMap<Rc<Expr>, usize>;

pub struct DAG {
    top: Rc<Expr>,
    map: ExprMap,
}

impl DAG {
    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        EXPRS.with(|exprs| {
            exprs.borrow_mut().clear();
        });
        let top = infix::EParser::new().parse(s)?;
        let map = EXPRS
            .with(|exprs| exprs.replace(HashSet::new()))
            .into_iter()
            .zip(0..)
            .collect();

        Ok(DAG { top, map })
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }
}

impl Debug for DAG {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "top node: {}", self.map[&self.top])?;
        let mut pairs: Vec<_> = self.map.iter().collect();
        pairs.sort_by_key(|(_, v)| *v);
        for (expr, _) in pairs {
            expr.format(&self.map, f)?;
        }
        Ok(())
    }
}

#[test]
fn dedup_test() {
    let dag = DAG::parse("a+a*(b-c)+(b-c)*d").unwrap();
    assert_eq!(dag.size(), 9);
}
