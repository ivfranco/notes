use super::{BinOp, Var};
use std::fmt::{self, Debug, Display, Formatter};

pub type Reg = u8;
pub type Tmp = u8;

#[derive(Clone, Copy)]
pub enum Mem {
    Var(Var),
    Tmp(Tmp),
    Ind(Reg),
    Null,
}

impl Display for Mem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Mem::Var(v) => write!(f, "#{}", v),
            Mem::Tmp(t) => write!(f, "#t{}", t),
            Mem::Ind(r) => write!(f, "*R{}", r),
            Mem::Null => write!(f, "NULL"),
        }
    }
}

pub enum Word {
    Reg(Reg),
    Lit(u32),
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Word::Reg(r) => write!(f, "R{}", r),
            Word::Lit(l) => write!(f, "{}", l),
        }
    }
}

pub enum Code {
    Ld(Reg, Mem),
    St(Mem, Reg),
    Op(BinOp, Reg, Word, Word),
}

impl Debug for Code {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Code::Ld(r, m) => write!(f, "LD R{}, {}", r, m),
            Code::St(m, r) => write!(f, "ST {}, R{}", m, r),
            Code::Op(op, dst, lhs, rhs) => write!(f, "{} R{}, {}, {}", op.code(), dst, lhs, rhs),
        }
    }
}

pub struct Binary {
    codes: Vec<Code>,
}

impl Binary {
    pub fn new(codes: Vec<Code>) -> Self {
        Binary { codes }
    }

    pub fn len(&self) -> usize {
        self.codes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
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
