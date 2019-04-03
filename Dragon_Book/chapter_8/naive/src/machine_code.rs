use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

lalrpop_mod!(pub code);

type Var = String;
pub type Reg = usize;
type Label = String;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Idx {
    Lit(usize),
    Var(Var),
}

impl Debug for Idx {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Idx::Lit(i) => write!(f, "{}", i),
            Idx::Var(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Word {
    Reg(Reg),
    Lit(usize),
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Word::Reg(r) => write!(f, "R{}", r),
            Word::Lit(l) => write!(f, "#{}", l),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Addr {
    LValue(Var),
    Indexed(Idx, Reg),
    Deref(usize, Reg),
    Imm(usize),
    Reg(Reg),
}

impl Addr {
    pub fn reg(r: Reg) -> Self {
        Addr::Reg(r)
    }

    fn cost(&self) -> u32 {
        use self::Addr::*;

        match self {
            Reg(..) => 0,
            _ => 1,
        }
    }

    fn format_sp(&self) -> String {
        match self {
            Addr::LValue(v) => format!("SP + {}.rel_addr", v),
            _ => format!("{:?}", self),
        }
    }
}

impl Debug for Addr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use self::Addr::*;

        match self {
            LValue(var) => write!(f, "{}", var),
            Indexed(i, r) => write!(f, "{:?}(R{})", i, r),
            Deref(i, r) => write!(f, "*{}(R{})", i, r),
            Imm(i) => write!(f, "#{}", i),
            Reg(r) => write!(f, "R{}", r),
        }
    }
}

#[derive(Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    pub fn tag(self) -> &'static str {
        use self::BinOp::*;

        match self {
            Add => "ADD",
            Sub => "SUB",
            Mul => "MUL",
            Div => "DIV",
        }
    }

    pub fn symbol(self) -> &'static str {
        use self::BinOp::*;

        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
        }
    }
}

pub enum Cond {
    Ltz,
}

impl Cond {
    fn tag(&self) -> &'static str {
        use self::Cond::*;

        match self {
            Ltz => "LTZ",
        }
    }
}

pub enum Code {
    Ld(Reg, Addr),
    St(Addr, Word),
    Op(BinOp, Reg, Word, Word),
    Br(Label),
    Cbr(Cond, Addr, Label),
}

impl Code {
    fn cost(&self) -> u32 {
        use self::Code::*;

        match self {
            Ld(_, a) => 1 + a.cost(),
            St(a, _) => 1 + a.cost(),
            Cbr(_, a, _) => 1 + a.cost(),
            _ => 1,
        }
    }

    fn format_sp(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use self::Code::*;

        match self {
            Ld(r, a) => write!(f, "LD R{}, {}", r, a.format_sp()),
            St(a, w) => write!(f, "ST {}, {:?}", a.format_sp(), w),
            Cbr(c, addr, l) => write!(f, "B{} {}, {}", c.tag(), addr.format_sp(), l),
            _ => Debug::fmt(self, f),
        }
    }
}

impl Debug for Code {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use self::Code::*;

        match self {
            Ld(r, a) => write!(f, "LD R{}, {:?}", r, a),
            St(a, w) => write!(f, "ST {:?}, {:?}", a, w),
            Op(op, dst, lhs, rhs) => write!(f, "{} R{}, {:?}, {:?}", op.tag(), dst, lhs, rhs),
            Br(l) => write!(f, "BR {}", l),
            Cbr(c, addr, l) => write!(f, "B{} {:?}, {}", c.tag(), addr, l),
        }
    }
}

pub struct Binary {
    pub codes: Vec<Code>,
}

impl Binary {
    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        code::BinParser::new().parse(s).map_err(Box::from)
    }

    pub fn cost(&self) -> u32 {
        self.codes.iter().map(|code| code.cost()).sum()
    }
}

impl Debug for Binary {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for code in &self.codes {
            writeln!(f, "{:?}", code)?;
        }
        Ok(())
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for code in &self.codes {
            code.format_sp(f)?;
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[test]
fn parse_test() {
    let binary = "LD R0, x
LD R1, y
SUB R0, R0, R1
BLTZ *R3, L0";

    let bin = Binary::parse(binary).unwrap();
    println!("{:?}", bin);
    assert_eq!(bin.codes.len(), 4);
    assert_eq!(bin.cost(), 7);
}
