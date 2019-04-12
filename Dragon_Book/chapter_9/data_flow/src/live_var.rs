use crate::utils::sorted;
use crate::{Block, BlockID, Program};
use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};

struct UseDef<'a> {
    used: HashSet<&'a str>,
    defd: HashSet<&'a str>,
}

impl<'a> UseDef<'a> {
    fn new(block: &'a Block) -> Self {
        let mut used = HashSet::new();
        let mut defd = HashSet::new();

        // for a statement x = x + y
        // x belongs to useB but not defB as x is used before the first definition of x in B
        // this definition does not confirm to the definition in the textbook
        // but IN[B] = useB | (OUT[B] - defB) will not change
        for (_, stmt) in block.stmts() {
            for var in stmt.uses() {
                if !defd.contains(var) {
                    used.insert(var);
                }
            }

            if let Some(var) = stmt.def() {
                if !used.contains(var) {
                    defd.insert(var);
                }
            }
        }

        UseDef { used, defd }
    }

    fn transfer(&self, out_set: &HashSet<&'a str>) -> HashSet<&'a str> {
        &self.used | &(out_set - &self.defd)
    }
}

struct LiveVariable<'a> {
    block_id: BlockID,
    in_set: HashSet<&'a str>,
    out_set: HashSet<&'a str>,
    use_def: UseDef<'a>,
}

impl<'a> LiveVariable<'a> {
    fn new(block_id: BlockID, program: &'a Program) -> Self {
        LiveVariable {
            block_id,
            in_set: HashSet::new(),
            out_set: HashSet::new(),
            use_def: UseDef::new(program.get_block(block_id).unwrap()),
        }
    }

    fn update(&mut self) -> bool {
        let new_in = self.use_def.transfer(&self.out_set);
        let changed = new_in != self.in_set;
        self.in_set = new_in;
        changed
    }
}

impl<'a> Debug for LiveVariable<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "attributes of B{}:", self.block_id)?;
        writeln!(f, "    IN: {:?}", sorted(&self.in_set))?;
        writeln!(f, "    OUT: {:?}", sorted(&self.in_set))?;
        writeln!(f, "    def: {:?}", sorted(&self.use_def.defd))?;
        writeln!(f, "    use: {:?}", sorted(&self.use_def.used))
    }
}

pub struct LiveVariables<'a> {
    attrs: Vec<LiveVariable<'a>>,
}

impl<'a> Debug for LiveVariables<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for attr in &self.attrs {
            write!(f, "{:?}", attr)?;
        }
        Ok(())
    }
}

fn meet<'a>(
    block_id: BlockID,
    program: &'a Program,
    attrs: &[LiveVariable<'a>],
) -> HashSet<&'a str> {
    program
        .successors(block_id)
        .map(|(s, _)| &attrs[s].in_set)
        .fold(HashSet::new(), |set, in_s| &set | in_s)
}

pub fn live_variables(program: &Program) -> LiveVariables<'_> {
    let mut attrs: Vec<_> = (0..program.len())
        .map(|i| LiveVariable::new(i, program))
        .collect();
    let mut updated = true;

    while updated {
        updated = false;
        for i in 0..program.len() {
            attrs[i].out_set = meet(i, program, &attrs);
            updated = attrs[i].update() || updated;
        }
    }

    LiveVariables { attrs }
}

#[test]
fn live_variables_test() {
    let program = crate::figure_9_13();
    let lvs = live_variables(&program);
    println!("{:?}", lvs);
    assert_eq!(
        lvs.attrs[2].use_def.used,
        vec!["i", "j"].into_iter().collect()
    );
    assert_eq!(lvs.attrs[2].use_def.defd, HashSet::new());
}
