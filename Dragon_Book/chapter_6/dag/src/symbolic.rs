use self::three_addr::{Instr, ProcBuilder, RValue, Var};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

// would be unnecessary if the syntax of LALRPOP is more flexible
thread_local! {
    static EXPRS: RefCell<HashSet<Rc<Expr>>> = RefCell::new(HashSet::new());
}

fn exprs_init() {
    EXPRS.with(|exprs| exprs.borrow_mut().clear());
}

lalrpop_mod!(pub infix);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
}

impl BinOp {
    fn symbol(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnOp {
    Pos,
    Neg,
}

impl UnOp {
    fn symbol(self) -> &'static str {
        match self {
            UnOp::Pos => "+",
            UnOp::Neg => "-",
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Expr {
    Bin(BinOp, Rc<Expr>, Rc<Expr>),
    Un(UnOp, Rc<Expr>),
    Access(Access),
    Var(String),
}

impl Expr {
    fn dedup(self) -> Rc<Self> {
        let expr = Rc::new(self);
        EXPRS.with(move |exprs| {
            let mut borrowed = exprs.borrow_mut();
            if let Some(v) = borrowed.get(&expr) {
                v.clone()
            } else {
                borrowed.insert(expr.clone());
                expr
            }
        })
    }

    pub fn bin(op: BinOp, lhs: Rc<Expr>, rhs: Rc<Expr>) -> Rc<Self> {
        Expr::Bin(op, lhs, rhs).dedup()
    }

    pub fn un(op: UnOp, inner: Rc<Expr>) -> Rc<Self> {
        Expr::Un(op, inner).dedup()
    }

    pub fn access(access: Access) -> Rc<Self> {
        Expr::Access(access).dedup()
    }

    pub fn var(s: String) -> Rc<Self> {
        Expr::Var(s).dedup()
    }

    pub fn parse<'a>(s: &'a str) -> Result<Rc<Self>, Box<Error + 'a>> {
        exprs_init();
        infix::EParser::new().parse(s).map_err(Box::from)
    }

    fn format(&self, map: &ExprMap, f: &mut Formatter) -> Result<(), fmt::Error> {
        let id = map[self];
        write!(f, "{}: ", id)?;
        match self {
            Expr::Bin(op, lhs, rhs) => {
                let lhs_id = map[lhs];
                let rhs_id = map[rhs];
                writeln!(f, "{:?}({}, {})", op, lhs_id, rhs_id)
            }
            Expr::Un(op, inner) => {
                let inner_id = map[inner];
                writeln!(f, "{:?}({})", op, inner_id)
            }
            Expr::Access(access) => writeln!(f, "{:?}", access),
            Expr::Var(var) => writeln!(f, "{}", var),
        }
    }

    fn walk(&self, builder: &mut ProcBuilder) -> Var {
        match self {
            Expr::Bin(op, lhs, rhs) => {
                let l = lhs.walk(builder);
                let r = rhs.walk(builder);
                let t = builder.new_temp();
                let instr = Instr::Bin(*op, l.into(), r.into(), t.clone());
                builder.push(instr);
                t
            }
            Expr::Un(op, inner) => {
                let inn = inner.walk(builder);
                let t = builder.new_temp();
                let instr = Instr::Un(*op, inn.into(), t.clone());
                builder.push(instr);
                t
            }
            Expr::Access(access) => access.rwalk(builder),
            Expr::Var(var) => var.clone(),
        }
    }
}

type ExprMap = HashMap<Rc<Expr>, usize>;

pub struct DAG {
    top: Rc<Expr>,
    map: ExprMap,
}

impl DAG {
    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        let top = Expr::parse(s)?;
        let map = EXPRS
            .with(|exprs| exprs.replace(HashSet::new()))
            .into_iter()
            .zip(0..)
            .collect();

        Ok(DAG { top, map })
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }
}

impl Debug for DAG {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "top node: {}", self.map[&self.top])?;
        let mut pairs: Vec<_> = self.map.iter().collect();
        pairs.sort_by_key(|(_, v)| *v);
        for (expr, _) in pairs {
            expr.format(&self.map, f)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Access {
    var: String,
    dims: Vec<Rc<Expr>>,
}

impl Access {
    fn lwalk(&self, builder: &mut ProcBuilder) -> (Var, RValue) {
        assert!(!self.dims.is_empty());

        let mut t = self.dims[0].walk(builder).into();
        if self.dims.len() > 1 {
            let prod = builder.new_temp();

            for (i, expr) in self.dims.iter().skip(1).enumerate() {
                let width: RValue = format!("{}.dim{}", self.var, i).into();
                builder.push(Instr::Bin(BinOp::Mul, t, width, prod.clone()));
                let next = builder.new_temp();
                let dim = expr.walk(builder).into();
                builder.push(Instr::Bin(
                    BinOp::Add,
                    prod.clone().into(),
                    dim,
                    next.clone(),
                ));
                t = next.into();
            }
        }

        let width: RValue = format!("{}.base.width", self.var).into();
        let next = builder.new_temp();
        builder.push(Instr::Bin(BinOp::Mul, t, width, next.clone()));
        t = next.into();

        (self.var.clone(), t)
    }

    fn rwalk(&self, builder: &mut ProcBuilder) -> Var {
        let (var, idx) = self.lwalk(builder);
        let t = builder.new_temp();
        let instr = Instr::Access(var, idx, t.clone());
        builder.push(instr);
        t
    }
}

impl Debug for Access {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ", self.var)?;
        for _ in &self.dims {
            write!(f, "[..]")?;
        }
        Ok(())
    }
}

pub struct Stmt {
    lvalue: LValue,
    rvalue: Rc<Expr>,
}

impl Stmt {
    fn walk(&self, builder: &mut ProcBuilder) {
        let rhs = self.rvalue.walk(builder);
        match &self.lvalue {
            LValue::Var(var) => {
                let instr = Instr::Copy(rhs.into(), var.clone());
                builder.push(instr);
            }
            LValue::Access(access) => {
                let (var, idx) = access.lwalk(builder);
                let instr = Instr::Access(rhs, idx, var);
                builder.push(instr);
            }
        }
    }
}

pub enum LValue {
    Var(Var),
    Access(Access),
}

pub struct Stmts {
    stmts: Vec<Stmt>,
}

impl Stmts {
    pub fn walk(&self, builder: &mut ProcBuilder) {
        for stmt in &self.stmts {
            stmt.walk(builder);
        }
    }
}

impl Stmts {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Stmts { stmts }
    }

    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        exprs_init();
        infix::PParser::new().parse(s).map_err(Box::from)
    }
}

pub mod three_addr {
    use super::{BinOp, Expr, Stmts, UnOp};
    use std::error::Error;
    use std::fmt::{self, Debug, Formatter};

    pub type Var = String;

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub enum RValue {
        Var(Var),
        Lit(usize),
    }

    impl Debug for RValue {
        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            match self {
                RValue::Var(var) => write!(f, "{}", var),
                RValue::Lit(lit) => write!(f, "{}", lit),
            }
        }
    }

    impl From<String> for RValue {
        fn from(var: String) -> RValue {
            RValue::Var(var)
        }
    }

    impl From<usize> for RValue {
        fn from(lit: usize) -> RValue {
            RValue::Lit(lit)
        }
    }

    pub enum Instr {
        Bin(BinOp, RValue, RValue, Var),
        Un(UnOp, RValue, Var),
        Access(Var, RValue, Var),
        Copy(RValue, Var),
    }

    impl Debug for Instr {
        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            match self {
                Instr::Bin(op, lhs, rhs, res) => {
                    write!(f, "{} = {:?} {} {:?}", res, lhs, op.symbol(), rhs)
                }
                Instr::Un(op, inner, res) => write!(f, "{} = {} {:?}", res, op.symbol(), inner),
                Instr::Access(inner, idx, res) => write!(f, "{} = {} [{:?}]", res, inner, idx),
                Instr::Copy(source, res) => write!(f, "{} = {:?}", res, source),
            }
        }
    }

    #[derive(Default)]
    pub struct ProcBuilder {
        instrs: Vec<Instr>,
        temp: usize,
    }

    pub trait Walkable {
        fn walk_into(&self, builder: &mut ProcBuilder);
    }

    impl Walkable for Stmts {
        fn walk_into(&self, builder: &mut ProcBuilder) {
            self.walk(builder);
        }
    }

    impl Walkable for Expr {
        fn walk_into(&self, builder: &mut ProcBuilder) {
            self.walk(builder);
        }
    }

    impl ProcBuilder {
        pub fn new() -> Self {
            ProcBuilder::default()
        }

        pub fn new_temp(&mut self) -> Var {
            let var = format!("t{}", self.temp);
            self.temp += 1;
            var
        }

        pub fn push(&mut self, instr: Instr) {
            self.instrs.push(instr);
        }

        pub fn build<W: Walkable>(w: &W) -> Vec<Instr> {
            let mut builder = ProcBuilder::new();
            w.walk_into(&mut builder);
            builder.instrs
        }

        pub fn parse<'a>(s: &'a str) -> Result<Vec<Instr>, Box<Error + 'a>> {
            let stmts = Stmts::parse(s)?;
            Ok(ProcBuilder::build(&stmts))
        }
    }
}

#[test]
fn dedup_test() {
    let dag = DAG::parse("a+a*(b-c)+(b-c)*d").unwrap();
    assert_eq!(dag.size(), 9);
}

#[test]
fn build_test() {
    let stmts = Stmts::parse("a = b +-c;").unwrap();
    let instrs = ProcBuilder::build(&stmts);
    // println!("{:?}", instrs);
    assert_eq!(instrs.len(), 3);

    let expr = Expr::parse("c + a[i][j]").unwrap();
    let instrs = ProcBuilder::build(&*expr);
    // println!("{:#?}", instrs);
    assert_eq!(instrs.len(), 5);
}
