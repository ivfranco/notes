type Var = String;
pub type Reg = usize;
type Label = usize;

pub enum Idx {
    Lit(usize),
    Var(Var),
}

pub enum Addr {
    LValue(Var),
    Indexed(Idx, Reg),
    Deref(usize, Reg),
    Imm(usize),
}

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Cond {
    Ltz,
}

pub enum Code {
    Ld(Reg, Addr),
    St(Addr, Reg),
    Op(Op, Reg, Reg, Reg),
    Br(Label),
    Cbr(Cond, Addr, Label),
}
