use crate::{build::*, Expr, Symbol};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Literal {
    Neg(Symbol),
    Pos(Symbol),
}

impl Literal {
    fn to_expr(&self) -> Expr {
        match self {
            Neg(s) => not(var(*s)),
            Pos(s) => var(*s),
        }
    }
}

use Literal::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    fn new(literals: Vec<Literal>) -> Self {
        Clause { literals }
    }

    fn new_pos(s: Symbol) -> Self {
        Clause::new(vec![Pos(s)])
    }

    fn new_neg(s: Symbol) -> Self {
        Clause::new(vec![Neg(s)])
    }

    fn new_empty() -> Self {
        Clause::new(vec![])
    }

    fn normalize(mut self) -> Self {
        self.literals.sort();
        self.literals.dedup();
        self
    }

    fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    fn join(&self, other: &Self) -> Self {
        if self.is_empty() || other.is_empty() {
            Clause::new_empty()
        } else {
            let mut literals = self.literals.to_vec();
            literals.extend(other.literals.iter().cloned());
            Clause::new(literals).normalize()
        }
    }

    fn to_expr(&self) -> Expr {
        if self.is_empty() {
            f()
        } else {
            let first = self.literals[0].to_expr();
            self.literals
                .iter()
                .skip(1)
                .fold(first, |expr, lit| or(expr, lit.to_expr()))
        }
    }
}

pub struct CNF {
    clauses: Vec<Clause>,
}

impl CNF {
    fn new(clauses: Vec<Clause>) -> Self {
        CNF { clauses }.normalize()
    }

    fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    fn normalize(mut self) -> Self {
        self.clauses.sort();
        self.clauses.dedup();
        self
    }

    pub fn to_expr(&self) -> Expr {
        if self.is_empty() {
            t()
        } else {
            let first = self.clauses[0].to_expr();
            self.clauses
                .iter()
                .skip(1)
                .fold(first, |expr, clause| and(expr, clause.to_expr()))
        }
    }
}

pub trait ToCNF {
    fn to_clauses(&self) -> Vec<Clause>;

    fn to_cnf(&self) -> CNF {
        CNF::new(self.to_clauses())
    }
}

impl ToCNF for Expr {
    fn to_clauses(&self) -> Vec<Clause> {
        use Expr::*;

        match self.to_cnf_expr() {
            True => vec![],
            False => vec![Clause::new_empty()],
            Var(s) => vec![Clause::new_pos(s)],
            Not(v) => {
                if let Var(s) = *v {
                    vec![Clause::new_neg(s)]
                } else {
                    unreachable!()
                }
            }
            And(lhs, rhs) => {
                let mut clauses = lhs.to_clauses();
                clauses.extend(rhs.to_clauses());
                clauses
            }
            Or(lhs, rhs) => {
                let lhs_clauses = lhs.to_clauses();
                let rhs_clauses = rhs.to_clauses();
                if lhs_clauses.is_empty() {
                    rhs_clauses
                } else if rhs_clauses.is_empty() {
                    lhs_clauses
                } else {
                    vec![lhs_clauses[0].join(&rhs_clauses[0])]
                }
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn cnf_test() {
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
            expr.equivalent(&expr.to_cnf().to_expr()),
            "{}th expr is not equivalent to its CNF",
            i + 1
        );
    }
}