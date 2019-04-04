//! An implementation of bottom-up register allocator introduced in Engnieering a Compiler by Cooper and Torczon
//! imcomplete

use crate::machine_code::{Addr, Binary, Code, Reg, Word};
use crate::three_addr::{Program, RValue, Var, IR};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug)]
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

    fn ensure(&mut self, addr: &str, codes: &mut Vec<Code>) -> Reg {
        if let Some(r) = self.ensured.get(addr) {
            *r
        } else {
            let r = self.allocate(addr, codes);
            let code = Code::Ld(r, Addr::LValue(addr.to_string()));
            codes.push(code);
            r
        }
    }

    fn store(&mut self, addr: &str, codes: &mut Vec<Code>) {
        if let Some(r) = self.ensured.get(addr).cloned() {
            let st = Code::St(Addr::LValue(addr.to_string()), Word::Reg(r));
            codes.push(st);
            self.ensured.remove(addr);
            self.free[r] = None;
        }
    }

    fn allocate(&mut self, addr: &str, codes: &mut Vec<Code>) -> Reg {
        let r = if let Some(r) = self.free_reg() {
            r
        } else {
            let r = self.furthest_reg();
            let orig = self.free[r].take().unwrap();
            self.ensured.remove(&orig);
            if self.next[r] != Next::Never {
                let code = Code::St(Addr::LValue(orig), Word::Reg(r));
                codes.push(code);
            }
            r
        };

        self.ensured.insert(addr.to_string(), r);
        self.free[r] = Some(addr.to_string());

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

#[derive(Debug, Default)]
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
            IR::Op(_, lhs, _, rhs) => {
                self.push_rvalue(i, lhs);
                self.push_rvalue(i, rhs);
            }
            IR::Copy(_, src) => {
                self.push_rvalue(i, src);
            }
            _ => unimplemented!(),
        }
    }

    fn push_var(&mut self, i: usize, var: &str) {
        self.uses
            .entry(var.to_string())
            .or_insert_with(Vec::new)
            .push(i);
    }

    fn push_rvalue(&mut self, i: usize, rvalue: &RValue) {
        if let RValue::Var(var) = rvalue {
            self.push_var(i, var);
        }
    }

    fn next_use(&self, start: usize, var: &str) -> Next {
        if let Some(i) = self.uses[var].iter().find(|i| **i > start) {
            Next::Pos(*i)
        } else if self.on_exit.contains(var) {
            Next::Exit
        } else {
            Next::Never
        }
    }
}

pub struct Builder {
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

    fn load(&mut self, addr: &str) -> Reg {
        let r = self.allocator.ensure(addr, &mut self.codes);
        self.lock(r);
        r
    }

    fn load_rvalue(&mut self, rvalue: &RValue) -> Word {
        match rvalue {
            RValue::Lit(lit) => Word::Lit(*lit),
            RValue::Var(var) => Word::Reg(self.load(var)),
        }
    }

    fn update_next(&mut self, i: usize, r: Reg, addr: &str) {
        let next = self.use_map.next_use(i, addr);
        self.allocator.next[r] = next;
    }

    fn cleanup(&mut self, i: usize, r: Reg, addr: &str) {
        if let Next::Never = self.use_map.next_use(i, addr) {
            self.allocator.free(r);
        }
        self.update_next(i, r, addr);
    }

    fn cleanup_rvalue(&mut self, i: usize, w: Word, rvalue: &RValue) {
        if let (Word::Reg(r), RValue::Var(v)) = (w, rvalue) {
            self.cleanup(i, r, v);
        }
    }

    fn lock(&mut self, r: Reg) {
        self.allocator.next[r] = Next::Locked;
    }

    fn new_reg(&mut self, var: &str) -> Reg {
        self.allocator.allocate(var, &mut self.codes)
    }

    fn store(&mut self, dst: &str, w: Word) {
        let code = Code::St(Addr::LValue(dst.to_string()), w);
        self.codes.push(code);
    }

    pub fn gen(&mut self, i: usize, ir: &IR) {
        use self::IR::*;

        match ir {
            Op(dst, lhs, op, rhs) => {
                let l = self.load_rvalue(lhs);
                let r = self.load_rvalue(rhs);

                self.cleanup_rvalue(i, l, lhs);
                self.cleanup_rvalue(i, r, rhs);

                let res = self.new_reg(dst);
                self.codes.push(Code::Op(*op, res, l, r));
                self.update_next(i, res, dst);
            }
            Copy(dst, src) => {
                let w = self.load_rvalue(src);
                self.store(dst, w);
                self.cleanup_rvalue(i, w, src);
            }
            _ => unimplemented!(),
        }
    }

    fn seal(mut self) -> Binary {
        for v in &self.use_map.on_exit {
            self.allocator.store(v, &mut self.codes);
        }

        Binary { codes: self.codes }
    }

    pub fn build(program: Program, on_exit: &[&str], regs: usize) -> Binary {
        let mut builder = Builder::new(&program, on_exit, regs);

        for (i, ir) in program.lines.iter().map(|line| &line.ir).enumerate() {
            builder.gen(i, ir);
        }

        builder.seal()
    }
}

#[test]
fn build_test() {
    let program = "
t0 = b + c;
t1 = a / t0;
t2 = e + f;
t3 = d * t2;
t4 = t1 - t3;
x = t4;
    ";

    let p = Program::parse(program).unwrap();
    let bin = Builder::build(p, &["x"], 2);
    // println!("{:?}", bin);
    // LD R0, b
    // LD R1, c
    // ADD R0, R0, R1
    // LD R1, a
    // DIV R0, R1, R0
    // LD R1, e
    // ST t1, R0
    // LD R0, f
    // ADD R0, R1, R0
    // LD R1, d
    // MUL R0, R1, R0
    // LD R1, t1
    // SUB R0, R1, R0
    // ST x, R0
    assert_eq!(bin.codes.len(), 14);
}
