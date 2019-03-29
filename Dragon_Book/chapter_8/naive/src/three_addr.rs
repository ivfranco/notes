use crate::builder::Builder;
pub use crate::machine_code::{BinOp, Binary};
use std::error::Error;
use std::fmt::{self, Debug, Formatter};

lalrpop_mod!(pub ir);

type Var = String;
pub type Label = String;

pub enum RValue {
    Lit(usize),
    Var(Var),
}

impl Debug for RValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            RValue::Lit(lit) => write!(f, "{}", lit),
            RValue::Var(var) => write!(f, "{}", var),
        }
    }
}

pub enum RelOp {
    Gt,
    Lt,
}

impl RelOp {
    fn symbol(&self) -> &'static str {
        match self {
            RelOp::Gt => ">",
            RelOp::Lt => "<",
        }
    }
}

pub enum IR {
    ArrayAccess(Var, Var, RValue),
    ArrayAssign(Var, RValue, RValue),
    RefAccess(Var, Var),
    RefAssign(Var, Var),
    Op(Var, RValue, BinOp, RValue),
    Copy(Var, RValue),
    Goto(Label),
    If(RValue, RelOp, RValue, Label),
    Noop,
}

impl IR {
    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        ir::IRParser::new().parse(s).map_err(Box::from)
    }
}

impl Debug for IR {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use self::IR::*;

        match self {
            ArrayAccess(dst, src, idx) => write!(f, "{} = {}[{:?}]", dst, src, idx),
            ArrayAssign(dst, idx, src) => write!(f, "{}[{:?}] = {:?}", dst, idx, src),
            RefAccess(dst, src) => write!(f, "{} = *{}", dst, src),
            RefAssign(dst, src) => write!(f, "*{} = {}", dst, src),
            Op(dst, lhs, op, rhs) => write!(f, "{} = {:?} {} {:?}", dst, lhs, op.symbol(), rhs),
            Copy(dst, src) => write!(f, "{} = {:?}", dst, src),
            Goto(label) => write!(f, "goto {}", label),
            If(lhs, op, rhs, label) => {
                write!(f, "if {:?} {} {:?} goto {}", lhs, op.symbol(), rhs, label)
            }
            Noop => write!(f, "noop"),
        }
    }
}

pub struct Line {
    labels: Vec<Label>,
    ir: IR,
}

impl Line {
    fn new(labels: Vec<Label>, ir: IR) -> Self {
        Line { labels, ir }
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.labels.is_empty() {
            write!(f, "{:?}", self.ir)
        } else {
            let prefix = self
                .labels
                .iter()
                .map(|label| label.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            write!(f, "{}: {:?}", prefix, self.ir)
        }
    }
}

pub struct Program {
    lines: Vec<Line>,
}

impl Program {
    pub fn build(&self) -> Binary {
        let mut builder = Builder::new();

        for ir in self.lines.iter().map(|line| &line.ir) {
            builder.gen(ir);
        }

        builder.seal()
    }
}

impl Debug for Program {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for line in &self.lines {
            writeln!(f, "{:?}", line)?;
        }
        Ok(())
    }
}

impl Program {
    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        ir::ProgParser::new().parse(s).map_err(Box::from)
    }
}

#[test]
fn parse_test() {
    let program = "s = 0;
i = 0;
L1: if i > n goto L2;
s = s + i;
i = i + 1;
goto L1;
L2:;";

    let p = Program::parse(program).unwrap();
    // println("{:?}", p);
    assert_eq!(p.lines.len(), 7);
}
