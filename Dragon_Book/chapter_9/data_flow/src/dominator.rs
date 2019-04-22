use crate::framework::{DataFlow, Forward, SemiLattice, Transfer};
use crate::{BlockID, Program};
use petgraph::prelude::*;
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
struct Dominator {
    ids: HashSet<BlockID>,
}

impl SemiLattice<'_> for Dominator {
    fn top(program: &Program) -> Self {
        let ids = program.block_range().collect();
        Dominator { ids }
    }

    fn start(program: &Program) -> Self {
        let mut ids = HashSet::new();
        ids.insert(program.entry());
        Dominator { ids }
    }

    fn meet(&self, other: &Self) -> Self {
        let ids = &self.ids & &other.ids;
        Dominator { ids }
    }
}

#[derive(Clone)]
struct DominatorT {
    block_id: BlockID,
}

impl Transfer<'_> for DominatorT {
    type Target = Dominator;
    type Extra = ();

    fn new(block_id: BlockID, _: &Program, _: &()) -> Self {
        DominatorT { block_id }
    }

    fn apply(&self, in_value: &Self::Target) -> Self::Target {
        let mut ids = in_value.ids.clone();
        ids.insert(self.block_id);
        Dominator { ids }
    }
}

pub struct Dominators {
    sets: Vec<HashSet<BlockID>>,
    entry: BlockID,
    exit: Option<BlockID>,
}

impl Dominators {
    pub fn rel(&self, from: BlockID, to: BlockID) -> bool {
        self.sets[to].contains(&from)
    }

    pub fn tree(&self) -> GraphMap<BlockID, (), Directed> {
        let least = |n: BlockID, m: BlockID| -> BlockID {
            if self.sets[n].contains(&m) {
                n
            } else {
                m
            }
        };

        let mut tree = GraphMap::new();

        for id in 0..self.sets.len() {
            tree.add_node(id);
        }

        for (id, doms) in self.sets.iter().enumerate() {
            let opt_imm = doms
                .iter()
                .filter(|dom| **dom != id)
                .fold(None, |opt, dom| {
                    if let Some(imm) = opt {
                        Some(least(imm, *dom))
                    } else {
                        Some(*dom)
                    }
                });

            if let Some(imm) = opt_imm {
                tree.add_edge(imm, id, ());
            }
        }

        tree
    }
}

impl std::fmt::Debug for Dominators {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (i, doms) in self.sets.iter().enumerate() {
            let name = if i == self.entry {
                "ENTRY".into()
            } else if Some(i) == self.exit {
                "EXIT".into()
            } else {
                format!("B{}", i)
            };

            writeln!(f, "Dominator of {}: {:?}", name, doms)?;
        }

        Ok(())
    }
}

pub fn dominators(program: &Program) -> Dominators {
    let sets = DataFlow::<Dominator, Forward, DominatorT>::run(program, &())
        .into_iter()
        .map(|(_, out_value)| out_value.ids)
        .collect();

    Dominators {
        sets,
        entry: program.entry(),
        exit: program.exit(),
    }
}

#[cfg(test)]
pub fn figure_9_38() -> Program {
    use crate::Block;
    Program::new(
        vec![
            Block::entry(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
        ],
        &[
            (0, 1),
            (1, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 5),
            (4, 6),
            (5, 7),
            (6, 7),
            (7, 4),
            (7, 8),
            (8, 3),
            (8, 9),
            (8, 10),
            (9, 1),
            (10, 7),
        ],
    )
}

#[test]
fn dominators_test() {
    let program = figure_9_38();
    let dominators = dominators(&program).sets;
    assert_eq!(dominators[4], vec![1, 3, 4, 0].into_iter().collect());
    assert_eq!(dominators[5], vec![1, 3, 4, 5, 0].into_iter().collect());
    assert_eq!(dominators[6], vec![1, 3, 4, 6, 0].into_iter().collect());
    assert_eq!(dominators[7], vec![1, 3, 4, 7, 0].into_iter().collect());
    assert_eq!(dominators[8], vec![1, 3, 4, 7, 8, 0].into_iter().collect());
    assert_eq!(
        dominators[9],
        vec![1, 3, 4, 7, 8, 9, 0].into_iter().collect()
    );
    assert_eq!(
        dominators[10],
        vec![1, 3, 4, 7, 8, 10, 0].into_iter().collect()
    );
}
