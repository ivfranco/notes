//! An implementation of bottom-up register allocator introduced in Engnieering a Compiler by Cooper and Torczon
//! imcomplete

#![allow(dead_code)]
use crate::machine_code::{Addr, BinOp, Binary, Code, Cond, Idx, Reg, Word};
use crate::three_addr::{Program, RValue, RelOp, Var, IR};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Next {
    Locked,
    Never,
    Exit,
    Pos(usize),
}

impl PartialOrd for Next {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Next {
    fn cmp(&self, other: &Self) -> Ordering {
        fn to_tuple(next: Next) -> (u8, usize) {
            match next {
                Next::Locked => (0, 0),
                Next::Never => (3, 0),
                Next::Exit => (2, 0),
                Next::Pos(d) => (1, d),
            }
        }

        to_tuple(*self).cmp(&to_tuple(*other))
    }
}

struct Allocator {
    size: usize,
    free: Vec<Option<Var>>,
    next: Vec<Next>,
    ensured: HashMap<Var, Reg>,
}

impl Allocator {
    fn new(size: usize) -> Self {
        Allocator {
            size,
            free: vec![None; size],
            next: vec![Next::Never; size],
            ensured: HashMap::new(),
        }
    }

    fn ensure(&mut self, addr: &Var, codes: &mut Vec<Code>) -> Reg {
        if let Some(r) = self.ensured.get(addr) {
            *r
        } else {
            let r = self.allocate(addr, codes);
            let code = Code::Ld(r, Addr::LValue(addr.clone()));
            codes.push(code);
            r
        }
    }

    fn allocate(&mut self, addr: &Var, codes: &mut Vec<Code>) -> Reg {
        let r = if let Some(r) = self.free_reg() {
            r
        } else {
            let r = self.furthest_reg();
            let orig = self.free[r].take().unwrap();
            let code = Code::St(Addr::LValue(orig), Word::Reg(r));
            codes.push(code);
            r
        };

        self.ensured.insert(addr.clone(), r);
        self.free[r] = Some(addr.clone());

        r
    }

    fn free(&mut self, r: Reg) {
        self.free[r] = None;
    }

    fn free_reg(&self) -> Option<Reg> {
        self.free
            .iter()
            .enumerate()
            .find_map(|(r, v)| if v.is_none() { Some(r) } else { None })
    }

    fn furthest_reg(&self) -> Reg {
        self.next
            .iter()
            .enumerate()
            .filter(|(_, next)| **next != Next::Locked)
            .max_by_key(|(_, next)| *next)
            .expect("Error: Registers exhausted")
            .0
    }
}

#[derive(Default)]
struct UseMap {
    uses: HashMap<Var, Vec<usize>>,
    on_exit: HashSet<Var>,
}

impl UseMap {
    fn new(program: &Program, on_exit: &[&str]) -> Self {
        let mut map = Self::default();
        map.on_exit = on_exit.iter().map(|v| v.to_string()).collect();
        for (i, ir) in program.lines.iter().map(|line| &line.ir).enumerate() {
            map.update(i, ir);
        }
        map
    }

    fn update(&mut self, i: usize, ir: &IR) {
        match ir {
            IR::Op(dst, lhs, _, rhs) => {
                self.push_rvalue(i, lhs);
                self.push_rvalue(i, rhs);
            }
            IR::Copy(dst, src) => {
                self.push_var(i, dst);
                self.push_rvalue(i, src);
            }
            _ => unimplemented!(),
        }
    }

    fn push_var(&mut self, i: usize, var: &Var) {
        self.uses
            .entry(var.clone())
            .or_insert_with(Vec::new)
            .push(i);
    }

    fn push_rvalue(&mut self, i: usize, rvalue: &RValue) {
        if let RValue::Var(var) = rvalue {
            self.push_var(i, var);
        }
    }

    fn next_use(&self, start: usize, var: &Var) -> Next {
        if let Some(i) = self.uses[var].iter().find(|i| **i > start) {
            Next::Pos(*i)
        } else if self.on_exit.contains(var) {
            Next::Exit
        } else {
            Next::Never
        }
    }
}

struct Builder {
    codes: Vec<Code>,
    use_map: UseMap,
    allocator: Allocator,
}

impl Builder {
    fn new(program: &Program, on_exit: &[&str], regs: usize) -> Self {
        Builder {
            codes: vec![],
            use_map: UseMap::new(program, on_exit),
            allocator: Allocator::new(regs),
        }
    }

    fn load(&mut self, addr: &Var) -> Reg {
        self.allocator.ensure(addr, &mut self.codes)
    }

    fn load_rvalue(&mut self, rvalue: &RValue) -> Word {
        match rvalue {
            RValue::Lit(lit) => Word::Lit(*lit),
            RValue::Var(var) => Word::Reg(self.load(var)),
        }
    }

    fn update_next(&mut self, i: usize, r: Reg, addr: &Var) {
        let next = self.use_map.next_use(i, addr);
        self.allocator.next[r] = next;
    }

    fn cleanup(&mut self, i: usize, r: Reg, addr: &Var) {
        if let Next::Never = self.use_map.next_use(i, addr) {
            self.allocator.free(r);
        } else {
            self.update_next(i, r, addr);
        }
    }

    fn new_reg(&mut self, var: &Var) -> Reg {
        self.allocator.allocate(var, &mut self.codes)
    }

    fn store(&mut self, dst: &Var, w: Word) {
        let code = Code::St(Addr::LValue(dst.clone()), w);
        self.codes.push(code);
    }

    pub fn gen(&mut self, i: usize, ir: &IR) {
        use self::IR::*;

        match ir {
            Op(dst, lhs, op, rhs) => {
                let l = self.load_rvalue(lhs);
                let r = self.load_rvalue(rhs);

                if let (Word::Reg(r), RValue::Var(v)) = (l, lhs) {
                    self.cleanup(i, r, v);
                }
                if let (Word::Reg(r), RValue::Var(v)) = (r, rhs) {
                    self.cleanup(i, r, v);
                }

                let res = self.new_reg(dst);
                self.codes.push(Code::Op(*op, res, l, r));
                self.update_next(i, res, dst);
            }
            Copy(dst, src) => {
                let d = Addr::LValue(dst.to_owned());

                let s = self.load_rvalue(src);
                self.store(dst, s);
            }
            _ => unimplemented!(),
        }
    }
}
