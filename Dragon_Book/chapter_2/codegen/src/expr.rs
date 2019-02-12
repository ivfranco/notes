use crate::three_address::{Builder, Instruction, LValue, Line, Partial, RValue};

#[derive(Debug)]
pub enum Expr {
    Id(String),
    Const(u32),
    Op(Box<Expr>, String, Box<Expr>),
    Access(String, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
}

use self::Expr::*;

impl Expr {
    fn lvalue(self, builder: &mut Builder) -> LValue {
        match self {
            Id(id) => LValue::Ident(id),
            Access(array, index) => {
                let i = index.rvalue(builder);
                LValue::Access(array, i)
            }
            _ => panic!("Error: Invalid lvalue {:?}", self),
        }
    }

    fn rvalue(self, builder: &mut Builder) -> RValue {
        self.rvalue_partial(builder).unwrap_or_else(|| {
            let t = builder.new_temp();
            builder.commit_partial(t.clone());
            RValue::Ident(t)
        })
    }

    fn rvalue_partial(self, builder: &mut Builder) -> Option<RValue> {
        match self {
            Id(id) => Some(RValue::Ident(id)),

            Const(int) => Some(RValue::Const(int)),

            Op(lhs, op, rhs) => {
                let l = lhs.rvalue(builder);
                let r = rhs.rvalue(builder);
                let partial = Partial { lhs: l, op, rhs: r };

                builder.init_partial(partial);
                None
            }

            Access(array, index) => {
                let t = builder.new_temp();
                let i = index.rvalue(builder);
                let instr = Instruction::Access {
                    target: t.clone(),
                    array,
                    index: i,
                };

                builder.commit_instr(instr);
                Some(RValue::Ident(t))
            }

            Assign(target, value) => match target.lvalue(builder) {
                LValue::Access(array, index) => {
                    let r = value.rvalue(builder);
                    let instr = Instruction::Assign {
                        array,
                        index,
                        value: r.clone(),
                    };

                    builder.commit_instr(instr);
                    Some(r)
                }
                LValue::Ident(id) => {
                    if let Some(r) = value.rvalue_partial(builder) {
                        let instr = Instruction::Copy {
                            target: id,
                            value: r.clone(),
                        };
                        builder.commit_instr(instr);
                        Some(r)
                    } else {
                        builder.commit_partial(id.clone());
                        Some(RValue::Ident(id))
                    }
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    If(Expr, Vec<Stmt>),
    For(Expr, Expr, Expr, Vec<Stmt>),
}

use self::Stmt::*;

impl Stmt {
    fn gen(self, builder: &mut Builder) {
        match self {
            Expr(expr) => {
                expr.rvalue_partial(builder);
            }

            If(cond, stmts) => {
                let b = cond.rvalue(builder);
                let after = builder.new_label();
                let instr = Instruction::IfTrue {
                    cond: b,
                    label: after.clone(),
                };

                builder.commit_instr(instr);

                for stmt in stmts {
                    stmt.gen(builder);
                }

                builder.attach_label(after);
            }

            For(init, cond, end, stmts) => {
                init.rvalue_partial(builder);

                let loop_start = builder.new_label();
                let loop_end = builder.new_label();

                builder.attach_label(loop_start.clone());
                let b = cond.rvalue(builder);
                let instr = Instruction::IfFalse {
                    cond: b,
                    label: loop_end.clone(),
                };
                builder.commit_instr(instr);

                for stmt in stmts {
                    stmt.gen(builder);
                }

                end.rvalue_partial(builder);
                let goto = Instruction::Goto { label: loop_start };
                builder.commit_instr(goto);

                builder.attach_label(loop_end);
            }
        }
    }
}

pub fn codegen(stmt: Stmt) -> Vec<Line> {
    let mut builder = Builder::new();
    stmt.gen(&mut builder);
    builder.build()
}

#[test]
fn codegen_test() {
    // syntax tree for 'a[i] = 2*a[j-k]'
    let expr = Assign(
        Box::new(Access("a".to_owned(), Box::new(Id("i".to_owned())))),
        Box::new(Op(
            Box::new(Const(2)),
            "*".to_owned(),
            Box::new(Access(
                "a".to_owned(),
                Box::new(Op(
                    Box::new(Id("j".to_owned())),
                    "-".to_owned(),
                    Box::new(Id("k".to_owned())),
                )),
            )),
        )),
    );

    let expr_code = codegen(Stmt::Expr(expr));
    assert_eq!(expr_code.len(), 4);
}

#[test]
fn for_test() {
    // syntax tree for 'for (i = 0; i < 10; i = i + 1) { sum = sum + i; }'
    let stmt = For(
        Assign(Box::new(Id("i".to_owned())), Box::new(Const(0))),
        Op(
            Box::new(Id("i".to_owned())),
            "<".to_owned(),
            Box::new(Const(10)),
        ),
        Assign(
            Box::new(Id("i".to_owned())),
            Box::new(Op(
                Box::new(Id("i".to_owned())),
                "+".to_owned(),
                Box::new(Const(1)),
            )),
        ),
        vec![Stmt::Expr(Assign(
            Box::new(Id("sum".to_owned())),
            Box::new(Op(
                Box::new(Id("sum".to_owned())),
                "+".to_owned(),
                Box::new(Id("i".to_owned())),
            )),
        ))],
    );

    let stmt_code = codegen(stmt);
    assert_eq!(stmt_code.len(), 7);
}
