#![allow(clippy::many_single_char_names)]
#![allow(non_snake_case)]

use petgraph::{
    prelude::*,
    dot::Dot,
    data::FromElements,
    algo::{min_spanning_tree, bellman_ford},
    visit::IntoNodeReferences,
};
use problems::{print_all_paths, print_dijkstra, dv::DVGraph};

fn main() {
    problem_24();
    problem_25();
    problem_26();
    problem_27();
    problem_28();
    problem_31();
    problem_44();
    problem_48();
    problem_49();
    problem_50();
}

#[allow(clippy::many_single_char_names)]
fn figure_4_27() -> (UnGraph<char, ()>, [NodeIndex; 6]) {
    let mut graph = UnGraph::new_undirected();
    let u = graph.add_node('u');
    let v = graph.add_node('v');
    let w = graph.add_node('w');
    let x = graph.add_node('x');
    let y = graph.add_node('y');
    let z = graph.add_node('z');

    graph.add_edge(u, v, ());
    graph.add_edge(u, x, ());
    graph.add_edge(u, w, ());
    graph.add_edge(v, w, ());
    graph.add_edge(v, x, ());
    graph.add_edge(w, x, ());
    graph.add_edge(w, y, ());
    graph.add_edge(w, z, ());
    graph.add_edge(x, y, ());
    graph.add_edge(y, z, ());

    (graph, [u, v, w, x, y, z])
}

fn problem_24() {
    println!("\nP24");

    let (graph, [u, _, _, _, y, _]) = figure_4_27();
    print_all_paths(&graph, y, u);
}

fn problem_25() {
    println!("\nP25");
    let (graph, [u, _, w, x, _, z]) = figure_4_27();
    println!("x ~> z");
    print_all_paths(&graph, x, z);
    println!("z ~> u");
    print_all_paths(&graph, z, u);
    println!("z ~> w");
    print_all_paths(&graph, z, w);
}

fn figure_p26() -> (UnGraph<char, u32>, [NodeIndex; 7]) {
    let mut graph = UnGraph::new_undirected();
    let t = graph.add_node('t');
    let u = graph.add_node('u');
    let v = graph.add_node('v');
    let w = graph.add_node('w');
    let x = graph.add_node('x');
    let y = graph.add_node('y');
    let z = graph.add_node('z');

    for &(from, to, cost) in &[
        (t, u, 2),
        (t, v, 4),
        (t, y, 7),
        (u, v, 3),
        (u, w, 3),
        (v, w, 4),
        (v, x, 3),
        (v, y, 8),
        (w, x, 6),
        (x, y, 6),
        (x, z, 8),
        (y, z, 12),
    ] {
        graph.add_edge(from, to, cost);
    }

    (graph, [t, u, v, w, x, y, z])
}

fn problem_26() {
    println!("\nP26");

    let (graph, nodes) = figure_p26();
    print_dijkstra(&graph, nodes[4]);
}

fn problem_27() {
    println!("\nP27");

    let (graph, nodes) = figure_p26();
    for &node in &nodes {
        let chr = graph.node_weight(node).unwrap();
        println!("Shortest paths from {}", chr);
        print_dijkstra(&graph, node);
    }
}

fn problem_28() {
    println!("\nP28");

    let mut dv_graph = DVGraph::new(5);
    let u = dv_graph.add_node('u');
    let v = dv_graph.add_node('v'); 
    let x = dv_graph.add_node('x'); 
    let y = dv_graph.add_node('y'); 
    let z = dv_graph.add_node('z');

    dv_graph.add_edge(u, v, 1); 
    dv_graph.add_edge(u, y, 2); 
    dv_graph.add_edge(v, x, 3); 
    dv_graph.add_edge(v, z, 6);
    dv_graph.add_edge(y, x, 3); 
    dv_graph.add_edge(x, z, 2);

    while dv_graph.sync_update() {
        println!("{:?}", dv_graph);
    }
}

fn problem_31() {
    println!("\nP31");

    let mut dv_graph = DVGraph::new(3);
    let x = dv_graph.add_node('x'); 
    let y = dv_graph.add_node('y'); 
    let z = dv_graph.add_node('z'); 

    dv_graph.add_edge(x, y, 3);
    dv_graph.add_edge(x, z, 4);
    dv_graph.add_edge(y, z, 6);

    while dv_graph.sync_update() {
        println!("{:?}", dv_graph);
    }
}

fn problem_44() {
    println!("\nP44");

    let (graph, _) = figure_p26();
    let mst: UnGraph<_, _> = Graph::from_elements(min_spanning_tree(&graph));
    println!("{}", Dot::new(&mst));
}

fn figure_4_46() -> (UnGraph<char, u32>, [NodeIndex; 7]) {
    let mut graph = UnGraph::new_undirected();

    let A = graph.add_node('A');
    let B = graph.add_node('B'); 
    let C = graph.add_node('C'); 
    let D = graph.add_node('D'); 
    let E = graph.add_node('E'); 
    let F = graph.add_node('F'); 
    let G = graph.add_node('G');

    graph.add_edge(A, B, 1); 
    graph.add_edge(A, C, 1); 
    graph.add_edge(B, C, 1); 
    graph.add_edge(B, D, 1); 
    graph.add_edge(C, E, 1); 
    graph.add_edge(C, F, 1); 
    graph.add_edge(D, E, 1); 
    graph.add_edge(D, G, 1); 
    graph.add_edge(E, F, 1); 

    (graph, [A, B, C, D, E, F, G])
}

fn prev_map(graph: &UnGraph<char, u32>, source: NodeIndex) {
    let (_, prevs) = bellman_ford(&graph.map(|_, &n| n, |_, &e| f64::from(e)), source).unwrap();
    for (node, symbol) in graph.node_references() {
        println!("p({}) = {:?}", symbol, prevs[node.index()].and_then(|prev| graph.node_weight(prev)));
    }
}

fn problem_48() {
    println!("\nP48");

    let (graph, nodes) = figure_p26();
    let source = nodes[6];
    prev_map(&graph, source);
}

fn problem_49() {
    println!("\nP49");

    let (graph, nodes) = figure_4_46();
    prev_map(&graph, nodes[2]);
}

fn problem_50() {
    println!("\nP50");

    let (graph, nodes) = figure_p26();
    prev_map(&graph, nodes[2]);
}

