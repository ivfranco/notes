pub mod cnf;

use std::fmt::{self, Formatter};

pub type Symbol = usize;

#[derive(Clone)]
pub enum Expr {
    True,
    False,
    Var(Symbol),
    Not(Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Imply(Box<Self>, Box<Self>),
    Iff(Box<Self>, Box<Self>),
}

use Expr::*;

pub mod build {
    use super::*;

    pub fn t() -> Expr {
        True
    }

    pub fn f() -> Expr {
        False
    }

    pub fn var(var: usize) -> Expr {
        Var(var)
    }

    pub fn not(s: Expr) -> Expr {
        Not(Box::new(s))
    }

    pub fn and(lhs: Expr, rhs: Expr) -> Expr {
        And(Box::new(lhs), Box::new(rhs))
    }

    pub fn or(lhs: Expr, rhs: Expr) -> Expr {
        Or(Box::new(lhs), Box::new(rhs))
    }

    pub fn imply(lhs: Expr, rhs: Expr) -> Expr {
        Imply(Box::new(lhs), Box::new(rhs))
    }

    pub fn iff(lhs: Expr, rhs: Expr) -> Expr {
        Iff(Box::new(lhs), Box::new(rhs))
    }
}

impl Expr {
    pub fn partial_truth(&self, model: &[bool]) -> Option<bool> {
        match self {
            True => Some(true),
            False => Some(false),
            Var(var) => model.get(*var).cloned(),
            Not(e) => e.partial_truth(model).map(|t| !t),
            And(lhs, rhs) => match (lhs.partial_truth(model), rhs.partial_truth(model)) {
                (Some(true), Some(true)) => Some(true),
                (Some(false), _) => Some(false),
                (_, Some(false)) => Some(false),
                _ => None,
            },
            Or(lhs, rhs) => match (lhs.partial_truth(model), rhs.partial_truth(model)) {
                (Some(true), _) => Some(true),
                (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            },
            Imply(lhs, rhs) => match (lhs.partial_truth(model), rhs.partial_truth(model)) {
                (Some(true), Some(false)) => Some(false),
                (Some(false), _) => Some(true),
                (_, Some(true)) => Some(true),
                _ => None,
            },
            Iff(lhs, rhs) => match (lhs.partial_truth(model), rhs.partial_truth(model)) {
                (Some(l), Some(r)) if l == r => Some(true),
                (Some(l), Some(r)) if l != r => Some(false),
                _ => None,
            },
        }
    }

    pub fn is_taotology(&self) -> bool {
        self.check_all(&mut vec![])
    }

    fn check_all(&self, model: &mut Vec<bool>) -> bool {
        if let Some(t) = self.partial_truth(model) {
            t
        } else {
            model.push(true);
            let true_check = self.check_all(model);
            model.pop();
            model.push(false);
            let false_check = self.check_all(model);
            model.pop();
            true_check && false_check
        }
    }

    pub fn equivalent(&self, other: &Self) -> bool {
        build::iff(self.clone(), other.clone()).is_taotology()
    }

    fn to_nnf_expr(&self) -> Expr {
        use build::*;

        match self {
            True | False | Var(..) => self.clone(),
            Not(e) => match e.as_ref() {
                True => False,
                False => True,
                Var(..) => self.clone(),
                Not(inn) => inn.to_nnf_expr(),
                And(lhs, rhs) => or(not(*lhs.clone()).to_nnf_expr(), not(*rhs.clone()).to_nnf_expr()),
                Or(lhs, rhs) => and(not(*lhs.clone()).to_nnf_expr(), not(*rhs.clone()).to_nnf_expr()),
                Imply(lhs, rhs) => not(or(not(*lhs.clone()), *rhs.clone())).to_nnf_expr(),
                Iff(lhs, rhs) => not(and(
                    imply(*lhs.clone(), *rhs.clone()),
                    imply(*rhs.clone(), *lhs.clone()),
                ))
                .to_nnf_expr(),
            },
            And(lhs, rhs) => and(lhs.to_nnf_expr(), rhs.to_nnf_expr()),
            Or(lhs, rhs) => or(lhs.to_nnf_expr(), rhs.to_nnf_expr()),
            Imply(lhs, rhs) => or(not(*lhs.clone()), *rhs.clone()).to_nnf_expr(),
            Iff(lhs, rhs) => and(
                imply(*lhs.clone(), *rhs.clone()),
                imply(*rhs.clone(), *lhs.clone()),
            )
            .to_nnf_expr(),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            True | False | Var(..) => Precedence::Lit,
            Not(..) => Precedence::Not,
            And(..) | Or(..) => Precedence::AndOr,
            Imply(..) | Iff(..) => Precedence::ImpIff,
        }
    }

    fn distribute_ors(&self) -> Expr {
        use build::*;

        match self {
            And(lhs, rhs) => and(lhs.distribute_ors(), rhs.distribute_ors()),
            Or(lhs, rhs) => {
                if let And(p, q) = rhs.as_ref() {
                    and(
                        or(*lhs.clone(), *p.clone()).distribute_ors(),
                        or(*lhs.clone(), *q.clone()).distribute_ors(),
                    )
                } else if let And(p, q) = lhs.as_ref() {
                    and(
                        or(*p.clone(), *rhs.clone()).distribute_ors(),
                        or(*q.clone(), *rhs.clone()).distribute_ors(),
                    )
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }

    pub(crate) fn to_cnf_expr(&self) -> Expr {
        self.to_nnf_expr().distribute_ors()
    }

    fn format(&self, parent: Precedence, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.precedence() <= parent {
            write!(f, "(")?;
        }

        fn to_symbol(i: usize) -> char {
            use std::convert::TryInto;

            const A: u32 = b'A' as u32;
            (A + i as u32).try_into().unwrap()
        }

        fn binary(
            lhs: &Expr,
            rhs: &Expr,
            prec: Precedence,
            symbol: &str,
            f: &mut Formatter,
        ) -> Result<(), fmt::Error> {
            lhs.format(prec, f)?;
            write!(f, " {} ", symbol)?;
            rhs.format(prec, f)?;
            Ok(())
        }

        match self {
            True => write!(f, "True")?,
            False => write!(f, "False")?,
            Var(i) => write!(f, "{}", to_symbol(*i))?,
            Not(e) => {
                write!(f, "~")?;
                e.format(Precedence::Not, f)?;
            }
            And(lhs, rhs) => binary(lhs, rhs, Precedence::AndOr, "&", f)?,
            Or(lhs, rhs) => binary(lhs, rhs, Precedence::AndOr, "|", f)?,
            Imply(lhs, rhs) => binary(lhs, rhs, Precedence::ImpIff, "=>", f)?,
            Iff(lhs, rhs) => binary(lhs, rhs, Precedence::ImpIff, "<=>", f)?,
        }

        if self.precedence() <= parent {
            write!(f, ")")?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Precedence {
    Top,
    ImpIff,
    AndOr,
    Not,
    Lit,
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.format(Precedence::Top, f)
    }
}

pub struct Sentence {
    expr: Expr,
    vars: usize,
}

impl Sentence {
    pub fn new(expr: Expr, vars: usize) -> Self {
        Sentence { expr, vars }
    }

    pub fn partial_truth(&self, model: &[bool]) -> Option<bool> {
        self.expr.partial_truth(model)
    }

    pub fn truth(&self, model: &[bool]) -> bool {
        assert!(model.len() >= self.vars);

        self.partial_truth(model).unwrap()
    }

    pub fn is_taotology(&self) -> bool {
        self.expr.is_taotology()
    }
}

#[test]
fn nnf_test() {
    use build::*;

    for (i, expr) in vec![
        iff(and(var(0), var(1)), and(var(1), var(0))),
        iff(or(var(0), var(1)), or(var(1), var(0))),
        iff(
            and(and(var(0), var(1)), var(2)),
            and(var(0), and(var(1), var(2))),
        ),
        iff(
            or(or(var(0), var(1)), var(2)),
            or(var(0), or(var(1), var(2))),
        ),
        iff(imply(var(0), var(1)), imply(not(var(1)), not(var(0)))),
        iff(imply(var(0), var(1)), or(not(var(0)), var(1))),
        iff(
            iff(var(0), var(1)),
            and(imply(var(0), var(1)), imply(var(1), var(0))),
        ),
        iff(not(and(var(0), var(1))), or(not(var(0)), not(var(1)))),
        iff(not(or(var(0), var(1))), and(not(var(0)), not(var(1)))),
        iff(
            and(var(0), or(var(1), var(2))),
            or(and(var(0), var(1)), and(var(0), var(2))),
        ),
        iff(
            or(var(0), and(var(1), var(2))),
            and(or(var(0), var(1)), or(var(0), var(2))),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        assert!(
            expr.equivalent(&expr.to_nnf_expr()),
            "{}th expr is not equivalent to its NNF",
            i + 1
        );
    }
}
