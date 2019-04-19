use crate::framework::{Attrs, DataFlow, Forward, SemiLattice, Transfer};
use crate::{Block, BlockID, Lit, Program, RValue, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Value {
    Undef,
    Nac,
    Cst(Lit),
}

impl Value {
    fn meet(self, other: Self) -> Self {
        use Value::*;
        match (self, other) {
            (Cst(a), Cst(b)) => {
                if a == b {
                    Cst(a)
                } else {
                    Nac
                }
            }
            (Nac, _) => Nac,
            (_, Nac) => Nac,
            _ => Undef,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constants<'a> {
    map: HashMap<&'a str, Value>,
}

impl<'a> Constants<'a> {
    fn eval(&self, rvalue: &'a RValue) -> Value {
        match rvalue {
            RValue::Lit(lit) => Value::Cst(*lit),
            RValue::Var(var) => self.map[var.as_str()],
        }
    }

    fn eval_rhs(&self, stmt: &'a Stmt) -> Value {
        use Stmt::*;
        use Value::*;

        match stmt {
            Copy(_, src) => self.eval(src),
            Op(_, lhs, op, rhs) => {
                let l = self.eval(lhs);
                let r = self.eval(rhs);

                match (l, r) {
                    (Cst(a), Cst(b)) => Cst(op.apply(a, b)),
                    _ => l.meet(r),
                }
            }
        }
    }

    fn update(&mut self, stmt: &'a Stmt) {
        if let Some(def) = stmt.def() {
            *self.map.get_mut(def).unwrap() = self.eval_rhs(stmt);
        }
    }

    #[allow(dead_code)]
    fn get(&self, var: &str) -> Value {
        self.map.get(var).cloned().unwrap_or(Value::Undef)
    }
}

impl<'a> SemiLattice<'a> for Constants<'a> {
    fn top(program: &'a Program) -> Self {
        let map = program
            .stmts()
            .flat_map(|(_, stmt)| stmt.def().into_iter().chain(stmt.uses()))
            .map(|def| (def, Value::Undef))
            .collect();
        Constants { map }
    }

    fn start(program: &'a Program) -> Self {
        Self::top(program)
    }

    fn meet(&self, other: &Self) -> Self {
        let map = self
            .map
            .iter()
            .map(|(var, value)| (*var, value.meet(other.map[var])))
            .collect();
        Constants { map }
    }
}

#[derive(Clone)]
pub struct RefBlock<'a> {
    block: &'a Block,
}

impl<'a> Transfer<'a> for RefBlock<'a> {
    type Target = Constants<'a>;
    type Extra = ();

    fn new(block_id: BlockID, program: &'a Program, _: &'a ()) -> Self {
        let block = program
            .get_block(block_id)
            .expect("RefBlock: Block in-bound");
        RefBlock { block }
    }

    fn apply(&self, constants: &Self::Target) -> Self::Target {
        self.block
            .stmts()
            .map(|(_, stmt)| stmt)
            .fold(constants.clone(), |mut csts, stmt| {
                csts.update(stmt);
                csts
            })
    }
}

pub fn constant_propagation(program: &Program) -> Attrs<Constants<'_>, Forward, RefBlock<'_>, ()> {
    DataFlow::run(program, &())
}

#[cfg(test)]
fn figure_9_27() -> Program {
    Program::with_entry_exit(
        vec![
            Block::parse(1, "x = 2\ny = 3"),
            Block::parse(3, "x = 3\ny = 2"),
            Block::parse(5, "z = x + y"),
        ],
        &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4)],
    )
}

#[test]
fn meet_test() {
    use Value::*;

    let ca = Constants {
        map: vec![("x", Cst(0)), ("y", Cst(1)), ("z", Undef)]
            .into_iter()
            .collect(),
    };
    let cb = Constants {
        map: vec![("x", Cst(0)), ("y", Cst(2)), ("z", Cst(3))]
            .into_iter()
            .collect(),
    };
    let meet = ca.meet(&cb);

    assert_eq!(meet.get("x"), Cst(0));
    assert_eq!(meet.get("y"), Nac);
    assert_eq!(meet.get("z"), Undef);
}

#[test]
fn constant_propagation_test() {
    let program = figure_9_27();
    let attrs = constant_propagation(&program).attrs;

    assert_eq!(attrs[3].out_value().unwrap().get("x"), Value::Nac);
    assert_eq!(attrs[3].out_value().unwrap().get("y"), Value::Nac);
    assert_eq!(attrs[3].out_value().unwrap().get("z"), Value::Nac);
}
