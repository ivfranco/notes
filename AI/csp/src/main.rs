fn main() {
    exercise_6_1();
}

use petgraph::prelude::*;

type Map = UnGraph<(), ()>;

fn australia() -> Map {
    let mut graph = UnGraph::new_undirected();

    let wa = graph.add_node(());
    let nt = graph.add_node(());
    let sa = graph.add_node(());
    let q = graph.add_node(());
    let nsw = graph.add_node(());
    let v = graph.add_node(());
    let _t = graph.add_node(());

    graph.add_edge(wa, nt, ());
    graph.add_edge(wa, sa, ());
    graph.add_edge(nt, sa, ());
    graph.add_edge(sa, q, ());
    graph.add_edge(sa, nsw, ());
    graph.add_edge(sa, v, ());
    graph.add_edge(q, nsw, ());
    graph.add_edge(q, v, ());
    
    graph
}

fn valid(solution: &[u32], map: &Map) -> bool {
    map.edge_references().all(|e| {
        let ca = solution.get(e.source().index());
        let ct = solution.get(e.target().index());

        match (ca, ct) {
            (Some(c0), Some(c1)) => c0 != c1,
            _ => true,
        }
    })
}

fn count_solutions(partial: &mut Vec<u32>, colors: u32, map: &Map) -> u32 {
    if partial.len() == map.node_count() {
        if valid(partial.as_slice(), map) {
            1
        } else {
            0
        }
    } else {
        let mut count = 0;
        for color in 0 .. colors {
            partial.push(color);
            count += count_solutions(partial, colors, map);
            partial.pop();
        }
        count
    }
}

fn exercise_6_1() {
    println!("6.1");

    for colors in 2 ..= 4 {
        let solutions = count_solutions(&mut vec![], colors, &australia());
        println!("Solution with {} colors: {}", colors, solutions);
    }
}