use std::fmt::{self, Debug, Formatter};

pub type Var = String;

// absolute location of an instruction, also the target of Goto instruction
#[derive(Clone, Copy)]
pub enum Label {
    Filled(usize),
    Empty,
}

impl Label {
    pub fn patch(&mut self, dest: usize) {
        *self = Label::Filled(dest);
    }

    pub fn is_filled(&self) -> bool {
        if let Label::Filled(..) = self {
            true
        } else {
            false
        }
    }
}

impl From<usize> for Label {
    fn from(i: usize) -> Self {
        Label::Filled(i)
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Label::Filled(i) => write!(f, "{}", i),
            Label::Empty => write!(f, "_"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RelOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

impl RelOp {
    fn symbol(self) -> &'static str {
        use self::RelOp::*;
        match self {
            Eq => "==",
            Ne => "!=",
            Gt => ">",
            Ge => ">=",
            Lt => "<",
            Le => "<=",
        }
    }
}

#[derive(Clone, Copy)]
pub enum BinOp {
    Add,
}

impl BinOp {
    fn symbol(self) -> &'static str {
        match self {
            BinOp::Add => "+",
        }
    }
}

#[derive(Clone)]
pub enum RValue {
    Var(Var),
    Lit(usize),
}

impl From<String> for RValue {
    fn from(var: String) -> Self {
        RValue::Var(var)
    }
}

impl From<usize> for RValue {
    fn from(lit: usize) -> Self {
        RValue::Lit(lit)
    }
}

impl Debug for RValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            RValue::Var(var) => write!(f, "{}", var),
            RValue::Lit(lit) => write!(f, "{}", lit),
        }
    }
}

pub enum PatchError {
    NoDest,
    Repatching,
}

pub enum Instr {
    If(RelOp, RValue, RValue, Label),
    Goto(Label),
    Bin(BinOp, RValue, RValue, Var),
    Copy(RValue, Var),
    Noop,
}

impl Instr {
    fn dest(&self) -> Option<&Label> {
        match self {
            Instr::If(_, _, _, label) => Some(label),
            Instr::Goto(label) => Some(label),
            _ => None,
        }
    }

    fn is_filled(&self) -> bool {
        match self.dest() {
            None => true,
            Some(dest) => dest.is_filled(),
        }
    }

    fn dest_mut(&mut self) -> Option<&mut Label> {
        match self {
            Instr::If(_, _, _, label) => Some(label),
            Instr::Goto(label) => Some(label),
            _ => None,
        }
    }

    pub fn patch(&mut self, dest: usize) -> Result<(), PatchError> {
        match self.dest_mut() {
            None => Err(PatchError::NoDest),
            Some(label) => {
                if label.is_filled() {
                    Err(PatchError::Repatching)
                } else {
                    label.patch(dest);
                    Ok(())
                }
            }
        }
    }
}

impl Debug for Instr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Instr::If(op, lhs, rhs, label) => {
                write!(f, "if {:?} {} {:?} goto {:?}", lhs, op.symbol(), rhs, label)
            }
            Instr::Goto(label) => write!(f, "goto {:?}", label),
            Instr::Bin(op, lhs, rhs, res) => {
                write!(f, "{} = {:?} {} {:?}", res, lhs, op.symbol(), rhs)
            }
            Instr::Copy(source, res) => write!(f, "{} = {:?}", res, source),
            Instr::Noop => write!(f, "noop"),
        }
    }
}

pub struct Fragment {
    instrs: Vec<Instr>,
    start: usize,
}

impl Fragment {
    pub fn new(instrs: Vec<Instr>, start: usize) -> Self {
        Fragment { instrs, start }
    }

    pub fn len(&self) -> usize {
        self.instrs.len()
    }

    pub fn is_complete(&self) -> bool {
        self.instrs.iter().all(|instr| instr.is_filled())
    }
}

impl Debug for Fragment {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (i, instr) in self.instrs.iter().enumerate() {
            writeln!(f, "{:>width$}: {:?}", i + self.start, instr, width = 3)?;
        }
        Ok(())
    }
}
