use crate::{Block, BlockType};
use std::marker::PhantomData;

trait Direction {}

pub enum Forward {}
impl Direction for Forward {}
pub enum Backward {}
impl Direction for Backward {}

pub trait SemiLattice: PartialEq {
    fn top() -> Self;
    fn start() -> Self;
    fn meet(&self, other: &Self) -> Self;
}

pub trait Transfer {
    type Target;
    fn new(block: &Block) -> Self;
    fn apply(&self, v: &Self::Target) -> Self::Target;
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
            AttrType::Entry(v) => Some(v),
            AttrType::Basic(v, _) => Some(v),
            _ => None,
        }
    }

    fn out_value(&self) -> Option<&V> {
        match &self.block {
            AttrType::Basic(_, v) => Some(v),
            AttrType::Exit(v) => Some(v),
            _ => None,
        }
    }
}

impl<V, D, T> Attr<V, D, T>
where
    V: SemiLattice,
    D: Direction,
    T: Transfer<Target = V>,
{
    fn new_basic(transfer: T) -> Self {
        Attr::new_block(AttrType::Basic(V::top(), V::top()), transfer)
    }
}

impl<V, T> Attr<V, Forward, T>
where
    V: SemiLattice,
    T: Transfer<Target = V>,
{
    fn new_entry(transfer: T) -> Self {
        Attr::new_block(AttrType::Entry(V::start()), transfer)
    }

    fn new_exit(transfer: T) -> Self {
        Attr::new_block(AttrType::Exit(V::top()), transfer)
    }

    fn new(block: &Block) -> Self {
        let transfer = T::new(block);
        match block.btype {
            BlockType::Entry => Self::new_entry(transfer),
            BlockType::Basic => Self::new_basic(transfer),
            BlockType::Exit => Self::new_exit(transfer),
        }
    }

    fn update(&mut self) -> bool {
        match &mut self.block {
            AttrType::Basic(in_set, out_set) => {
                let new_out = self.tranfer.apply(in_set);
                let changed = out_set != &new_out;
                *out_set = new_out;
                changed
            }
            _ => false,
        }
    }
}

impl<V, T> Attr<V, Backward, T>
where
    V: SemiLattice,
    T: Transfer<Target = V>,
{
    fn new_entry(transfer: T) -> Self {
        Attr::new_block(AttrType::Entry(V::top()), transfer)
    }

    fn new_exit(transfer: T) -> Self {
        Attr::new_block(AttrType::Exit(V::start()), transfer)
    }

    fn new(block: &Block) -> Self {
        let transfer = T::new(block);
        match block.btype {
            BlockType::Entry => Self::new_entry(transfer),
            BlockType::Basic => Self::new_basic(transfer),
            BlockType::Exit => Self::new_exit(transfer),
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
