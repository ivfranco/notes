#![allow(clippy::implicit_hasher)]

use crate::framework::{Backward, DataFlow, Forward, SemiLattice, Transfer};
use crate::{Block, BlockID, Expr, Program, Stmt};
use std::collections::HashSet;

type PairVec<T> = Vec<(T, T)>;
pub type PairSlice<T> = [(T, T)];

#[allow(dead_code)]
fn cut_edges(mut program: Program) -> Program {
    for (from, to) in program.edges() {
        if program.predecessors(to).count() > 1 {
            program.insert_empty_between(from, to);
        }
    }

    program
}

#[derive(Clone, PartialEq, Default)]
pub struct Anticipate<'a> {
    pub exprs: HashSet<Expr<'a>>,
}

impl<'a> std::fmt::Debug for Anticipate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(&self.exprs, f)
    }
}

impl<'a> SemiLattice<'a> for Anticipate<'a> {
    fn top(program: &'a Program) -> Self {
        let exprs = program.exprs().collect();
        Anticipate { exprs }
    }

    fn start(_: &'a Program) -> Self {
        Anticipate {
            exprs: HashSet::new(),
        }
    }

    fn meet(&self, other: &Self) -> Self {
        Anticipate {
            exprs: &self.exprs & &other.exprs,
        }
    }
}

#[derive(Clone)]
struct AnticipateT<'a> {
    gen: HashSet<Expr<'a>>,
    kill: HashSet<&'a str>,
}

impl<'a> AnticipateT<'a> {
    fn new(block: &'a Block) -> Self {
        let mut gen = HashSet::new();
        let mut kill = HashSet::new();

        for stmt in block.stmts().rev() {
            gen.extend(stmt.as_expr());
            if let Some(def) = stmt.def() {
                gen.retain(|expr| !expr.uses(def));
                kill.insert(def);
            }
        }

        AnticipateT { gen, kill }
    }
}

impl<'a> Transfer<'a> for AnticipateT<'a> {
    type Target = Anticipate<'a>;
    type Extra = ();

    fn new(block_id: BlockID, program: &'a Program, _: &()) -> Self {
        AnticipateT::new(
            program
                .get_block(block_id)
                .expect("AnticipateT: BlockID in bound"),
        )
    }

    fn apply(&self, anticipate: &Self::Target) -> Self::Target {
        let out_exprs = &anticipate.exprs;
        let in_exprs = out_exprs
            .iter()
            .filter(|expr| self.kill.iter().all(|var| !expr.uses(var)))
            .chain(self.gen.iter())
            .cloned()
            .collect();
        Anticipate { exprs: in_exprs }
    }
}

pub fn anticipates(program: &Program) -> PairVec<Anticipate<'_>> {
    DataFlow::<Anticipate<'_>, Backward, AnticipateT<'_>>::run(program, &()).into_pairs()
}

#[derive(Clone, PartialEq, Default)]
pub struct Available<'a> {
    pub exprs: HashSet<Expr<'a>>,
}

impl<'a> std::fmt::Debug for Available<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(&self.exprs, f)
    }
}

impl<'a> SemiLattice<'a> for Available<'a> {
    fn top(program: &'a Program) -> Self {
        Available {
            exprs: program.exprs().collect(),
        }
    }

    fn start(_: &'a Program) -> Self {
        Available {
            exprs: HashSet::new(),
        }
    }

    fn meet(&self, other: &Self) -> Self {
        let exprs = &self.exprs & &other.exprs;
        Available { exprs }
    }
}

#[derive(Clone)]
struct AvailableT<'a> {
    gen: HashSet<Expr<'a>>,
    kill: HashSet<&'a str>,
}

impl<'a> AvailableT<'a> {
    fn new(block: &'a Block, anticipate_in: &Anticipate<'a>) -> Self {
        let gen = anticipate_in.exprs.clone();
        let kill = block.stmts().filter_map(Stmt::def).collect();
        AvailableT { gen, kill }
    }
}

impl<'a> Transfer<'a> for AvailableT<'a> {
    type Target = Available<'a>;
    type Extra = [&'a Anticipate<'a>];

    fn new(block_id: BlockID, program: &'a Program, extra: &[&'a Anticipate<'a>]) -> Self {
        let block = program
            .get_block(block_id)
            .expect("AvailableT: Block in-bound");
        let anticipate_in = extra
            .get(block_id)
            .expect("AvailableT: Extra data in-bound");
        AvailableT::new(block, anticipate_in)
    }

    fn apply(&self, value: &Self::Target) -> Self::Target {
        let exprs = (&self.gen | &value.exprs)
            .into_iter()
            .filter(|expr| self.kill.iter().all(|var| !expr.uses(var)))
            .collect();

        Available { exprs }
    }
}

pub fn availables<'a>(
    program: &'a Program,
    anticipates: &'a PairSlice<Anticipate<'a>>,
) -> PairVec<Available<'a>> {
    let anticipate_ins: Vec<_> = anticipates.iter().map(|(in_value, _)| in_value).collect();
    DataFlow::<Available<'a>, Forward, AvailableT<'a>, [&'a Anticipate<'a>]>::run(
        program,
        &anticipate_ins,
    )
    .into_pairs()
}

pub fn earliests<'a>(
    anticipates: &PairSlice<Anticipate<'a>>,
    availables: &PairSlice<Available<'a>>,
) -> Vec<HashSet<Expr<'a>>> {
    assert_eq!(anticipates.len(), availables.len());
    anticipates
        .iter()
        .zip(availables.iter())
        .map(|((anticipate_in, _), (available_in, _))| &anticipate_in.exprs - &available_in.exprs)
        .collect()
}

#[derive(Clone, PartialEq)]
pub struct Postponable<'a> {
    pub exprs: HashSet<Expr<'a>>,
}

impl<'a> std::fmt::Debug for Postponable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(&self.exprs, f)
    }
}

impl<'a> SemiLattice<'a> for Postponable<'a> {
    fn top(program: &'a Program) -> Self {
        Postponable {
            exprs: program.exprs().collect(),
        }
    }

    fn start(_: &'a Program) -> Self {
        Postponable {
            exprs: HashSet::new(),
        }
    }

    fn meet(&self, other: &Self) -> Self {
        let exprs = &self.exprs & &other.exprs;
        Postponable { exprs }
    }
}

#[derive(Clone)]
struct PostponableT<'a> {
    earliest: HashSet<Expr<'a>>,
    used: HashSet<Expr<'a>>,
}

impl<'a> PostponableT<'a> {
    fn new(block: &'a Block, earliest: &HashSet<Expr<'a>>) -> Self {
        PostponableT {
            earliest: earliest.clone(),
            used: block.exprs().collect(),
        }
    }
}

impl<'a> Transfer<'a> for PostponableT<'a> {
    type Target = Postponable<'a>;
    type Extra = [HashSet<Expr<'a>>];

    fn new(block_id: BlockID, program: &'a Program, extra: &Self::Extra) -> Self {
        let block = program
            .get_block(block_id)
            .expect("PostponableT: block in-bound");
        let earliest = extra
            .get(block_id)
            .expect("PostponableT: earliest set in-bound");
        PostponableT::new(block, earliest)
    }

    fn apply(&self, value: &Self::Target) -> Self::Target {
        let exprs = &(&value.exprs | &self.earliest) - &self.used;
        Postponable { exprs }
    }
}

pub fn postponables<'a>(
    program: &'a Program,
    earliests: &[HashSet<Expr<'a>>],
) -> PairVec<Postponable<'a>> {
    DataFlow::<Postponable<'a>, Forward, PostponableT<'a>, [HashSet<Expr<'a>>]>::run(
        program, earliests,
    )
    .into_pairs()
}

pub fn latests<'a>(
    program: &'a Program,
    earliests: &[HashSet<Expr<'a>>],
    postponable: &PairSlice<Postponable<'a>>,
) -> Vec<HashSet<Expr<'a>>> {
    assert_eq!(earliests.len(), program.len());
    assert_eq!(postponable.len(), program.len());

    let exprs: HashSet<_> = program.exprs().collect();
    program
        .block_range()
        .map(|i| {
            let eop = &earliests[i] | &postponable[i].0.exprs;
            let suc = program
                .successors(i)
                .map(|(s, _)| &earliests[s] | &postponable[s].0.exprs)
                .fold(exprs.clone(), |intersection, s_set| &intersection & &s_set);
            let using: HashSet<_> = program.get_block(i).unwrap().exprs().collect();
            &eop & &(&using | &(&exprs - &suc))
        })
        .collect()
}

#[derive(Clone, PartialEq)]
pub struct Used<'a> {
    pub exprs: HashSet<Expr<'a>>,
}

impl<'a> std::fmt::Debug for Used<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(&self.exprs, f)
    }
}

impl<'a> SemiLattice<'a> for Used<'a> {
    fn top(_: &'a Program) -> Self {
        Used {
            exprs: HashSet::new(),
        }
    }

    fn start(_: &'a Program) -> Self {
        Used {
            exprs: HashSet::new(),
        }
    }

    fn meet(&self, other: &Self) -> Self {
        let exprs = &self.exprs | &other.exprs;
        Used { exprs }
    }
}

#[derive(Clone)]
struct UsedT<'a> {
    gen: HashSet<Expr<'a>>,
    kill: HashSet<Expr<'a>>,
}

impl<'a> UsedT<'a> {
    fn new(block: &'a Block, latest: &HashSet<Expr<'a>>) -> Self {
        let gen = block.exprs().collect();
        let kill = latest.clone();
        UsedT { gen, kill }
    }
}

impl<'a> Transfer<'a> for UsedT<'a> {
    type Target = Used<'a>;
    type Extra = [HashSet<Expr<'a>>];

    fn new(block_id: BlockID, program: &'a Program, latests: &[HashSet<Expr<'a>>]) -> Self {
        let block = program.get_block(block_id).expect("UsedT: block in-bound");
        let latest = latests.get(block_id).expect("UsedT: latest in-bound");
        UsedT::new(block, latest)
    }

    fn apply(&self, out_value: &Self::Target) -> Self::Target {
        let exprs = &(&self.gen | &out_value.exprs) - &self.kill;
        Used { exprs }
    }
}

pub fn used<'a>(program: &'a Program, latests: &'a [HashSet<Expr<'a>>]) -> PairVec<Used<'a>> {
    DataFlow::<Used<'a>, Backward, UsedT<'a>, [HashSet<Expr<'a>>]>::run(program, latests)
        .into_pairs()
}

pub fn where_to_compute<'a>(
    expr: &Expr<'a>,
    latests: &[HashSet<Expr<'a>>],
    used: &PairSlice<Used<'a>>,
) -> HashSet<BlockID> {
    assert_eq!(latests.len(), used.len());

    (0..latests.len())
        .filter(|i| latests[*i].contains(&expr) && used[*i].1.exprs.contains(&expr))
        .collect()
}

pub fn where_to_use<'a>(
    expr: &Expr<'a>,
    program: &'a Program,
    latests: &[HashSet<Expr<'a>>],
    used: &PairSlice<Used<'a>>,
) -> HashSet<BlockID> {
    assert_eq!(latests.len(), program.len());
    assert_eq!(used.len(), program.len());

    program
        .blocks()
        .enumerate()
        .filter_map(|(i, block)| {
            let using: HashSet<_> = block.exprs().collect();
            if using.contains(expr)
                && (!latests[i].contains(expr) || used[i].1.exprs.contains(expr))
            {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
fn figure_9_33() -> Program {
    Program::with_entry_exit(
        vec![
            Block::empty(),
            Block::parse(1, "c = 2"),
            Block::empty(),
            Block::empty(),
            Block::parse(2, "a = b + c"),
            Block::empty(),
            Block::parse(3, "d = b + c"),
            Block::empty(),
            Block::parse(4, "e = b + c"),
            Block::empty(),
            Block::empty(),
        ],
        &[
            (0, 1),
            (1, 2),
            (1, 5),
            (2, 3),
            (3, 4),
            (4, 7),
            (5, 6),
            (6, 7),
            (7, 8),
            (8, 9),
            (8, 11),
            (9, 10),
            (10, 9),
            (10, 11),
            (11, 12),
        ],
    )
}

#[test]
fn cut_edges_test() {
    let program = cut_edges(Program::new(
        vec![
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
            Block::empty(),
        ],
        &[(0, 1), (0, 2), (1, 3), (2, 3), (2, 4)],
    ));

    assert_eq!(program.len(), 7);
    let edges = program.edges();
    assert_eq!(edges.len(), 7);
    assert!(!edges.contains(&(1, 3)));
    assert!(!edges.contains(&(2, 3)));
    let empty = program.successors(1).next().unwrap().0;
    assert!(edges.contains(&(empty, 3)));
}

#[test]
fn lazy_motion_test() {
    use crate::utils::filter_indices;

    let program = figure_9_33();
    let anticipates = anticipates(&program);
    let stmt = Stmt::parse("a = b + c");
    let expr = stmt.as_expr().unwrap();

    let anticipated_blocks: HashSet<BlockID> =
        filter_indices(&anticipates, |(in_value, _)| in_value.exprs.contains(&expr)).collect();

    assert_eq!(
        anticipated_blocks,
        vec![3, 4, 5, 6, 7, 9].into_iter().collect()
    );

    let availables = availables(&program, &anticipates);
    let not_available_blocks: HashSet<BlockID> =
        filter_indices(&availables, |(in_value, _)| !in_value.exprs.contains(&expr)).collect();

    assert_eq!(
        not_available_blocks,
        vec![0, 1, 2, 3, 5].into_iter().collect()
    );

    let earliests = earliests(&anticipates, &availables);
    let earliest_blocks: HashSet<BlockID> =
        filter_indices(&earliests, |earliest| earliest.contains(&expr)).collect();

    assert_eq!(earliest_blocks, vec![3, 5].into_iter().collect());

    let postponables = postponables(&program, &earliests);
    let postponable_blocks: HashSet<BlockID> =
        filter_indices(&postponables, |(in_value, out_value)| {
            // darkly shaded boxes in figure 9.33(b) indicates a postponable code point instead of a postponable block
            // `b + c` is not in postponable[B3].in but in postponable[B3].out
            in_value.exprs.contains(&expr) || out_value.exprs.contains(&expr)
        })
        .collect();

    assert_eq!(postponable_blocks, vec![3, 4].into_iter().collect());

    let latests = latests(&program, &earliests, &postponables);
    let latest_blocks: HashSet<BlockID> =
        filter_indices(&latests, |latest| latest.contains(&expr)).collect();
    assert_eq!(latest_blocks, vec![4, 5].into_iter().collect());

    let used = used(&program, &latests);
    let used_blocks: HashSet<BlockID> =
        filter_indices(&used, |(_, out_value)| out_value.exprs.contains(&expr)).collect();
    assert_eq!(
        &used_blocks & &latest_blocks,
        vec![4, 5].into_iter().collect()
    );
}
