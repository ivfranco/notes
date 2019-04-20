use crate::framework::{Backward, DataFlow, Forward, SemiLattice, Transfer};
use crate::{Block, BlockID, Expr, Program, Stmt};
use std::collections::HashSet;

type TupleVec<T> = Vec<(T, T)>;
type TupleSlice<T> = [(T, T)];

fn cut_edges(mut program: Program) -> Program {
    for (from, to) in program.edges() {
        if program.predecessors(to).count() > 1 {
            program.insert_empty_between(from, to);
        }
    }

    program
}

#[derive(Clone, PartialEq, Default)]
struct Anticipate<'a> {
    exprs: HashSet<Expr<'a>>,
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

fn anticipates(program: &Program) -> TupleVec<Anticipate<'_>> {
    DataFlow::<Anticipate<'_>, Backward, AnticipateT<'_>>::run(program, &()).into_pairs()
}

#[derive(Clone, PartialEq, Default)]
struct Available<'a> {
    exprs: HashSet<Expr<'a>>,
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

fn availables<'a>(
    program: &'a Program,
    anticipates: &'a TupleSlice<Anticipate<'a>>,
) -> TupleVec<Available<'a>> {
    let anticipate_ins: Vec<_> = anticipates.iter().map(|(in_value, _)| in_value).collect();
    DataFlow::<Available<'a>, Forward, AvailableT<'a>, [&'a Anticipate<'a>]>::run(
        program,
        &anticipate_ins,
    )
    .into_pairs()
}

fn earliests<'a>(
    anticipates: &TupleSlice<Anticipate<'a>>,
    availables: &TupleSlice<Available<'a>>,
) -> Vec<HashSet<Expr<'a>>> {
    assert_eq!(anticipates.len(), availables.len());
    anticipates
        .iter()
        .zip(availables.iter())
        .map(|((anticipate_in, _), (available_in, _))| &anticipate_in.exprs - &available_in.exprs)
        .collect()
}

#[derive(Clone, PartialEq)]
struct Postponable<'a> {
    exprs: HashSet<Expr<'a>>,
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

fn postponables<'a>(
    program: &'a Program,
    earliests: &[HashSet<Expr<'a>>],
) -> TupleVec<Postponable<'a>> {
    DataFlow::<Postponable<'a>, Forward, PostponableT<'a>, [HashSet<Expr<'a>>]>::run(
        program, earliests,
    )
    .into_pairs()
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
    use crate::{BinOp, RValue};

    let program = figure_9_33();
    let anticipates = anticipates(&program);

    let b = RValue::Var("b".into());
    let c = RValue::Var("c".into());
    let expr = Expr {
        lhs: &b,
        op: BinOp::Add,
        rhs: &c,
    };
    let anticipated_blocks: HashSet<BlockID> = anticipates
        .iter()
        .enumerate()
        .filter_map(|(i, (in_value, _))| {
            if in_value.exprs.contains(&expr) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(
        anticipated_blocks,
        vec![3, 4, 5, 6, 7, 9].into_iter().collect()
    );

    let availables = availables(&program, &anticipates);
    let not_available_blocks: HashSet<BlockID> = availables
        .iter()
        .enumerate()
        .filter_map(|(i, (in_value, _))| {
            if !in_value.exprs.contains(&expr) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(
        not_available_blocks,
        vec![0, 1, 2, 3, 5].into_iter().collect()
    );

    let earliests = earliests(&anticipates, &availables);
    let earliest_blocks: HashSet<BlockID> = earliests
        .iter()
        .enumerate()
        .filter_map(|(i, earliest)| {
            if earliest.contains(&expr) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(earliest_blocks, vec![3, 5].into_iter().collect());

    let postponables = postponables(&program, &earliests);
    let postponable_blocks: HashSet<BlockID> = postponables
        .iter()
        .enumerate()
        .filter_map(|(i, (in_value, out_value))| {
            if in_value.exprs.contains(&expr) || out_value.exprs.contains(&expr) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(postponable_blocks, vec![3, 4].into_iter().collect());
}
