use crate::machine_code::{Addr, BinOp, Binary, Code, Cond, Idx, Reg, Word};
use crate::three_addr::{RValue, RelOp, IR};
use std::collections::HashMap;
use std::mem;

pub const INT_SIZE: usize = 4;

#[derive(Default)]
pub struct Builder {
    codes: Vec<Code>,
    cache: HashMap<Addr, Reg>,
    next_reg: Reg,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    fn new_reg(&mut self) -> Reg {
        let r = self.next_reg;
        self.next_reg += 1;
        r
    }

    fn load(&mut self, addr: Addr) -> Reg {
        if let Some(r) = self.cache.get(&addr) {
            *r
        } else {
            let r = self.new_reg();
            self.cache.insert(addr.clone(), r);
            let code = Code::Ld(r, addr);
            self.codes.push(code);
            r
        }
    }

    fn load_var(&mut self, var: &str) -> Reg {
        self.load(Addr::LValue(var.to_owned()))
    }

    fn load_rvalue(&mut self, rvalue: &RValue) -> Word {
        match rvalue {
            RValue::Lit(l) => Word::Lit(*l),
            RValue::Var(v) => Word::Reg(self.load_var(v)),
        }
    }

    fn store(&mut self, addr: Addr, w: Word) {
        let code = Code::St(addr.clone(), w);
        self.codes.push(code);
        if let Word::Reg(r) = w {
            self.cache.insert(addr, r);
        }
    }

    fn access(&mut self, src: &str, idx: &RValue) -> Addr {
        match idx {
            RValue::Lit(i) => {
                let s = self.load_var(src);
                Addr::Indexed(Idx::Lit(i * INT_SIZE), s)
            }
            RValue::Var(v) => {
                let r = self.load_var(v);
                let i = self.new_reg();
                let mul = Code::Op(BinOp::Mul, i, Word::Reg(r), Word::Lit(INT_SIZE));

                self.codes.push(mul);
                Addr::Indexed(Idx::Var(src.to_owned()), i)
            }
        }
    }

    pub fn gen(&mut self, ir: &IR) {
        use self::IR::*;

        match ir {
            ArrayAccess(dst, src, idx) => {
                let addr = self.access(src, idx);
                let r = self.load(addr);
                self.store(Addr::LValue(dst.to_owned()), Word::Reg(r));
            }
            ArrayAssign(dst, idx, src) => {
                let addr = self.access(dst, idx);
                let s = self.load_rvalue(src);
                self.store(addr, s);
            }
            RefAccess(dst, src) => {
                let r = self.load_var(src);
                let addr = Addr::Deref(0, r);
                let d = Addr::LValue(dst.to_owned());

                let s = self.load(addr);
                self.store(d, Word::Reg(s));
            }
            RefAssign(dst, src) => {
                let r = self.load_var(dst);
                let d = Addr::Deref(0, r);

                let s = self.load_var(src);
                self.store(d, Word::Reg(s));
            }
            Op(dst, lhs, op, rhs) => {
                let d = Addr::LValue(dst.to_owned());
                let l = self.load_rvalue(lhs);
                let r = self.load_rvalue(rhs);

                let res = self.new_reg();
                self.codes.push(Code::Op(*op, res, l, r));
                self.store(d, Word::Reg(res));
            }
            Copy(dst, src) => {
                let d = Addr::LValue(dst.to_owned());

                let s = self.load_rvalue(src);
                self.store(d, s);
            }
            Goto(label) => {
                let br = Code::Br(label.to_owned());
                self.codes.push(br);
            }
            If(lhs, op, rhs, label) => {
                let l = self.load_rvalue(lhs);
                let r = self.load_rvalue(rhs);
                let rel = self.new_reg();

                let cmp = match op {
                    RelOp::Gt => Code::Op(BinOp::Sub, rel, r, l),
                    RelOp::Lt => Code::Op(BinOp::Sub, rel, l, r),
                };
                let cbr = Code::Cbr(Cond::Ltz, Addr::reg(rel), label.to_owned());

                self.codes.push(cmp);
                self.codes.push(cbr);
            }
            _ => (),
        }
    }

    pub fn seal(&mut self) -> Binary {
        let codes = mem::replace(&mut self.codes, vec![]);
        Binary { codes }
    }
}

#[test]
fn build_test() {
    use crate::three_addr::Program;

    let binary = Program::parse("x = b * c; y = a + x;").unwrap().build();
    assert_eq!(binary.codes.len(), 7);
}
