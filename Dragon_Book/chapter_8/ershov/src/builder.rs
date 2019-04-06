use super::{BinOp, Expr, Node, UnOp};
use crate::machine_code::{Binary, Code, Mem, Reg, Word};
use std::cmp;

#[derive(Default)]
struct Allocator {
    codes: Vec<Code>,
    regs: u8,
    tmps: u8,
    base: u8,
}

impl Allocator {
    fn new(regs: u8) -> Self {
        Allocator {
            regs,
            ..Self::default()
        }
    }

    fn alloc(&mut self) -> Reg {
        assert!(self.regs > self.base);

        let reg = self.base;
        self.base += 1;
        reg
    }

    fn store(&mut self, reg: Reg) -> Mem {
        let tmp = Mem::Tmp(self.tmps);
        let code = Code::St(tmp, reg);
        self.tmps += 1;
        self.push(code);
        tmp
    }

    fn load(&mut self, reg: Reg, mem: Mem) {
        let code = Code::Ld(reg, mem);
        self.push(code);
    }

    fn reset(&mut self, base: u8) {
        self.base = base;
    }

    fn push(&mut self, code: Code) {
        self.codes.push(code);
    }
}

pub struct Builder {
    allocator: Allocator,
}

impl Builder {
    fn new(regs: u8) -> Self {
        assert!(regs >= 2);

        Builder {
            allocator: Allocator::new(regs),
        }
    }

    fn regs(&self) -> u8 {
        self.allocator.regs
    }

    fn gen(&mut self, node: &Node) -> Reg {
        match &node.expr {
            Expr::Var(v) => {
                let r = self.allocator.alloc();
                let code = Code::Ld(r, Mem::Var(*v));
                self.allocator.push(code);
                r
            }
            Expr::Un(UnOp::Deref, inner) => {
                let r = self.gen(inner);
                let code = Code::Ld(r, Mem::Ind(r));
                self.allocator.push(code);
                r
            }
            Expr::Un(UnOp::Neg, inner) => {
                let r = self.gen(inner);
                let code = Code::Op(BinOp::Sub, r, Word::Lit(0), Word::Reg(r));
                self.allocator.push(code);
                r
            }
            Expr::Bin(lhs, op, rhs) => {
                let (l, r) = self.eval(lhs, rhs);
                let dst = cmp::max(l, r);
                let code = Code::Op(*op, dst, Word::Reg(l), Word::Reg(r));
                self.allocator.push(code);
                dst
            }
        }
    }

    fn eval(&mut self, lhs: &Node, rhs: &Node) -> (Reg, Reg) {
        let swapped = lhs.label < rhs.label;
        let (gt, lt) = if !swapped { (lhs, rhs) } else { (rhs, lhs) };

        let label = if gt.label == lt.label {
            gt.label + 1
        } else {
            gt.label
        };

        let (g, l) = if label > self.regs() {
            self.eval_capped(gt, lt)
        } else {
            self.eval_uncapped(gt, lt)
        };

        if swapped {
            (l, g)
        } else {
            (g, l)
        }
    }

    fn eval_uncapped(&mut self, gt: &Node, lt: &Node) -> (Reg, Reg) {
        let base = self.allocator.base;
        if gt.label == lt.label {
            self.allocator.reset(base + 1);
        }
        let g = self.gen(gt);

        self.allocator.reset(base);
        let l = self.gen(lt);

        (g, l)
    }

    fn eval_capped(&mut self, gt: &Node, lt: &Node) -> (Reg, Reg) {
        self.allocator.reset(0);
        let g = self.gen(gt);
        assert_eq!(g + 1, self.regs());

        // it's not always necessary to store g
        // if the smaller child only uses l < r registers, result of the bigger child in r will not be overwritten
        let tmp = if lt.label + 1 >= self.regs() {
            self.allocator.store(g)
        } else {
            Mem::Null
        };

        self.allocator.reset(0);
        let l = self.gen(lt);

        let f = if l + 1 == self.regs() { l - 1 } else { g };
        if let Mem::Tmp(..) = tmp {
            self.allocator.load(f, tmp);
        }
        (f, l)
    }

    fn seal(self) -> Binary {
        Binary::new(self.allocator.codes)
    }

    pub fn build(node: &Node, regs: u8) -> Binary {
        let mut builder = Builder::new(regs);
        builder.gen(node);
        builder.seal()
    }
}

#[test]
fn build_test() {
    let expr = "(a - b) + e * (c + d)";
    let node = Node::parse(expr).unwrap();
    let binary = Builder::build(&node, 2);
    // println!("{:?}", binary);
    assert_eq!(binary.len(), 11);
}
