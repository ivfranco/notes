#[macro_use]
extern crate lalrpop_util;

pub mod builder;
mod three_addr;

use crate::builder::ProcBuilder;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::error::Error;
use three_addr::{BinOp, Fragment, Instr, Label, RValue, RelOp, Var};

thread_local! {
    static BUILDER: RefCell<ProcBuilder> = RefCell::new(ProcBuilder::default());
}

lalrpop_mod!(pub infix);

pub fn next_instr() -> usize {
    BUILDER.with(|builder| builder.borrow().next_instr())
}

fn gen(instr: Instr) {
    BUILDER.with(|builder| builder.borrow_mut().gen(instr))
}

fn backpatch(list: &LinkedList<usize>, label: usize) {
    BUILDER.with(|builder| builder.borrow_mut().backpatch(list, label))
}

fn new_temp() -> String {
    BUILDER.with(|builder| builder.borrow_mut().new_temp())
}

fn merge(mut lhs: LinkedList<usize>, mut rhs: LinkedList<usize>) -> LinkedList<usize> {
    lhs.append(&mut rhs);
    lhs
}

fn builder_init(start: usize) {
    BUILDER.with(|builder| builder.replace(ProcBuilder::new(start)));
}

fn build_fragment() -> Fragment {
    let builder = BUILDER.with(|builder| builder.replace(ProcBuilder::default()));
    builder.build_fragment()
}

pub struct Boolean {
    t_list: LinkedList<usize>,
    f_list: LinkedList<usize>,
}

impl Boolean {
    pub fn new(t_list: LinkedList<usize>, f_list: LinkedList<usize>) -> Self {
        // which subexpression this is usually can be recovered from the rm nature of LR parser
        println!("truelist = {:?}, falselist = {:?}", t_list, f_list);
        Boolean { t_list, f_list }
    }

    pub fn t() -> Self {
        let t_list = [next_instr()].iter().cloned().collect();
        let f_list = LinkedList::new();
        gen(Instr::Goto(Label::Empty));

        Boolean::new(t_list, f_list)
    }

    pub fn f() -> Self {
        let t_list = LinkedList::new();
        let f_list = [next_instr()].iter().cloned().collect();
        gen(Instr::Goto(Label::Empty));

        Boolean::new(t_list, f_list)
    }

    pub fn rel(lhs: Expr, op: RelOp, rhs: Expr) -> Self {
        let t_list = [next_instr()].iter().cloned().collect();
        let f_list = [next_instr() + 1].iter().cloned().collect();
        gen(Instr::If(op, lhs.rvalue, rhs.rvalue, Label::Empty));
        gen(Instr::Goto(Label::Empty));

        Boolean::new(t_list, f_list)
    }

    pub fn or(self, dest: usize, rhs: Self) -> Self {
        let lhs = self;
        backpatch(&lhs.f_list, dest);

        Boolean::new(merge(lhs.t_list, rhs.t_list), rhs.f_list)
    }

    pub fn and(self, dest: usize, rhs: Self) -> Self {
        let lhs = self;
        backpatch(&lhs.t_list, dest);

        Boolean::new(rhs.t_list, merge(lhs.f_list, rhs.f_list))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        Boolean::new(self.f_list, self.t_list)
    }

    pub fn parse<'a>(start: usize, s: &'a str) -> Result<Fragment, Box<Error + 'a>> {
        builder_init(start);
        infix::BOrParser::new().parse(s)?;
        Ok(build_fragment())
    }
}

pub struct Expr {
    rvalue: RValue,
}

impl Expr {
    fn new(rvalue: RValue) -> Self {
        Expr { rvalue }
    }

    fn var(id: Var) -> Self {
        Expr::new(id.into())
    }

    fn lit(n: usize) -> Self {
        Expr::new(n.into())
    }

    fn bin(lhs: Self, op: BinOp, rhs: Self) -> Self {
        let t = new_temp();
        gen(Instr::Bin(op, lhs.rvalue, rhs.rvalue, t.clone()));
        Expr::var(t)
    }
}

pub struct Stmt {
    next: LinkedList<usize>,
}

impl Stmt {
    fn new(next: LinkedList<usize>) -> Self {
        Stmt { next }
    }

    pub fn assign(res: Var, rhs: Expr) -> Self {
        gen(Instr::Copy(rhs.rvalue, res));
        Stmt::new(LinkedList::new())
    }

    pub fn if_only(cond: Boolean, dest: usize, body: Self) -> Self {
        backpatch(&cond.t_list, dest);
        Stmt::new(merge(cond.f_list, body.next))
    }

    pub fn goto() -> Self {
        let next = [next_instr()].iter().cloned().collect();
        gen(Instr::Goto(Label::Empty));
        Stmt::new(next)
    }

    pub fn if_else(
        cond: Boolean,
        t_start: usize,
        t_clause: Self,
        mut goto: Self,
        f_start: usize,
        mut f_clause: Self,
    ) -> Self {
        backpatch(&cond.t_list, t_start);
        backpatch(&cond.f_list, f_start);

        let mut next = t_clause.next;
        next.append(&mut goto.next);
        next.append(&mut f_clause.next);
        Stmt::new(next)
    }

    pub fn while_clause(cond_start: usize, cond: Boolean, body_start: usize, body: Self) -> Self {
        backpatch(&cond.t_list, body_start);
        backpatch(&body.next, cond_start);
        gen(Instr::Goto(cond_start.into()));

        Stmt::new(cond.f_list)
    }

    pub fn append(self, dest: usize, other: Self) -> Self {
        backpatch(&self.next, dest);
        other
    }

    pub fn parse<'a>(start: usize, s: &'a str) -> Result<Fragment, Box<Error + 'a>> {
        builder_init(start);
        infix::LParser::new().parse(s)?;
        Ok(build_fragment())
    }
}

#[cfg(test)]
const START: usize = 100;

#[test]
fn boolean_build_test() {
    let program = Boolean::parse(START, "x < 100 || x > 200 && x != y").unwrap();
    // println!("{:?}", program);
    // 100: if x < 100 goto _
    // 101: goto 102
    // 102: if x > 200 goto 104
    // 103: goto _
    // 104: if x != y goto _
    // 105: goto _
    assert_eq!(program.len(), 6);
}

#[test]
fn stmt_build_test() {
    let program = Stmt::parse(START, "if( x < 100 || x > 200 && x != y ) { x = 0; }").unwrap();
    // println!("{:?}", program);
    // 100: if x < 100 goto 106
    // 101: goto 102
    // 102: if x > 200 goto 104
    // 103: goto _
    // 104: if x != y goto 106
    // 105: goto _
    // 106: x = 0
    assert_eq!(program.len(), 7);
}
