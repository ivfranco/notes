use crate::builder::Builder;
pub use crate::machine_code::{BinOp, Binary};
use std::error::Error;
use std::fmt::{self, Debug, Formatter};

lalrpop_mod!(pub ir);

pub type Var = String;
pub type Label = String;

#[derive(PartialEq)]
pub enum RValue {
    Lit(usize),
    Var(Var),
}

impl RValue {
    pub fn is(&self, var: &str) -> bool {
        if let RValue::Var(v) = self {
            v == var
        } else {
            false
        }
    }
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

    fn jump_target(&self) -> Option<&Label> {
        match self {
            IR::Goto(label) => Some(label),
            IR::If(_, _, _, label) => Some(label),
            _ => None,
        }
    }

    fn jump_target_mut(&mut self) -> Option<&mut Label> {
        match self {
            IR::Goto(label) => Some(label),
            IR::If(_, _, _, label) => Some(label),
            _ => None,
        }
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
    pub labels: Vec<Label>,
    pub ir: IR,
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
                .map(Label::as_str)
                .collect::<Vec<_>>()
                .join(", ");

            write!(f, "{}: {:?}", prefix, self.ir)
        }
    }
}

pub struct Program {
    pub lines: Vec<Line>,
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

    fn len(&self) -> usize {
        self.lines.len()
    }

    fn find_label(&self, label: &str) -> Option<&IR> {
        self.lines
            .iter()
            .find(|line| line.labels.iter().any(|l| l == label))
            .map(|line| &line.ir)
    }

    pub fn build(&self) -> Binary {
        let mut builder = Builder::new();

        for ir in self.lines.iter().map(|line| &line.ir) {
            builder.gen(ir);
        }

        builder.seal()
    }
}

#[allow(dead_code)]
fn flow_of_control(mut program: Program) -> Program {
    let mut i = 0;

    while i < program.len() {
        if let Some(j) = program.lines[i].ir.jump_target() {
            let ir = program.find_label(j).unwrap();
            if let IR::Goto(k) = ir {
                *program.lines[i].ir.jump_target_mut().unwrap() = k.to_string();
            }
        }
        i += 1;
    }

    program
}

#[allow(dead_code)]
fn algebraic(program: Program) -> Program {
    fn algebraic_noop(ir: &IR) -> bool {
        match ir {
            IR::Op(_, lhs, BinOp::Add, rhs) => lhs == &RValue::Lit(0) || rhs == &RValue::Lit(0),
            IR::Op(_, lhs, BinOp::Mul, rhs) => lhs == &RValue::Lit(1) || rhs == &RValue::Lit(1),
            _ => false,
        }
    }

    let lines = program
        .lines
        .into_iter()
        .filter(|line| !algebraic_noop(&line.ir))
        .collect();

    Program { lines }
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

#[test]
fn flow_of_control_test() {
    let program = "
goto L0;
L0: goto L1;
L1: if i > n goto L2;
L2: goto L3;
L3: ;
    ";

    let p = flow_of_control(Program::parse(program).unwrap());
    // println!("{:?}", p);
    assert_eq!(p.lines[0].ir.jump_target().unwrap(), "L1");
    assert_eq!(p.lines[3].ir.jump_target().unwrap(), "L3");
}

#[test]
fn algebraic_test() {
    let program = "
x = x + 0;
x = x * 1;
x = 2;
    ";

    let p = algebraic(Program::parse(program).unwrap());
    // println!("{:?}", p);
    assert_eq!(p.lines.len(), 1);
}
