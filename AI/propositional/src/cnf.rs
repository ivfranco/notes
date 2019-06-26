use crate::{build::*, to_symbol, Expr, Symbol};
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Neg(Symbol),
    Pos(Symbol),
}

use Literal::*;

impl Literal {
    fn symbol(&self) -> Symbol {
        match self {
            Neg(s) => *s,
            Pos(s) => *s,
        }
    }

    fn to_expr(&self) -> Expr {
        match self {
            Neg(s) => not(var(*s)),
            Pos(s) => var(*s),
        }
    }

    fn partial_truth(&self, model: &Model) -> Option<bool> {
        match self {
            Pos(s) => model.get(*s),
            Neg(s) => model.get(*s).map(|b| !b),
        }
    }
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Pos(s) => write!(f, "{}", to_symbol(*s)),
            Neg(s) => write!(f, "~{}", to_symbol(*s)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CNFClause {
    literals: Vec<Literal>,
}

impl CNFClause {
    fn new(literals: Vec<Literal>) -> Self {
        CNFClause { literals }.normalize()
    }

    fn new_pos(s: Symbol) -> Self {
        CNFClause::new(vec![Pos(s)])
    }

    fn new_neg(s: Symbol) -> Self {
        CNFClause::new(vec![Neg(s)])
    }

    fn new_empty() -> Self {
        CNFClause::new(vec![])
    }

    fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    fn normalize(self) -> Self {
        let mut literals = self.literals;
        literals.sort();
        literals.dedup();
        CNFClause { literals }
    }

    fn join(&self, other: &Self) -> Self {
        if self.is_empty() || other.is_empty() {
            CNFClause::new_empty()
        } else {
            let mut literals = self.literals.to_vec();
            literals.extend(other.literals.iter().cloned());
            CNFClause::new(literals).normalize()
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

    fn literals<'a>(&'a self) -> impl Iterator<Item = Literal> + 'a {
        self.literals.iter().cloned()
    }

    fn partial_truth(&self, model: &Model) -> Option<bool> {
        self.literals()
            .try_fold(Some(false), |truth, lit| match lit.partial_truth(model) {
                Some(true) => Err(()),
                Some(false) => Ok(truth),
                None => Ok(None),
            })
            .unwrap_or(Some(true))
    }

    fn collect_symbols(&self, symbols: &mut HashSet<Symbol>) {
        for lit in self.literals() {
            symbols.insert(lit.symbol());
        }
    }
}

impl std::fmt::Debug for CNFClause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({:?})", self.literals.iter().format(" | "))
    }
}

pub struct CNF {
    clauses: Vec<CNFClause>,
}

impl CNF {
    fn new(clauses: Vec<CNFClause>) -> Self {
        CNF { clauses }.normalize()
    }

    fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    fn normalize(self) -> Self {
        let mut clauses = self.clauses;

        clauses.sort();
        clauses.dedup();

        CNF { clauses }
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

    fn clauses<'a>(&'a self) -> impl Iterator<Item = &'a CNFClause> + 'a {
        self.clauses.iter()
    }

    fn partial_truth(&self, model: &Model) -> Option<bool> {
        self.clauses()
            .try_fold(Some(true), |truth, clause| {
                match clause.partial_truth(model) {
                    Some(false) => Err(()),
                    Some(true) => Ok(truth),
                    None => Ok(None),
                }
            })
            .unwrap_or(Some(false))
    }

    pub fn satisfiable(&self) -> bool {
        let mut model = Model::new(self);
        dpll(self, &mut model)
    }
}

struct Model {
    assignment: BTreeMap<Symbol, bool>,
    unassigned: HashSet<Symbol>,
}

impl Model {
    fn new(cnf: &CNF) -> Self {
        let mut unassigned = HashSet::new();
        for clause in cnf.clauses() {
            clause.collect_symbols(&mut unassigned);
        }
        Model {
            assignment: BTreeMap::new(),
            unassigned,
        }
    }

    fn get(&self, symbol: Symbol) -> Option<bool> {
        self.assignment.get(&symbol).copied()
    }

    fn insert(&mut self, symbol: Symbol, t: bool) {
        self.assignment.insert(symbol, t);
        self.unassigned.remove(&symbol);
    }

    fn contains(&self, symbol: Symbol) -> bool {
        self.assignment.contains_key(&symbol)
    }

    fn remove(&mut self, symbol: Symbol) {
        self.assignment.remove(&symbol);
        self.unassigned.insert(symbol);
    }

    fn next_symbol(&self) -> Option<Symbol> {
        self.unassigned.iter().next().copied()
    }
}

impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{{{}}}",
            self.assignment
                .iter()
                .map(|(s, t)| format!("{} = {}", to_symbol(*s), t))
                .format(", ")
        )
    }
}

fn find_pure_symbol(cnf: &CNF, model: &Model) -> Option<(Symbol, bool)> {
    #[derive(Clone, Copy, PartialEq)]
    enum State {
        PurePos,
        PureNeg,
        Both,
    }
    use State::*;

    let mut states: BTreeMap<Symbol, State> = BTreeMap::new();

    for clause in cnf
        .clauses()
        .filter(|clause| clause.partial_truth(model) == Some(true))
    {
        for lit in clause.literals() {
            match lit {
                Pos(s) => {
                    if states.get(&s) == Some(&PureNeg) {
                        states.insert(s, Both);
                    } else {
                        states.insert(s, PurePos);
                    }
                }
                Neg(s) => {
                    if states.get(&s) == Some(&PurePos) {
                        states.insert(s, Both);
                    } else {
                        states.insert(s, PurePos);
                    }
                }
            }
        }
    }

    states
        .into_iter()
        .filter(|(symbol, _)| !model.contains(*symbol))
        .find_map(|(symbol, state)| match state {
            PurePos => Some((symbol, true)),
            PureNeg => Some((symbol, false)),
            _ => None,
        })
}

fn find_unique_clause(cnf: &CNF, model: &Model) -> Option<(Symbol, bool)> {
    for clause in cnf.clauses() {
        let unique = clause
            .literals()
            .filter(|lit| !model.contains(lit.symbol()))
            .try_fold(None, |unique, lit| match unique {
                Some(_) if lit.partial_truth(model).is_none() => None,
                Some(_) => Some(unique),
                None => Some(Some(lit)),
            })
            .unwrap_or(None);

        if let Some(lit) = unique {
            match lit {
                Pos(s) => return Some((s, true)),
                Neg(s) => return Some((s, false)),
            }
        }
    }

    None
}

fn dpll(cnf: &CNF, model: &mut Model) -> bool {
    println!("{:?}", model);

    fn backtrack(symbol: Symbol, t: bool, cnf: &CNF, model: &mut Model) -> bool {
        model.insert(symbol, t);
        let ret = dpll(cnf, model);
        model.remove(symbol);
        ret
    }

    if let Some(t) = cnf.partial_truth(model) {
        t
    } else if let Some((symbol, t)) = find_pure_symbol(cnf, model) {
        println!("chosen {} = {} by pure symbol heuristic", to_symbol(symbol), t);
        backtrack(symbol, t, cnf, model)
    } else if let Some((symbol, t)) = find_unique_clause(cnf, model) {
        println!("chosen {} = {} by unique clause heuristic", to_symbol(symbol), t);
        backtrack(symbol, t, cnf, model)
    } else if let Some(symbol) = model.next_symbol() {
        let true_ret = backtrack(symbol, true, cnf, model);
        let false_ret = backtrack(symbol, true, cnf, model);
        true_ret && false_ret
    } else {
        unreachable!()
    }
}

impl std::fmt::Debug for CNF {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.clauses.iter().format(" & "))
    }
}

pub trait ToCNF {
    fn to_clauses(&self) -> Vec<CNFClause>;

    fn to_cnf(&self) -> CNF {
        CNF::new(self.to_clauses())
    }
}

impl ToCNF for Expr {
    fn to_clauses(&self) -> Vec<CNFClause> {
        use Expr::*;

        match self.to_cnf_expr() {
            True => vec![],
            False => vec![CNFClause::new_empty()],
            Var(s) => vec![CNFClause::new_pos(s)],
            Not(v) => {
                if let Var(s) = *v {
                    vec![CNFClause::new_neg(s)]
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

#[test]
fn dpll_test() {
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
        let cnf = (!expr).to_cnf();
        assert!(
            !cnf.satisfiable(),
            "{}th expr is not equivalent to its CNF",
            i + 1
        );
    }
}
