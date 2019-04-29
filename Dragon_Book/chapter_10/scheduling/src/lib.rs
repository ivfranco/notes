#[macro_use]
extern crate lalrpop_util;

use petgraph::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;

lalrpop_mod!(pub code);

pub type Mem = String;
pub type Reg = u8;
pub type Lit = usize;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Addr {
    Reg(Reg),
    Mem(Mem),
    Idx(Lit, Reg),
}

impl Addr {
    fn is_indirect(&self) -> bool {
        if let Addr::Idx(..) = self {
            true
        } else {
            false
        }
    }

    fn is_memory(&self) -> bool {
        if let Addr::Reg(..) = self {
            false
        } else {
            true
        }
    }

    fn reads(&self) -> HashSet<Addr> {
        if let Addr::Idx(_, r) = self {
            vec![self.clone(), Addr::Reg(*r)].into_iter().collect()
        } else {
            Some(self.clone()).into_iter().collect()
        }
    }
}

impl std::fmt::Debug for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Addr::*;
        match self {
            Reg(r) => write!(f, "R{}", r),
            Mem(m) => write!(f, "{}", m),
            Idx(l, r) => write!(f, "{}(R{})", l, r),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
}

impl std::fmt::Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Op::*;
        match self {
            Add => write!(f, "ADD"),
            Sub => write!(f, "SUB"),
            Mul => write!(f, "MUL"),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Code {
    Ld(Reg, Addr),
    St(Addr, Reg),
    Op(Op, Reg, Reg, Reg),
}

impl Code {
    fn reads(&self) -> HashSet<Addr> {
        use Addr::*;
        use Code::*;

        match self {
            Ld(_, a) => a.reads(),
            St(a, r) => {
                let mut set = HashSet::new();
                set.insert(Reg(*r));
                if let Idx(_, r) = a {
                    set.insert(Reg(*r));
                }
                set
            }
            Op(_, _, lhs, rhs) => vec![Reg(*lhs), Reg(*rhs)].into_iter().collect(),
        }
    }

    fn writes(&self) -> HashSet<Addr> {
        use Addr::*;
        use Code::*;

        match self {
            Ld(r, _) => Some(Reg(*r)).into_iter().collect(),
            St(a, _) => Some(a.clone()).into_iter().collect(),
            Op(_, dst, _, _) => Some(Reg(*dst)).into_iter().collect(),
        }
    }

    pub fn dependency(&self, later: &Self) -> bool {
        fn indirect_access(set: &HashSet<Addr>) -> bool {
            set.iter().any(Addr::is_indirect)
        }

        fn memory_access(set: &HashSet<Addr>) -> bool {
            set.iter().any(Addr::is_memory)
        }

        let self_writes = self.writes();
        let later_reads = later.reads();
        let true_dependency = (indirect_access(&self_writes) && memory_access(&later_reads))
            || !self_writes.is_disjoint(&later_reads);

        let self_reads = self.reads();
        let later_writes = later.writes();
        let anti_dependency = (memory_access(&self_reads) && indirect_access(&later_writes))
            || !self_reads.is_disjoint(&later_writes);

        let output_dependency = (indirect_access(&self_writes) && memory_access(&later_writes))
            || (indirect_access(&later_writes) && memory_access(&self_writes))
            || !self_writes.is_disjoint(&later_writes);

        true_dependency || anti_dependency || output_dependency
    }
}

impl std::fmt::Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Code::*;
        match self {
            Ld(r, a) => write!(f, "LD R{}, {:?}", r, a),
            St(a, r) => write!(f, "ST {:?}, R{}", a, r),
            Op(op, dst, lhs, rhs) => write!(f, "{:?} R{}, R{}, R{}", op, dst, lhs, rhs),
        }
    }
}

pub struct Binary {
    codes: Vec<Code>,
}

impl Binary {
    fn new(codes: Vec<Code>) -> Self {
        Binary { codes }
    }

    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        code::BinaryParser::new().parse(s).map_err(Box::from)
    }

    pub fn dependency_graph<F, D>(&self, mut delay: F) -> DiGraph<&Code, D>
    where
        F: FnMut(&Code, &Code) -> D,
    {
        let mut graph = DiGraph::new();
        let mut node_map: HashMap<&Code, NodeIndex> = HashMap::new();

        for code in &self.codes {
            let idx = graph.add_node(code);
            node_map.insert(code, idx);
        }

        for (i, earlier) in self.codes.iter().enumerate() {
            for later in &self.codes[i + 1..] {
                if earlier.dependency(later) {
                    graph.add_edge(node_map[earlier], node_map[later], delay(earlier, later));
                }
            }
        }

        graph
    }
}

impl std::fmt::Debug for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for code in &self.codes {
            writeln!(f, "{:?}", code)?;
        }
        Ok(())
    }
}

#[cfg(test)]
const FIGURE_10_10_A: &str = "
LD R1, a
LD R2, b
SUB R3, R1, R2
ADD R2, R1, R2
ST a, R3
ST b, R2
";

#[test]
fn parse_test() {
    use crate::Op::*;
    use Addr::*;
    use Code::*;

    let binary = Binary::parse(FIGURE_10_10_A).unwrap();
    let codes = vec![
        Ld(1, Mem("a".into())),
        Ld(2, Mem("b".into())),
        Op(Sub, 3, 1, 2),
        Op(Add, 2, 1, 2),
        St(Mem("a".into()), 3),
        St(Mem("b".into()), 2),
    ];
    assert_eq!(binary.codes, codes);
}

#[test]
fn dependency_graph_test() {
    let binary = Binary::parse(FIGURE_10_10_A).unwrap();
    let graph = binary.dependency_graph(|_, _| ());
    assert_eq!(graph.edge_count(), 9);
}
