use self::three_addr::{Instr, Label, ProcBuilder, RValue, Var};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

pub(crate) mod global {
    use super::{Boolean, Expr, Stmt};
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;
    use std::rc::Rc;

    #[derive(Default)]
    struct NodeSet {
        exprs: HashSet<Rc<Expr>>,
        bools: HashSet<Rc<Boolean>>,
        stmts: HashSet<Rc<Stmt>>,
    }

    fn dedup<T>(set: &mut HashSet<Rc<T>>, elem: T) -> Rc<T>
    where
        T: Eq + Hash,
    {
        let rc = Rc::new(elem);
        if let Some(v) = set.get(&rc) {
            v.clone()
        } else {
            set.insert(rc.clone());
            rc
        }
    }

    impl NodeSet {
        pub fn dedeup_expr(&mut self, expr: Expr) -> Rc<Expr> {
            dedup(&mut self.exprs, expr)
        }

        pub fn dedup_bool(&mut self, boolean: Boolean) -> Rc<Boolean> {
            dedup(&mut self.bools, boolean)
        }

        pub fn dedup_stmt(&mut self, stmt: Stmt) -> Rc<Stmt> {
            dedup(&mut self.stmts, stmt)
        }

        pub fn enumerate(&self) -> NodeMap {
            unimplemented!()
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    enum Node {
        Expr(Rc<Expr>),
        Bool(Rc<Boolean>),
        Stmt(Rc<Stmt>),
    }

    pub struct NodeMap {
        nodes: HashMap<Node, usize>,
    }
}

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
    And,
    Or,
    Rel(RelOp),
}

impl BinOp {
    fn symbol(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::Rel(op) => op.symbol(),
        }
    }
}

impl From<RelOp> for BinOp {
    fn from(op: RelOp) -> Self {
        BinOp::Rel(op)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnOp {
    Pos,
    Neg,
    Not,
}

impl UnOp {
    fn symbol(self) -> &'static str {
        match self {
            UnOp::Pos => "+",
            UnOp::Neg => "-",
            UnOp::Not => "!",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Bin(BinOp, Rc<Expr>, Rc<Expr>),
    Un(UnOp, Rc<Expr>),
    Bool(Boolean),
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
            Expr::Bool(_boolean) => unimplemented!(),
            Expr::Access(access) => writeln!(f, "{:?}", access),
            Expr::Var(var) => writeln!(f, "{}", var),
        }
    }

    fn walk(&self, builder: &mut ProcBuilder) -> RValue {
        match self {
            Expr::Bin(op, lhs, rhs) => {
                let l = lhs.walk(builder);
                let r = rhs.walk(builder);
                let t = builder.new_temp();
                let instr = Instr::Bin(*op, l, r, t.clone());
                builder.push(instr);
                t.into()
            }
            Expr::Un(op, inner) => {
                let inn = inner.walk(builder);
                let t = builder.new_temp();
                let instr = Instr::Un(*op, inn, t.clone());
                builder.push(instr);
                t.into()
            }
            Expr::Bool(boolean) => boolean.rwalk(builder),
            Expr::Access(access) => access.rwalk(builder),
            Expr::Var(var) => var.clone().into(),
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Access {
    var: String,
    dims: Vec<Rc<Expr>>,
}

impl Access {
    fn lwalk(&self, builder: &mut ProcBuilder) -> (Var, RValue) {
        assert!(!self.dims.is_empty());

        let mut t = self.dims[0].walk(builder);
        if self.dims.len() > 1 {
            let prod = builder.new_temp();

            for (i, expr) in self.dims.iter().skip(1).enumerate() {
                let width: RValue = format!("{}.dim{}", self.var, i).into();
                builder.push(Instr::Bin(BinOp::Mul, t, width, prod.clone()));
                let next = builder.new_temp();
                let dim = expr.walk(builder);
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

    fn rwalk(&self, builder: &mut ProcBuilder) -> RValue {
        let (var, idx) = self.lwalk(builder);
        let t = builder.new_temp();
        let instr = Instr::Access(var, idx, t.clone());
        builder.push(instr);
        t.into()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Boolean {
    And(Box<Boolean>, Box<Boolean>),
    Or(Box<Boolean>, Box<Boolean>),
    Not(Box<Boolean>),
    Rel(RelOp, Rc<Expr>, Rc<Expr>),
    True,
    False,
}

impl Boolean {
    fn rwalk(&self, builder: &mut ProcBuilder) -> RValue {
        use self::Boolean::*;
        match self {
            And(lhs, rhs) => {
                let l = lhs.rwalk(builder);
                let r = rhs.rwalk(builder);
                let t = builder.new_temp();
                let instr = Instr::Bin(BinOp::And, l, r, t.clone());
                builder.push(instr);
                t.into()
            }
            Or(lhs, rhs) => {
                let l = lhs.rwalk(builder);
                let r = rhs.rwalk(builder);
                let t = builder.new_temp();
                let instr = Instr::Bin(BinOp::Or, l, r, t.clone());
                builder.push(instr);
                t.into()
            }
            Not(inner) => {
                let inn = inner.rwalk(builder);
                let t = builder.new_temp();
                let instr = Instr::Un(UnOp::Not, inn, t.clone());
                builder.push(instr);
                t.into()
            }
            Rel(op, lhs, rhs) => {
                let l = lhs.walk(builder);
                let r = rhs.walk(builder);
                let t = builder.new_temp();
                let instr = Instr::Bin((*op).into(), l, r, t.clone());
                builder.push(instr);
                t.into()
            }
            True => RValue::True,
            False => RValue::False,
        }
    }

    fn jwalk(&self, t: Option<Label>, f: Option<Label>, builder: &mut ProcBuilder) {
        use self::Boolean::*;
        match self {
            And(lhs, rhs) => {
                // if f == None, lhs cannot fall through to rhs on false
                let safe_net = f.unwrap_or_else(|| builder.new_label());
                lhs.jwalk(None, Some(safe_net), builder);
                rhs.jwalk(t, f, builder);
                if f.is_none() {
                    builder.attach_label(safe_net);
                }
            }
            Or(lhs, rhs) => {
                // if t == None, lhs cannot fall through to rhs on true
                let safe_net = t.unwrap_or_else(|| builder.new_label());
                lhs.jwalk(Some(safe_net), None, builder);
                rhs.jwalk(t, f, builder);
                if t.is_none() {
                    builder.attach_label(safe_net);
                }
            }
            // swaps true and false destination
            Not(inner) => inner.jwalk(f, t, builder),
            Rel(op, lhs, rhs) => {
                let l = lhs.walk(builder);
                let r = rhs.walk(builder);
                match (t, f) {
                    // both true and false fall through
                    (None, None) => (),
                    // false fall through
                    (Some(t), None) => {
                        let instr = Instr::IfTrue(*op, l, r, t);
                        builder.push(instr);
                    }
                    // true fall through
                    (None, Some(f)) => {
                        let instr = Instr::IfFalse(*op, l, r, f);
                        builder.push(instr);
                    }
                    (Some(t), Some(f)) => {
                        let instr = Instr::IfTrue(*op, l, r, t);
                        let goto = Instr::Goto(f);
                        builder.push(instr);
                        builder.push(goto);
                    }
                }
            }
            True => builder.push_goto(t),
            False => builder.push_goto(f),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Assign(Assign),
    If(Boolean, Box<Stmt>),
    IfElse(Boolean, Box<Stmt>, Box<Stmt>),
    While(Boolean, Box<Stmt>),
}

impl Stmt {
    fn walk(&self, next: Option<Label>, builder: &mut ProcBuilder) {
        match self {
            Stmt::Assign(assign) => {
                assign.walk(builder);
                builder.push_goto(next);
            }
            Stmt::If(cond, body) => {
                // if next == None, cond clause cannot fall through to body clause
                let safe_net = next.unwrap_or_else(|| builder.new_label());

                cond.jwalk(None, Some(safe_net), builder);
                body.walk(next, builder);
                if next.is_none() {
                    builder.attach_label(safe_net);
                }
            }
            Stmt::IfElse(cond, t_clause, f_clause) => {
                let f = builder.new_label();
                // if next == None, true clause cannot fall through to false clause
                let safe_net = next.unwrap_or_else(|| builder.new_label());

                cond.jwalk(None, Some(f), builder);
                t_clause.walk(Some(safe_net), builder);
                builder.attach_label(f);
                f_clause.walk(next, builder);
                if next.is_none() {
                    builder.attach_label(safe_net);
                }
            }
            Stmt::While(cond, body) => {
                let top = builder.new_label();
                let end = next.unwrap_or_else(|| builder.new_label());

                builder.attach_label(top);
                cond.jwalk(None, Some(end), builder);
                body.walk(Some(top), builder);
                if next.is_none() {
                    builder.attach_label(end);
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum LValue {
    Var(Var),
    Access(Access),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Assign {
    lvalue: LValue,
    rvalue: Rc<Expr>,
}

impl Assign {
    fn walk(&self, builder: &mut ProcBuilder) {
        let rhs = self.rvalue.walk(builder);
        match &self.lvalue {
            LValue::Var(var) => {
                let instr = Instr::Copy(rhs, var.clone());
                builder.push(instr);
            }
            LValue::Access(access) => {
                let (var, idx) = access.lwalk(builder);
                let arr = if let RValue::Var(var) = rhs {
                    var
                } else {
                    panic!("Error: Array evaluates to literals");
                };
                let instr = Instr::Assign(arr, idx, var);
                builder.push(instr);
            }
        }
    }
}

pub struct Stmts {
    stmts: Vec<Stmt>,
}

impl Stmts {
    pub fn walk(&self, builder: &mut ProcBuilder) {
        for stmt in &self.stmts {
            stmt.walk(None, builder);
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
    use super::{BinOp, Expr, RelOp, Stmts, UnOp};
    use std::error::Error;
    use std::fmt::{self, Debug, Formatter};
    use std::mem;

    pub type Var = String;

    pub type Label = usize;

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub enum RValue {
        Var(Var),
        Lit(usize),
        True,
        False,
    }

    impl Debug for RValue {
        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            match self {
                RValue::Var(var) => write!(f, "{}", var),
                RValue::Lit(lit) => write!(f, "{}", lit),
                RValue::True => write!(f, "true"),
                RValue::False => write!(f, "false"),
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
        Noop,
        Bin(BinOp, RValue, RValue, Var),
        Un(UnOp, RValue, Var),
        Access(Var, RValue, Var),
        Assign(Var, RValue, Var),
        Copy(RValue, Var),
        IfTrue(RelOp, RValue, RValue, Label),
        IfFalse(RelOp, RValue, RValue, Label),
        Goto(Label),
    }

    impl Debug for Instr {
        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            match self {
                Instr::Noop => write!(f, "Noop"),
                Instr::Bin(op, lhs, rhs, res) => {
                    write!(f, "{} = {:?} {} {:?}", res, lhs, op.symbol(), rhs)
                }
                Instr::Un(op, inner, res) => write!(f, "{} = {} {:?}", res, op.symbol(), inner),
                Instr::Access(inner, idx, res) => write!(f, "{} = {} [{:?}]", res, inner, idx),
                Instr::Assign(arr, idx, rhs) => write!(f, "{} [{:?}] = {}", arr, idx, rhs),
                Instr::Copy(source, res) => write!(f, "{} = {:?}", res, source),
                Instr::IfTrue(op, lhs, rhs, label) => write!(
                    f,
                    "IfTrue {:?} {} {:?} goto L{}",
                    lhs,
                    op.symbol(),
                    rhs,
                    label
                ),
                Instr::IfFalse(op, lhs, rhs, label) => write!(
                    f,
                    "IfFalse {:?} {} {:?} goto L{}",
                    lhs,
                    op.symbol(),
                    rhs,
                    label
                ),
                Instr::Goto(label) => write!(f, "Goto L{}", label),
            }
        }
    }

    pub struct Line {
        labels: Vec<Label>,
        instr: Instr,
    }

    impl Debug for Line {
        fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
            if !self.labels.is_empty() {
                let prefix = self
                    .labels
                    .iter()
                    .map(|l| format!("L{}", l))
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{}: ", prefix)?;
            }

            write!(f, "{:?}", self.instr)
        }
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

    #[derive(Default)]
    pub struct ProcBuilder {
        lines: Vec<Line>,
        next_temp: usize,
        next_label: usize,
        labels: Vec<Label>,
    }

    impl ProcBuilder {
        pub fn new() -> Self {
            ProcBuilder::default()
        }

        pub fn new_temp(&mut self) -> Var {
            let var = format!("t{}", self.next_temp);
            self.next_temp += 1;
            var
        }

        pub fn new_label(&mut self) -> Label {
            let label = self.next_label;
            self.next_label += 1;
            label
        }

        pub fn attach_label(&mut self, label: Label) {
            self.labels.push(label);
        }

        pub fn attach_opt(&mut self, label: Option<Label>) {
            self.labels.extend(label.into_iter());
        }

        pub fn push(&mut self, instr: Instr) {
            let labels = mem::replace(&mut self.labels, vec![]);
            self.lines.push(Line { labels, instr })
        }

        pub fn push_goto(&mut self, label: Option<Label>) {
            if let Some(l) = label {
                let instr = Instr::Goto(l);
                self.push(instr);
            }
        }

        pub fn build<W: Walkable>(w: &W) -> Vec<Line> {
            let mut builder = ProcBuilder::new();
            w.walk_into(&mut builder);
            if !builder.labels.is_empty() {
                builder.push(Instr::Noop);
            }
            builder.lines
        }

        pub fn parse<'a>(s: &'a str) -> Result<Vec<Line>, Box<Error + 'a>> {
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
