type Symbol = usize;

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
            Not(s) => s.partial_truth(model).map(|t| !t),
            And(lhs, rhs) => match (lhs.partial_truth(model), rhs.partial_truth(model)) {
                (Some(true), Some(true)) => Some(true),
                (Some(false), _) => Some(false),
                (_, Some(false)) => Some(false),
                _ => None,
            }
            Or(lhs, rhs) => match (lhs.partial_truth(model), rhs.partial_truth(model)) {
                (Some(true), _) => Some(true),
                (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            }
            Imply(lhs, rhs) => match (lhs.partial_truth(model), rhs.partial_truth(model)) {
                (Some(true), Some(false)) => Some(false),
                (Some(false), _) => Some(true),
                (_, Some(true)) => Some(true),
                _ => None,
            }
            Iff(lhs, rhs) => match (lhs.partial_truth(model), rhs.partial_truth(model)) {
                (Some(l), Some(r)) if l == r => Some(true),
                (Some(l), Some(r)) if l != r => Some(false),
                _ => None,
            }
        }
    }
}

// fn to_greek(i: usize) -> char {
//     use std::convert::TryInto;

//     let a = 0x3b1;
//     (a + i as u32).try_into().unwrap()
// }

// impl std::fmt::Debug for Expr {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         match self {
//             True => write!(f, "True"),
//             False => write!(f, "False"),
//             Var(i) => write!(f, "{}", to_greek(*i)),
//             Not(e) => write!(f, "¬({:?})", e),
//         }
//     }
// }

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

}