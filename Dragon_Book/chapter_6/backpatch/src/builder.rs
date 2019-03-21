use crate::three_addr::{Fragment, Instr, PatchError};
use std::collections::LinkedList;

#[derive(Default)]
pub struct ProcBuilder {
    instrs: Vec<Instr>,
    next_temp: usize,
    start: usize,
}

impl ProcBuilder {
    pub fn new(start: usize) -> Self {
        ProcBuilder {
            start,
            ..ProcBuilder::default()
        }
    }

    pub fn clear(&mut self) {
        self.instrs.clear();
    }

    pub fn gen(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    pub fn next_instr(&self) -> usize {
        self.instrs.len() + self.start
    }

    pub fn new_temp(&mut self) -> String {
        let var = format!("t{}", self.next_temp);
        self.next_temp += 1;
        var
    }

    pub fn backpatch(&mut self, list: &LinkedList<usize>, label: usize) {
        for i in list {
            let instr = self.instrs.get_mut(*i - self.start).unwrap_or_else(|| {
                panic!("Error: Patching out-of-bound instruction: {}", i);
            });

            match instr.patch(label) {
                Err(PatchError::NoDest) => panic!(
                    "Error: Patching instruction without dest: {:?} at {}",
                    instr, i
                ),
                Err(PatchError::Repatching) => {
                    panic!("Error: Re-patching instruction: {:?} at {}", instr, i)
                }
                _ => (),
            }
        }
    }

    pub fn build_fragment(self) -> Fragment {
        Fragment::new(self.instrs, self.start)
    }
}
