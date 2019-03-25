use petgraph::prelude::*;
use petgraph::visit::{IntoNodeReferences, Walker};
use std::collections::{BTreeSet, HashMap};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy)]
enum Alloc {
    Chunk(usize, usize),
    Null,
}

impl Alloc {
    fn realloc(self, offset: usize) -> Alloc {
        match self {
            Alloc::Chunk(_, size) => Alloc::Chunk(offset, size),
            _ => Alloc::Null,
        }
    }

    fn size(self) -> usize {
        match self {
            Alloc::Chunk(_, size) => size,
            Alloc::Null => 0,
        }
    }
}

#[derive(Clone)]
struct Object {
    name: String,
    alloc: Alloc,
}

impl Object {
    fn realloc(&self, offset: usize) -> Object {
        Object {
            name: self.name.clone(),
            alloc: self.alloc.realloc(offset),
        }
    }

    fn dealloc(&self) -> Object {
        Object {
            name: self.name.to_owned(),
            alloc: Alloc::Null,
        }
    }

    fn size(&self) -> usize {
        self.alloc.size()
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "name: {}, ", self.name)?;
        match self.alloc {
            Alloc::Null => write!(f, "Recycled"),
            Alloc::Chunk(offset, size) => write!(f, "offset: {}, size: {}", offset, size),
        }
    }
}

#[derive(Default, Clone)]
pub struct GC {
    graph: StableGraph<Object, ()>,
    free: usize,
    base: Vec<NodeIndex>,
    node_map: HashMap<String, NodeIndex>,
}

impl GC {
    pub fn new() -> Self {
        GC::default()
    }

    pub fn alloc(&mut self, name: &str, size: usize) -> NodeIndex {
        let alloc = Alloc::Chunk(self.free, size);
        let object = Object {
            name: name.to_owned(),
            alloc,
        };
        self.free += size;
        let v = self.graph.add_node(object);

        self.node_map.insert(name.to_owned(), v);
        v
    }

    pub fn alloc_base(&mut self, name: &str, size: usize) -> NodeIndex {
        let v = self.alloc(name, size);
        self.base.push(v);
        v
    }

    pub fn refer(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, ());
    }

    pub fn deref(&mut self, from: NodeIndex, to: NodeIndex) {
        if let Some(e) = self.graph.find_edge(from, to) {
            self.graph.remove_edge(e);
        }
    }

    pub fn remove(&mut self, node: NodeIndex) {
        self.graph.remove_node(node);
    }

    pub fn node(&self, name: &str) -> NodeIndex {
        self.node_map[name]
    }

    fn mark(&self) -> BTreeSet<NodeIndex> {
        self.base
            .iter()
            .map(|v| {
                Dfs::new(&self.graph, *v)
                    .iter(&self.graph)
                    .collect::<BTreeSet<NodeIndex>>()
            })
            .fold(BTreeSet::new(), |mut acc, mut reachable| {
                acc.append(&mut reachable);
                acc
            })
    }

    fn sweep(&mut self, mut free: usize, reachable: &BTreeSet<NodeIndex>) {
        self.graph = self.graph.map(
            |v, object| {
                if !reachable.contains(&v) {
                    object.dealloc()
                } else {
                    object.clone()
                }
            },
            |_, _| (),
        );

        for v in reachable {
            if let Some(object) = self.graph.node_weight_mut(*v) {
                *object = object.realloc(free);
                free += object.size();
            }
        }

        self.free = free;
    }

    pub fn mark_and_sweep(&mut self, free: usize) {
        self.sweep(free, &self.mark());
    }
}

impl Debug for GC {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (_, object) in self.graph.node_references() {
            writeln!(f, "{:?}", object)?;
        }
        Ok(())
    }
}
