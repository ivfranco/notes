use crate::dominator::Dominators;
use crate::{BlockID, Program};
use petgraph::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Region {
    head: BlockID,
    body: DiGraphMap<BlockID, ()>,
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head
            && self.node_count() == other.node_count()
            && self.body.edge_count() == other.body.edge_count()
            && self.body.nodes().all(|node| other.body.contains_node(node))
            && self
                .body
                .all_edges()
                .all(|(from, to, _)| other.body.contains_edge(from, to))
    }
}

impl std::fmt::Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Head: {}", self.head)?;
        writeln!(f, "Nodes: {:?}", self.body.nodes().collect::<Vec<_>>())?;
        writeln!(
            f,
            "Edges: {:?}",
            self.body
                .all_edges()
                .map(|(from, to, _)| (from, to))
                .collect::<Vec<_>>()
        )?;
        Ok(())
    }
}

impl Region {
    fn new(program: &Program, head: BlockID, nodes: &HashSet<BlockID>) -> Self {
        let body: DiGraphMap<BlockID, ()> = DiGraphMap::from_edges(
            program
                .edges()
                .into_iter()
                .filter(|(from, to)| nodes.contains(from) && nodes.contains(to)),
        );

        Region { head, body }
    }

    fn new_trivial(head: BlockID) -> Self {
        let mut body = DiGraphMap::new();
        body.add_node(head);
        Region { head, body }
    }

    fn new_complete(program: &Program) -> Self {
        Region {
            head: program.entry(),
            body: program.graph().clone(),
        }
    }

    fn from_back_edge(program: &Program, from: BlockID, to: BlockID) -> (Self, Self) {
        let nodes = program.natural_loop(from, to);
        let loop_region = Region::new(program, to, &nodes);
        let mut body_region = loop_region.clone();
        body_region.body.remove_edge(from, to);
        (body_region, loop_region)
    }

    fn node_count(&self) -> usize {
        self.body.node_count()
    }
}

pub fn possible_nontrivial_regions(program: &Program) -> Vec<Region> {
    let dominators = Dominators::new(program);

    program
        .block_range()
        .filter_map(|i| {
            let nodes = dominators.dominated(i);
            if nodes.len() > 1 {
                Some(Region::new(program, i, &nodes))
            } else {
                None
            }
        })
        .collect()
}

pub fn nested_regions(program: &Program) -> Vec<Region> {
    let mut regions = vec![];

    for block_id in program.block_range() {
        regions.push(Region::new_trivial(block_id));
    }

    let dominators = Dominators::new(program);

    for (from, to) in dominators.back_edges() {
        let (body_region, loop_region) = Region::from_back_edge(program, from, to);
        regions.push(body_region);
        regions.push(loop_region);
    }

    if regions
        .iter()
        .all(|region| region.node_count() < program.len())
    {
        regions.push(Region::new_complete(program));
    }

    regions
}

#[cfg(test)]
fn figure_9_48_flow_only() -> Program {
    use crate::Block;

    Program::with_entry_exit(
        vec![Block::empty(), Block::empty(), Block::empty()],
        &[(0, 1), (1, 2), (1, 3), (2, 3), (2, 4), (3, 1), (3, 4)],
    )
}

#[test]
fn regions_test() {
    let program = figure_9_48_flow_only();
    let regions = nested_regions(&program);
    println!("{:?}", regions);
    assert_eq!(regions.len(), 8);

    let body_region = Region {
        head: 1,
        body: DiGraphMap::from_edges(&[(1, 2), (1, 3), (2, 3)]),
    };

    let mut loop_region = body_region.clone();
    loop_region.body.add_edge(3, 1, ());

    assert!(regions.contains(&loop_region));
    assert!(regions.contains(&body_region));
    assert!(regions.contains(&Region::new_complete(&program)))
}
