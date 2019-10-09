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
            print!("{1:>0$}D({2}),p({2})", COLUMN - 9, "", chr);
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
            print!("{1:>0$},{2}", COLUMN - 2, cost, prev_chr);
        } else {
            print!("{1:>0$}", COLUMN, "∞");
        }
    }
    println!();
}
