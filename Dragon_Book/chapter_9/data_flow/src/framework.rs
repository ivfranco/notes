use crate::{BlockID, BlockType, Program};
use std::marker::PhantomData;

pub trait Direction {}

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
    // some data flow analysis requires data from outside the program or computed from previous passes
    // to initiate the transfer functions, e.g. the four-pass analysis of partial redundent expressions
    type Extra: ?Sized;

    fn new(block_id: BlockID, program: &'a Program, data: &Self::Extra) -> Self;
    fn apply(&self, value: &Self::Target) -> Self::Target;
}

#[derive(Debug)]
enum AttrType<V> {
    Entry(V),
    Basic(V, V),
    Exit(V),
}

pub struct Attr<V, D, T, E: ?Sized> {
    block: AttrType<V>,
    tranfer: T,
    _direction: PhantomData<D>,
    _extra: PhantomData<E>,
}

impl<V, D, T, E: ?Sized> Attr<V, D, T, E>
where
    V: Clone,
{
    fn new_block(block: AttrType<V>, tranfer: T) -> Self {
        Attr {
            block,
            tranfer,
            _direction: PhantomData,
            _extra: PhantomData,
        }
    }

    pub fn in_value(&self) -> Option<&V> {
        match &self.block {
            AttrType::Basic(v, _) => Some(v),
            AttrType::Exit(v) => Some(v),
            _ => None,
        }
    }

    pub fn in_value_mut(&mut self) -> Option<&mut V> {
        match &mut self.block {
            AttrType::Basic(v, _) => Some(v),
            AttrType::Exit(v) => Some(v),
            _ => None,
        }
    }

    pub fn out_value(&self) -> Option<&V> {
        match &self.block {
            AttrType::Entry(v) => Some(v),
            AttrType::Basic(_, v) => Some(v),
            _ => None,
        }
    }

    pub fn out_value_mut(&mut self) -> Option<&mut V> {
        match &mut self.block {
            AttrType::Entry(v) => Some(v),
            AttrType::Basic(_, v) => Some(v),
            _ => None,
        }
    }

    pub fn into_pair(self) -> (V, V) {
        use AttrType::*;
        match self.block {
            Entry(v) => (v.clone(), v),
            Basic(in_v, out_v) => (in_v, out_v),
            Exit(v) => (v.clone(), v),
        }
    }
}

impl<'a, V, D, T, E: ?Sized> Attr<V, D, T, E>
where
    V: SemiLattice<'a>,
    D: Direction,
    T: Transfer<'a, Target = V, Extra = E>,
{
    fn new_basic(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Basic(V::top(program), V::top(program)), transfer)
    }
}

impl<'a, V, T, E: ?Sized> Attr<V, Forward, T, E>
where
    V: SemiLattice<'a>,
    T: Transfer<'a, Target = V, Extra = E>,
{
    fn new_entry(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Entry(V::start(program)), transfer)
    }

    fn new_exit(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Exit(V::top(program)), transfer)
    }

    fn new(block_id: BlockID, program: &'a Program, extra: &E) -> Self {
        let block = program
            .get_block(block_id)
            .expect("Initialize Attr: Block inbound");
        let transfer = T::new(block_id, program, extra);

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

#[allow(dead_code)]
impl<'a, V, T, E: ?Sized> Attr<V, Backward, T, E>
where
    V: SemiLattice<'a>,
    T: Transfer<'a, Target = V, Extra = E>,
{
    fn new_entry(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Entry(V::top(program)), transfer)
    }

    fn new_exit(program: &'a Program, transfer: T) -> Self {
        Attr::new_block(AttrType::Exit(V::start(program)), transfer)
    }

    fn new(block_id: BlockID, program: &'a Program, extra: &E) -> Self {
        let block = program
            .get_block(block_id)
            .expect("Initialize Attr: Block inbound");
        let transfer = T::new(block_id, program, extra);

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

pub struct Attrs<V, D, T, E: ?Sized> {
    pub attrs: Vec<Attr<V, D, T, E>>,
}

impl<'a, V, D, T, E: ?Sized> Attrs<V, D, T, E>
where
    V: SemiLattice<'a>,
{
    pub fn into_iter(self) -> impl Iterator<Item = (V, V)> {
        self.attrs.into_iter().map(Attr::into_pair)
    }

    pub fn into_pairs(self) -> Vec<(V, V)> {
        self.into_iter().collect()
    }
}

pub struct DataFlow<V, D, T, E: ?Sized = ()> {
    attrs: Vec<Attr<V, D, T, E>>,
}

impl<'a, V, T, E: ?Sized> DataFlow<V, Forward, T, E>
where
    V: SemiLattice<'a>,
    T: Transfer<'a, Target = V, Extra = E>,
{
    fn new(program: &'a Program, extra: &E) -> Self {
        let attrs = program
            .block_range()
            .map(|block_id| Attr::<V, Forward, T, E>::new(block_id, program, extra))
            .collect();

        DataFlow { attrs }
    }

    // previous version inits the fold with the current in_value
    // which may cause inconvenience for certain kind of data flow analysis (e.g. constant propagation)
    // now this method only reuses in_value of the current block when the block have no predecessors / successors
    fn propagate(&self, block_id: BlockID, program: &Program) -> V {
        let meet = program
            .predecessors(block_id)
            .map(|(p, _)| {
                self.attrs[p]
                    .out_value()
                    .expect("EXIT should not have successors")
            })
            .fold(None, |opt: Option<V>, p_val| {
                if let Some(in_val) = opt {
                    Some(in_val.meet(p_val))
                } else {
                    Some(p_val.clone())
                }
            });

        meet.unwrap_or_else(|| {
            self.attrs[block_id]
                .in_value()
                .expect("Propagate: Non-ENTRY block")
                .clone()
        })
    }

    fn compute(mut self, program: &Program) -> Attrs<V, Forward, T, E> {
        let mut updated = true;

        while updated {
            updated = false;
            for block_id in program.block_range().filter(|i| *i != program.entry()) {
                *self.attrs[block_id]
                    .in_value_mut()
                    .expect("Forward compute: Non-ENTRY block") = self.propagate(block_id, program);
                updated = self.attrs[block_id].update() || updated;
            }
        }

        Attrs { attrs: self.attrs }
    }

    pub fn run(program: &'a Program, extra: &E) -> Attrs<V, Forward, T, E> {
        let data_flow = Self::new(program, extra);
        data_flow.compute(program)
    }
}

#[allow(dead_code)]
impl<'a, V, T, E: ?Sized> DataFlow<V, Backward, T, E>
where
    V: SemiLattice<'a>,
    T: Transfer<'a, Target = V, Extra = E>,
{
    fn new(program: &'a Program, extra: &E) -> Self {
        let attrs = program
            .block_range()
            .map(|block_id| Attr::<V, Backward, T, E>::new(block_id, program, extra))
            .collect();

        DataFlow { attrs }
    }

    // see propagate of Forward version
    fn propagate(&self, block_id: BlockID, program: &Program) -> V {
        let meet = program
            .successors(block_id)
            .map(|(s, _)| {
                self.attrs[s]
                    .in_value()
                    .expect("Propagate: ENTRY should not have predecessors")
            })
            .fold(None, |opt: Option<V>, s_val| {
                if let Some(out_val) = opt {
                    Some(out_val.meet(s_val))
                } else {
                    Some(s_val.clone())
                }
            });

        meet.unwrap_or_else(|| {
            self.attrs[block_id]
                .out_value()
                .expect("Propagate: Non-EXIT block")
                .clone()
        })
    }

    fn compute(mut self, program: &Program) -> Attrs<V, Backward, T, E> {
        let mut updated = true;

        while updated {
            updated = false;
            for block_id in program.block_range().filter(|i| Some(*i) != program.exit()) {
                *self.attrs[block_id]
                    .out_value_mut()
                    .expect("Backward compute: Non-EXIT block") = self.propagate(block_id, program);
                updated = self.attrs[block_id].update() || updated;
            }
        }

        Attrs { attrs: self.attrs }
    }

    pub fn run(program: &'a Program, extra: &'a E) -> Attrs<V, Backward, T, E> {
        let data_flow = Self::new(program, extra);
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
        type Extra = ();

        fn new(block_id: BlockID, program: &Program, _: &()) -> Self {
            Self::new(block_id, program)
        }

        fn apply(&self, defs: &Self::Target) -> Self::Target {
            let value = self.transfer(&defs.value);
            Defs { value }
        }
    }

    fn reaching_definitions(program: &Program) -> Attrs<Defs, Forward, GenKill, ()> {
        DataFlow::<Defs, Forward, GenKill, ()>::run(program, &())
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
