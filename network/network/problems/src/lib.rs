use petgraph::prelude::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

pub fn print_all_paths(graph: &UnGraph<char, ()>, from: NodeIndex, to: NodeIndex) {
    print_all_paths_recur(graph, from, to, &mut HashSet::new(), &mut String::new());
}

fn print_all_paths_recur(
    graph: &UnGraph<char, ()>,
    from: NodeIndex,
    to: NodeIndex,
    visited: &mut HashSet<NodeIndex>,
    path: &mut String,
) {
    visited.insert(from);
    path.push(*graph.node_weight(from).unwrap());

    if from == to {
        println!("{}", path);
    } else {
        let neighbors: Vec<_> = graph
            .neighbors(from)
            .filter(|n| !visited.contains(n))
            .collect();

        for neighbor in neighbors {
            print_all_paths_recur(graph, neighbor, to, visited, path)
        }
    }

    path.pop();
    visited.remove(&from);
}

pub fn print_dijkstra(graph: &UnGraph<char, u32>, from: NodeIndex) {
    let mut costs: Vec<Option<(u32, NodeIndex)>> = vec![None; graph.node_count()];
    let mut known: HashSet<NodeIndex> = HashSet::new();

    costs[from.index()] = Some((0, from));
}
