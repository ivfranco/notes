use crate::{Csp, Diff, Var};
use ordered_float::OrderedFloat;
use petgraph::prelude::*;
use rand::prelude::*;

pub fn map_init(nodes: u32, colors: u32, edges: &[(u32, u32)]) -> (Vec<Var<u32>>, Csp<Diff>) {
    let mut csp: Csp<Diff> = Csp::new();
    for _ in 0 .. nodes {
        csp.add_node(());
    }
    csp.extend_with_edges(edges.iter().map(|(from, to)| (*from, *to, Diff)));

    let domain: Var<u32> = (0..colors).collect();
    (vec![domain; csp.node_count()], csp)
}

type Coord = (f64, f64);
type Segment = (Coord, Coord);

fn distance((x0, y0): Coord, (x1, y1): Coord) -> OrderedFloat<f64> {
    ((x0 - x1).powf(2.0) + (y0 - y1).powf(2.0)).sqrt().into()
}

fn diff((x0, y0): Coord, (x1, y1): Coord) -> Coord {
    (x0 - x1, y0 - y1)
}

fn cross((x0, y0): Coord, (x1, y1): Coord) -> f64 {
    x0 * y1 - x1 * y0
}

fn direction(p0: Coord, p1: Coord, p2: Coord) -> f64 {
    cross(diff(p2, p0), diff(p1, p0))
}

// CLRS 3e 33.1
fn intersect((p0, p1): Segment, (p2, p3): Segment) -> bool {
    let d0 = direction(p2, p3, p0);
    let d1 = direction(p2, p3, p1);
    let d2 = direction(p0, p1, p2);
    let d3 = direction(p0, p1, p3);

    d0 * d1 < 0.0 || d2 * d3 < 0.0
}

fn crosses_no_other<C>(s0: Segment, coordinates: &[Coord], csp: &Csp<C>) -> bool {
    csp.edge_references().all(|e| {
        let p0 = coordinates[e.source().index()];
        let p1 = coordinates[e.target().index()];

        !intersect(s0, (p0, p1))
    })
}

pub fn unit_map_init(n: usize, k: u32) -> (Vec<Var<u32>>, Csp<Diff>) {
    assert!(n > 0 && k > 0);

    let mut rng = thread_rng();
    let mut coordinates: Vec<Coord> = vec![];
    let mut csp = Csp::new();
    for _ in 0..n {
        coordinates.push(random());
        csp.add_node(());
    }

    let mut fail = 0;
    // terminate after 10 failures in a row
    while fail < 10 {
        let i = rng.gen_range(0, n);
        let p0 = coordinates[i];

        let closest = coordinates
            .iter()
            .enumerate()
            .filter(|&(j, &p1)| {
                let n0 = NodeIndex::new(i);
                let n1 = NodeIndex::new(j);

                i != j
                    && !(csp.contains_edge(n0, n1) || csp.contains_edge(n1, n0))
                    && crosses_no_other((p0, p1), &coordinates, &csp)
            })
            .min_by_key(|(_, &p1)| distance(p0, p1))
            .map(|(i, _)| i);

        if let Some(j) = closest {
            csp.add_edge(NodeIndex::new(i), NodeIndex::new(j), Diff);
            fail = 0;
        } else {
            fail += 1;
        }
    }

    (vec![(0..k).collect(); n], csp)
}
