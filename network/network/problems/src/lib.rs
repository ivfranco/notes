use petgraph::{prelude::*, visit::IntoNodeReferences};
use std::collections::HashSet;

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

pub fn print_dijkstra(graph: &UnGraph<char, u32>, source: NodeIndex) {
    let mut costs: Vec<Option<(u32, NodeIndex)>> = vec![None; graph.node_count()];
    let mut known: HashSet<usize> = HashSet::new();

    title_row(graph, source);
    costs[source.index()] = Some((0, source));
    data_row(graph, source, &costs);

    while let Some((idx, cost)) = costs
        .iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            if known.contains(&idx) {
                None
            } else {
                pair.map(|(cost, _)| (idx, cost))
            }
        })
        .min_by_key(|(_, cost)| *cost)
    {
        known.insert(idx);
        let from = NodeIndex::new(idx);
        for edge in graph.edges(from) {
            let to = edge.target();
            let new_cost = cost + edge.weight();
            let (old_cost, prev) = costs[to.index()].get_or_insert((new_cost, from));
            if *old_cost > new_cost {
                *old_cost = new_cost;
                *prev = from;
            }
        }
        data_row(graph, source, &costs);
    }
}

const COLUMN: usize = 12;

fn title_row(graph: &UnGraph<char, u32>, source: NodeIndex) {
    for (node, chr) in graph.node_references() {
        if node != source {
            print!("{1:>0$}D({2}),p({2})", COLUMN - "D(a),p(a)".len(), "", chr);
        }
    }
    println!();
}

fn data_row(graph: &UnGraph<char, u32>, source: NodeIndex, costs: &[Option<(u32, NodeIndex)>]) {
    for (node, _) in graph.node_references() {
        if node == source {
            continue;
        }

        if let Some((cost, prev)) = costs[node.index()] {
            let prev_chr = graph.node_weight(prev).unwrap();
            print!("{1:>0$},{2}", COLUMN - ",a".len(), cost, prev_chr);
        } else {
            print!("{1:>0$}", COLUMN, "∞");
        }
    }
    println!();
}

pub mod dv {
    use super::*;

    type DV = Vec<Option<u32>>;
    type DVSlice<'a> = &'a [Option<u32>];

    pub struct DVGraph {
        nodes: usize,
        graph: UnGraph<(char, DV), u32>,
    }

    impl DVGraph {
        pub fn new(nodes: usize) -> Self {
            Self {
                nodes,
                graph: UnGraph::new_undirected(),
            }
        }

        pub fn add_node(&mut self, symbol: char) -> NodeIndex {
            let idx = self.graph.add_node((symbol, vec![None; self.nodes]));
            let (_, dv) = self.graph.node_weight_mut(idx).unwrap();
            dv[idx.index()] = Some(0);
            idx
        }

        pub fn add_edge(&mut self, source: NodeIndex, dest: NodeIndex, cost: u32) -> EdgeIndex {
            self.graph.add_edge(source, dest, cost)
        }

        // return true is any dv is modified
        pub fn sync_update(&mut self) -> bool {
            let mut new_graph = self.graph.clone();
            let modified = new_graph.node_indices().collect::<Vec<_>>()
                .into_iter()
                .map(|idx| {
                    let (_, self_dv) = new_graph.node_weight_mut(idx).unwrap();
                    self.graph.edges(idx)
                        .fold(false, |modified, edge| {
                            let (_, neighbor_dv) = self.graph.node_weight(edge.target()).unwrap();
                            merge(self_dv, neighbor_dv, *edge.weight()) || modified
                        })
                })
                .fold(false, |total, unit| unit || total);
            
            self.graph = new_graph;
            modified
        }
    }

    const COLUMN: usize = 4;

    impl std::fmt::Debug for DVGraph {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // title row
            write!(f, "{1:>0$}", COLUMN, "")?;
            for node in self.graph.node_indices() {
                write!(f, "{1:>0$}", COLUMN, self.graph[node].0)?;
            }
            writeln!(f)?;

            // data rows
            for (_, (symbol, dv)) in self.graph.node_references() {
                write!(f, "{1:>0$}", COLUMN, symbol)?;
                for cost in dv {
                    if let Some(c) = cost {
                        write!(f, "{1:>0$}", COLUMN, c)?;
                    } else {
                        write!(f, "{1:>0$}", COLUMN, "∞")?;
                    }
                }
                writeln!(f)?;
            }

            Ok(())
        }
    }
    

    // return true if self_dv is modified
    fn merge(self_dv: &mut DV, neighbor_dv: DVSlice, link_cost: u32) -> bool {
        self_dv
            .iter_mut()
            .zip(neighbor_dv)
            .fold(false, |modified, (s, n)| match (&s, n) {
                (Some(s_cost), Some(n_cost)) if *s_cost > n_cost + link_cost => {
                    *s = Some(n_cost + link_cost);
                    true
                }
                (None, Some(n_cost)) => {
                    *s = Some(n_cost + link_cost);
                    true
                }
                _ => modified,
            })
    }
}
