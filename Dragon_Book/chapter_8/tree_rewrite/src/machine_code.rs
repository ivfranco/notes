use super::{BinOp, Cst, Mem, Reg, Var};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy)]
pub enum Addr {
    Reg(Reg),
    Cst(Cst),
    Mem(Mem),
    Ref(Reg),
    Idx(Var, Reg),
}

impl Debug for Addr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use Addr::*;

        match self {
            Reg(reg) => Debug::fmt(reg, f),
            Cst(cst) => write!(f, "#{:?}", cst),
            Mem(mem) => write!(f, "{}", mem),
            Ref(reg) => write!(f, "*{:?}", reg),
            Idx(arr, idx) => write!(f, "{}({:?})", arr, idx),
        }
    }
}

pub enum Code {
    Op(BinOp, Reg, Addr, Addr),
    Ld(Reg, Addr),
    St(Addr, Reg),
    Inc(Reg),
}

impl Code {
    pub fn alloc(self, reg: u8) -> Self {
        match self {
            Code::Ld(_, src) => Code::Ld(Reg::GP(reg), src),
            _ => self,
        }
    }
}

impl Debug for Code {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use Code::*;

        match self {
            Op(op, dst, lhs, rhs) => write!(f, "{} {:?}, {:?}, {:?}", op.code(), dst, lhs, rhs),
            Ld(dst, src) => write!(f, "LD {:?}, {:?}", dst, src),
            St(dst, src) => write!(f, "ST {:?}, {:?}", dst, src),
            Inc(reg) => write!(f, "INC {:?}", reg),
        }
    }
}

pub struct Binary {
    codes: Vec<Code>,
}

#[allow(clippy::len_without_is_empty)]
impl Binary {
    pub fn new(codes: Vec<Code>) -> Self {
        Binary { codes }
    }

    pub fn len(&self) -> usize {
        self.codes.len()
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
