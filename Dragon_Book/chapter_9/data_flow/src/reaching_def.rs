use crate::utils::sorted;
use crate::{BlockID, Program, StmtID};
use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};

fn killed(program: &Program, i: StmtID) -> HashSet<usize> {
    if let Some(dst) = program.get_stmt(i).and_then(|stmt| stmt.def()) {
        program
            .stmts()
            .filter(|(j, stmt)| {
                if let Some(d) = stmt.def() {
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

struct ReachingDef {
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
        writeln!(f, "    IN: {:?}", sorted(&self.in_def))?;
        writeln!(f, "    OUT: {:?}", sorted(&self.out_def))?;
        writeln!(f, "    gen: {:?}", sorted(&self.gen_kill.gen))?;
        writeln!(f, "    kill: {:?}", sorted(&self.gen_kill.kill))
    }
}

pub struct ReachingDefs {
    attrs: Vec<ReachingDef>,
}

impl Debug for ReachingDefs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for attr in &self.attrs {
            write!(f, "{:?}", attr)?;
        }
        Ok(())
    }
}

fn meet(i: BlockID, program: &Program, attrs: &[ReachingDef]) -> HashSet<StmtID> {
    program
        .predecessors(i)
        .map(|(p, _)| &attrs[p].out_def)
        .fold(HashSet::new(), |set, out_p| &set | out_p)
}

pub fn reaching_definitions(program: &Program) -> ReachingDefs {
    let mut attrs: Vec<_> = (0..program.len())
        .map(|i| ReachingDef::new(i, program))
        .collect();
    let mut updated = true;

    while updated {
        updated = false;
        for i in 0..program.len() {
            attrs[i].in_def = meet(i, program, &attrs);
            updated = attrs[i].update() || updated;
        }
    }

    ReachingDefs { attrs }
}

#[test]
fn reaching_definitions_test() {
    let program = crate::figure_9_13();

    let rds = reaching_definitions(&program);
    // println!("{:?}", rds);
    assert_eq!(
        rds.attrs[2].gen_kill.kill,
        vec![1, 2, 7].into_iter().collect()
    );
    assert_eq!(rds.attrs[1].out_def, vec![1, 2, 3].into_iter().collect());
    assert_eq!(rds.attrs[2].out_def, vec![3, 4, 5, 6].into_iter().collect());
    assert_eq!(rds.attrs[3].out_def, vec![4, 5, 6].into_iter().collect());
    assert_eq!(rds.attrs[4].out_def, vec![3, 5, 6, 7].into_iter().collect());
}
