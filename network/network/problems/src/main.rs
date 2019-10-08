use petgraph::prelude::*;
use problems::print_all_paths;

fn main() {
    problem_24();
    problem_25();
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

#[allow(clippy::many_single_char_names)]
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
