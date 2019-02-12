use std::fmt::{self, Debug, Formatter};
use std::mem;

pub type Ident = String;

#[derive(Clone)]
pub enum RValue {
    Ident(Ident),
    Const(u32),
}

#[derive(Clone)]
pub enum LValue {
    Ident(Ident),
    Access(Ident, RValue),
}

impl Debug for RValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            RValue::Ident(id) => write!(f, "{}", id),
            RValue::Const(int) => write!(f, "{}", int),
        }
    }
}

pub enum Instruction {
    Noop,

    Op {
        target: Ident,
        lhs: RValue,
        op: String,
        rhs: RValue,
    },

    Access {
        target: Ident,
        array: Ident,
        index: RValue,
    },

    Assign {
        array: Ident,
        index: RValue,
        value: RValue,
    },

    Copy {
        target: Ident,
        value: RValue,
    },

    IfTrue {
        cond: RValue,
        label: Ident,
    },

    IfFalse {
        cond: RValue,
        label: Ident,
    },

    Goto {
        label: Ident,
    },
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use self::Instruction::*;
        match self {
            Noop => write!(f, "Noop"),

            Op {
                target,
                lhs,
                op,
                rhs,
            } => write!(f, "{} = {:?} {} {:?}", target, lhs, op, rhs),

            Access {
                target,
                array,
                index,
            } => write!(f, "{} = {}[{:?}]", target, array, index),

            Assign {
                array,
                index,
                value,
            } => write!(f, "{}[{:?}] = {:?}", array, index, value),

            Copy { target, value } => write!(f, "{} = {:?}", target, value),

            IfTrue { cond, label } => write!(f, "IfTrue {:?} goto {}", cond, label),

            IfFalse { cond, label } => write!(f, "IfFalse {:?} goto {}", cond, label),

            Goto { label } => write!(f, "goto {}", label),
        }
    }
}

pub struct Partial {
    pub lhs: RValue,
    pub op: String,
    pub rhs: RValue,
}

pub struct Line {
    labels: Vec<String>,
    instr: Instruction,
}

impl Debug for Line {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for label in &self.labels {
            write!(f, "{}: ", label)?;
        }

        Debug::fmt(&self.instr, f)
    }
}

pub struct Builder {
    lines: Vec<Line>,
    labels: Vec<String>,
    label_count: u32,
    temp_count: u32,
    partial: Option<Partial>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            lines: vec![],
            labels: vec![],
            label_count: 0,
            temp_count: 0,
            partial: None,
        }
    }

    pub fn new_temp(&mut self) -> Ident {
        let id = format!("t{}", self.temp_count);
        self.temp_count += 1;
        id
    }

    pub fn new_label(&mut self) -> String {
        let label = format!("label{}", self.label_count);
        self.label_count += 1;
        label
    }

    pub fn attach_label(&mut self, label: String) {
        self.labels.push(label);
    }

    pub fn commit_instr(&mut self, instr: Instruction) {
        assert!(
            self.partial.is_none(),
            "Error: Incomplete partial instruction"
        );
        let labels = mem::replace(&mut self.labels, vec![]);
        let line = Line { labels, instr };
        println!("{:?}", line);
        self.lines.push(line)
    }

    pub fn init_partial(&mut self, partial: Partial) {
        self.partial = Some(partial);
    }

    pub fn commit_partial(&mut self, id: Ident) {
        let partial = self
            .partial
            .take()
            .expect("Error: Prematured partial sealing");

        let instr = Instruction::Op {
            target: id,
            lhs: partial.lhs,
            op: partial.op,
            rhs: partial.rhs,
        };

        self.commit_instr(instr);
    }

    pub fn build(mut self) -> Vec<Line> {
        if !self.labels.is_empty() {
            self.commit_instr(Instruction::Noop);
        }

        self.lines
    }
}
