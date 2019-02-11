use crate::parser::{parse, Block, Stmt};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::process;

pub struct SBlock {
    stmts: Vec<SStmt>,
}

impl SBlock {
    fn translate(block: Block, env: &mut Env) -> Self {
        env.push();
        for decl in block.decls {
            env.declare(decl.id, decl.ty);
        }

        let stmts = block
            .stmts
            .into_iter()
            .map(|stmt| SStmt::new(stmt, env))
            .collect();

        env.discard();

        SBlock { stmts }
    }
}

impl Debug for SBlock {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{{")?;
        for stmt in &self.stmts {
            write!(f, " {:?}", stmt)?;
        }
        write!(f, " }}")
    }
}

enum SStmt {
    Block(SBlock),
    Use(String, String),
}

impl SStmt {
    fn new(stmt: Stmt, env: &mut Env) -> Self {
        match stmt {
            Stmt::Block(block) => {
                let sblock = SBlock::translate(block, env);
                SStmt::Block(sblock)
            }
            Stmt::Id(id) => {
                let ty = env.get(&id).unwrap_or_else(|| {
                    eprintln!("Error: Undeclared use of variable {}", id);
                    process::exit(1);
                });

                SStmt::Use(id, ty.to_owned())
            }
        }
    }
}

impl Debug for SStmt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            SStmt::Block(block) => Debug::fmt(block, f),
            SStmt::Use(id, ty) => write!(f, "{}:{};", id, ty),
        }
    }
}

#[derive(Default)]
struct Env {
    tables: Vec<HashMap<String, String>>,
}

impl Env {
    fn new() -> Self {
        Env::default()
    }

    fn top_mut(&mut self) -> &mut HashMap<String, String> {
        self.tables
            .last_mut()
            .expect("Error: Empty stack of symbol tables")
    }

    fn declare(&mut self, var: String, ty: String) {
        self.top_mut().insert(var.to_owned(), ty.to_owned());
    }

    fn get(&self, var: &str) -> Option<&str> {
        self.tables
            .iter()
            .rev()
            .find_map(|table| table.get(var))
            .map(|s| s.as_str())
    }

    fn push(&mut self) {
        self.tables.push(HashMap::new());
    }

    fn discard(&mut self) {
        self.tables.pop();
    }
}

pub fn translate(input: &str) -> SBlock {
    let block = parse(input);
    let mut env = Env::new();
    SBlock::translate(block, &mut env)
}

#[test]
fn translate_test() {
    let input = "{ int x; char y; { bool y; x; y; } x; y; }";
    assert_eq!(
        format!("{:?}", translate(input)),
        "{ { x:int; y:bool; } x:int; y:char; }"
    );
}
