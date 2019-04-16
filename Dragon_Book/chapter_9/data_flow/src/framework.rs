#![allow(dead_code)]

use crate::{BlockID, BlockType, Program};
use std::marker::PhantomData;

trait Direction {}

pub enum Forward {}
impl Direction for Forward {}
pub enum Backward {}
impl Direction for Backward {}

pub trait SemiLattice<'a>: PartialEq + Clone + 'a {
    fn top(program: &'a Program) -> Self;
    fn start(program: &'a Program) -> Self;
    fn meet(&self, other: &Self) -> Self;
}

pub trait Transfer<'a>: Clone + 'a {
    type Target;
    fn new(block_id: BlockID, program: &'a Program) -> Self;
    fn apply(&self, value: &Self::Target) -> Self::Target;
}

enum AttrType<V> {
    Entry(V),
    Basic(V, V),
    Exit(V),
}

struct Attr<V, D, T> {
    block: AttrType<V>,
    tranfer: T,
    _direction: PhantomData<D>,
}

impl<V, D, T> Attr<V, D, T> {
    fn new_block(block: AttrType<V>, tranfer: T) -> Self {
        Attr {
            block,
            tranfer,
            _direction: PhantomData,
        }
    }

    fn in_value(&self) -> Option<&V> {
        match &self.block {
            AttrType::Basic(v, _) => Some(v),
            AttrType::Exit(v) => Some(v),
            _ => None,
        }
    }

    fn in_value_mut(&mut self) -> Option<&mut V> {
        match &mut self.block {
            AttrType::Basic(v, _) => Some(v),
            AttrType::Exit(v) => Some(v),
            _ => None,
        }
    }

    fn out_value(&self) -> Option<&V> {
        match &self.block {
            AttrType::Entry(v) => Some(v),
            AttrType::Basic(_, v) => Some(v),
            _ => None,
        }
    }

    fn out_value_mut(&mut self) -> Option<&mut V> {
        match &mut self.block {
            AttrType::Entry(v) => Some(v),
            AttrType::Basic(_, v) => Some(v),
            _ => None,
        }
    }
}

impl<'a, V, D, T> Attr<V, D, T>
where
    V: SemiLattice<'a>,
    D: Direction,
    T: Transfer<'a, Target = V>,
{
    fn new_basic(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Basic(V::top(program), V::top(program)), transfer)
    }
}

impl<'a, V, T> Attr<V, Forward, T>
where
    V: SemiLattice<'a>,
    T: Transfer<'a, Target = V>,
{
    fn new_entry(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Entry(V::start(program)), transfer)
    }

    fn new_exit(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Exit(V::top(program)), transfer)
    }

    fn new(block_id: BlockID, program: &'a Program) -> Self {
        let block = program
            .get_block(block_id)
            .expect("Initialize Attr: Block inbound");
        let transfer = T::new(block_id, program);

        match block.btype {
            BlockType::Entry => Self::new_entry(program, transfer),
            BlockType::Basic => Self::new_basic(program, transfer),
            BlockType::Exit => Self::new_exit(program, transfer),
        }
    }

    fn update(&mut self) -> bool {
        match &mut self.block {
            AttrType::Basic(in_val, out_val) => {
                let new_out = self.tranfer.apply(in_val);
                let changed = out_val != &new_out;
                *out_val = new_out;
                changed
            }
            _ => false,
        }
    }
}

impl<'a, V, T> Attr<V, Backward, T>
where
    V: SemiLattice<'a>,
    T: Transfer<'a, Target = V>,
{
    fn new_entry(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Entry(V::top(program)), transfer)
    }

    fn new_exit(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Exit(V::start(program)), transfer)
    }

    fn new(block_id: BlockID, program: &'a Program) -> Self {
        let block = program
            .get_block(block_id)
            .expect("Initialize Attr: Block inbound");
        let transfer = T::new(block_id, program);

        match block.btype {
            BlockType::Entry => Self::new_entry(program, transfer),
            BlockType::Basic => Self::new_basic(program, transfer),
            BlockType::Exit => Self::new_exit(program, transfer),
        }
    }

    fn update(&mut self) -> bool {
        match &mut self.block {
            AttrType::Basic(in_val, out_val) => {
                let new_in = self.tranfer.apply(out_val);
                let changed = in_val != &new_in;
                *in_val = new_in;
                changed
            }
            _ => false,
        }
    }
}

pub struct Attrs<V, D, T> {
    attrs: Vec<Attr<V, D, T>>,
}

pub struct DataFlow<V, D, T> {
    attrs: Vec<Attr<V, D, T>>,
}

impl<'a, V, T> DataFlow<V, Forward, T>
where
    V: SemiLattice<'a>,
    T: Transfer<'a, Target = V>,
{
    fn new(program: &'a Program) -> Self {
        let attrs = program
            .block_indices()
            .map(|block_id| Attr::<V, Forward, T>::new(block_id, program))
            .collect();

        DataFlow { attrs }
    }

    fn meet(&self, block_id: BlockID, program: &Program) -> V {
        let init = self.attrs[block_id]
            .in_value()
            .expect("Non-ENTRY block")
            .clone();

        program
            .predecessors(block_id)
            .map(|(p, _)| {
                self.attrs[p]
                    .out_value()
                    .expect("EXIT should not have successors")
            })
            .fold(init, |in_val, p_val| in_val.meet(p_val))
    }

    fn compute(mut self, program: &Program) -> Attrs<V, Forward, T> {
        let mut updated = true;

        while updated {
            updated = false;
            for block_id in program.block_indices().filter(|i| *i != program.entry()) {
                *self.attrs[block_id].in_value_mut().unwrap() = self.meet(block_id, program);
                updated = self.attrs[block_id].update() || updated;
            }
        }

        Attrs { attrs: self.attrs }
    }

    pub fn run(program: &'a Program) -> Attrs<V, Forward, T> {
        let data_flow = Self::new(program);
        data_flow.compute(program)
    }
}

impl<'a, V, T> DataFlow<V, Backward, T>
where
    V: SemiLattice<'a>,
    T: Transfer<'a, Target = V>,
{
    fn new(program: &'a Program) -> Self {
        let attrs = program
            .block_indices()
            .map(|block_id| Attr::<V, Backward, T>::new(block_id, program))
            .collect();

        DataFlow { attrs }
    }

    fn meet(&self, block_id: BlockID, program: &Program) -> V {
        let init = self.attrs[block_id]
            .out_value()
            .expect("Non-EXIT block")
            .clone();

        program
            .successors(block_id)
            .map(|(p, _)| {
                self.attrs[p]
                    .in_value()
                    .expect("ENTRY should not have predecessors")
            })
            .fold(init, |out_val, s_val| out_val.meet(s_val))
    }

    fn compute(mut self, program: &Program) -> Attrs<V, Backward, T> {
        let mut updated = true;

        while updated {
            updated = false;
            for block_id in program.block_indices().filter(|i| *i != program.exit()) {
                *self.attrs[block_id].in_value_mut().unwrap() = self.meet(block_id, program);
                updated = self.attrs[block_id].update() || updated;
            }
        }

        Attrs { attrs: self.attrs }
    }

    fn run(program: &'a Program) -> Attrs<V, Backward, T> {
        let data_flow = Self::new(program);
        data_flow.compute(program)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::reaching_def::GenKill;
    use crate::StmtID;
    use std::collections::HashSet;

    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct Defs {
        value: HashSet<StmtID>,
    }

    impl<'a> SemiLattice<'a> for Defs {
        fn top(_: &Program) -> Self {
            Self::default()
        }

        fn start(_: &Program) -> Self {
            Self::default()
        }

        fn meet(&self, other: &Self) -> Self {
            let value = &self.value | &other.value;
            Defs { value }
        }
    }

    impl<'a> Transfer<'a> for GenKill {
        type Target = Defs;

        fn new(block_id: BlockID, program: &Program) -> Self {
            Self::new(block_id, program)
        }

        fn apply(&self, defs: &Self::Target) -> Self::Target {
            let value = self.transfer(&defs.value);
            Defs { value }
        }
    }

    fn reaching_definitions(program: &Program) -> Attrs<Defs, Forward, GenKill> {
        DataFlow::<Defs, Forward, GenKill>::run(program)
    }

    #[test]
    fn reaching_definitions_test() {
        let program = crate::figure_9_13();
        let attrs = reaching_definitions(&program).attrs;

        assert_eq!(
            attrs[1].out_value().unwrap().value,
            vec![1, 2, 3].into_iter().collect()
        );
        assert_eq!(
            attrs[2].out_value().unwrap().value,
            vec![3, 4, 5, 6].into_iter().collect()
        );
        assert_eq!(
            attrs[3].out_value().unwrap().value,
            vec![4, 5, 6].into_iter().collect()
        );
        assert_eq!(
            attrs[4].out_value().unwrap().value,
            vec![3, 5, 6, 7].into_iter().collect()
        );
    }
}
