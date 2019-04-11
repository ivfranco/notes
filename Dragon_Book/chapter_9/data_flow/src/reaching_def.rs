use crate::{BlockID, Program, StmtID};
use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};

fn sorted(set: &HashSet<BlockID>) -> Vec<BlockID> {
    let mut sorted: Vec<_> = set.iter().cloned().collect();
    sorted.sort();
    sorted
}

fn killed(program: &Program, i: StmtID) -> HashSet<usize> {
    if let Some(dst) = program.get_stmt(i).and_then(|stmt| stmt.dst()) {
        program
            .stmts()
            .filter(|(j, stmt)| {
                if let Some(d) = stmt.dst() {
                    d == dst && *j != i
                } else {
                    false
                }
            })
            .map(|(i, _)| i)
            .collect()
    } else {
        HashSet::new()
    }
}

struct GenKill {
    gen: HashSet<StmtID>,
    kill: HashSet<StmtID>,
}

impl GenKill {
    fn new(block_id: BlockID, program: &Program) -> Self {
        let block = program.get_block(block_id).expect("Block inbound");
        let mut gen = HashSet::new();
        let mut kill = HashSet::new();

        for (i, _) in block.stmts() {
            let kill_i = killed(program, i);
            gen.insert(i);
            kill = &kill | &kill_i;
            gen = &gen - &kill;
        }

        GenKill { gen, kill }
    }

    fn transfer(&self, in_def: &HashSet<StmtID>) -> HashSet<StmtID> {
        &self.gen | &(in_def - &self.kill)
    }
}

impl Debug for GenKill {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "gen: {:?}", sorted(&self.gen))?;
        writeln!(f, "kill: {:?}", sorted(&self.kill))
    }
}

pub struct ReachingDef {
    block_id: BlockID,
    in_def: HashSet<StmtID>,
    out_def: HashSet<StmtID>,
    gen_kill: GenKill,
}

impl ReachingDef {
    fn new(block_id: BlockID, program: &Program) -> Self {
        let gen_kill = GenKill::new(block_id, program);

        ReachingDef {
            block_id,
            in_def: HashSet::new(),
            out_def: HashSet::new(),
            gen_kill,
        }
    }

    fn update(&mut self) -> bool {
        let new_out = self.gen_kill.transfer(&self.in_def);
        let changed = self.out_def != new_out;
        self.out_def = new_out;
        changed
    }
}

impl Debug for ReachingDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "attributes of B{}:", self.block_id)?;
        writeln!(f, "IN: {:?}", sorted(&self.in_def))?;
        writeln!(f, "OUT: {:?}", sorted(&self.out_def))?;
        writeln!(f, "{:?}", self.gen_kill)
    }
}

fn meet(i: BlockID, program: &Program, attrs: &mut Vec<ReachingDef>) {
    let new_in = program
        .predecessors(i)
        .map(|(p, _)| &attrs[p].out_def)
        .fold(HashSet::new(), |set, out_p| &set | out_p);

    attrs[i].in_def = new_in;
}

pub fn reaching_definitions(program: &Program) -> Vec<ReachingDef> {
    let mut attrs: Vec<_> = (0..program.len())
        .map(|i| ReachingDef::new(i, program))
        .collect();
    let mut updated = true;

    while updated {
        updated = false;
        for i in 0..program.len() {
            meet(i, program, &mut attrs);
            updated = attrs[i].update() || updated;
        }
    }

    attrs
}

#[cfg(test)]
fn figure_9_13() -> Program {
    use crate::Block;

    let mut program = Program::new();

    for block in vec![
        Block::parse(0, ""), // ENTRY
        Block::parse(
            1,
            "i = m-1
j = n
a = u1",
        ),
        Block::parse(
            4,
            "i = i+1
j = j-1",
        ),
        Block::parse(6, "a = u2"),
        Block::parse(7, "i = u3"),
        Block::parse(7, ""), // EXIT
    ] {
        program.add_block(block);
    }

    for (i, j) in &[(0, 1), (1, 2), (2, 3), (2, 4), (3, 4), (4, 2), (4, 5)] {
        program.add_edge(*i, *j);
    }

    program
}

#[test]
fn reaching_definitions_test() {
    let program = figure_9_13();

    let attrs = reaching_definitions(&program);
    // println!("{:#?}", attrs);
    assert_eq!(attrs[2].gen_kill.kill, vec![1, 2, 7].into_iter().collect());
}
