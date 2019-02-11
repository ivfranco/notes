#[allow(unused_imports)]
use pest::{iterators::Pair, Parser};
use std::fmt::{self, Debug, Formatter};

#[derive(Parser)]
#[grammar = "./block.pest"]
struct BlockParser;

pub struct Block {
    pub decls: Vec<Decl>,
    pub stmts: Vec<Stmt>,
}

impl Block {
    fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::block);

        let mut pairs = pair.into_inner();

        let decls = pairs
            .next()
            .unwrap()
            .into_inner()
            .map(Decl::from_pair)
            .collect();

        let stmts = pairs
            .next()
            .unwrap()
            .into_inner()
            .map(Stmt::from_pair)
            .collect();

        Block { decls, stmts }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{{")?;
        for decl in &self.decls {
            write!(f, " {:?}", decl)?;
        }
        for stmt in &self.stmts {
            write!(f, " {:?}", stmt)?;
        }
        write!(f, " }}")
    }
}

pub struct Decl {
    pub ty: String,
    pub id: String,
}

impl Decl {
    fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::decl);

        let mut pairs = pair.into_inner();
        let ty = pairs.next().unwrap().as_str().to_owned();
        let id = pairs.next().unwrap().as_str().to_owned();

        Decl { ty, id }
    }
}

impl Debug for Decl {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {};", self.ty, self.id)
    }
}

pub enum Stmt {
    Block(Block),
    Id(String),
}

impl Stmt {
    fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::stmt);

        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::block => Stmt::Block(Block::from_pair(inner)),
            Rule::id => Stmt::Id(inner.as_str().to_owned()),
            _ => unreachable!(),
        }
    }
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Stmt::Block(block) => Debug::fmt(block, f),
            Stmt::Id(id) => write!(f, "{};", id),
        }
    }
}

pub fn parse(input: &str) -> Block {
    let pair = BlockParser::parse(Rule::block, input)
        .expect("Grammar definition error")
        .next()
        .unwrap();
    Block::from_pair(pair)
}

#[test]
fn parse_test() {
    let input = "{ int x; char y; { bool y; x; y; } x; y; }";
    assert_eq!(format!("{:?}", parse(input)), input);
}
