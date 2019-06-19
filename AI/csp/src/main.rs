use petgraph::prelude::*;

use std::{
    time::{Duration, Instant},
    thread,
    sync::mpsc::{self, RecvTimeoutError},
    io::Write,
};

use csp::{
    backtracking_search, ac3_total,
    colors::{map_init, unit_map_init},
};

fn main() {
    exercise_6_1();
    exercise_6_8();
    // runs for minutes
    // exercise_6_10();
    exercise_6_11();
}

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
    graph.add_edge(nt, q, ());
    graph.add_edge(q, nsw, ());
    graph.add_edge(nsw, v, ());
    graph.add_edge(sa, wa, ());
    graph.add_edge(sa, nt, ());
    graph.add_edge(sa, q, ());
    graph.add_edge(sa, nsw, ());
    graph.add_edge(sa, v, ());

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
        for color in 0..colors {
            partial.push(color);
            count += count_solutions(partial, colors, map);
            partial.pop();
        }
        count
    }
}

fn exercise_6_1() {
    println!("6.1");

    for colors in 2..=4 {
        let solutions = count_solutions(&mut vec![], colors, &australia());
        println!("Solution with {} colors: {}", colors, solutions);
    }
}

fn exercise_6_8() {
    println!("6.8");

    let (a1, a2, a3, a4, h, t, f1, f2) = (0, 1, 2, 3, 4, 5, 6, 7);

    let (vars, csp) = map_init(
        8,
        3,
        &[
            (a1, a2),
            (a2, a3),
            (a3, a4),
            (a1, h),
            (a2, h),
            (a3, h),
            (a4, h),
            (h, t),
            (t, f1),
            (t, f2),
        ],
    );

    let assignment = backtracking_search(vars, &csp).unwrap();
    println!("{:?}", assignment);
}

fn elapsed<F>(mut f: F) -> Duration 
where
    F: FnMut()
{
    let now = Instant::now();
    f();
    now.elapsed()
}

#[derive(Clone)]
struct Timeout;

fn average_elapsed<F>(n: u32, mut f: F) -> Duration
where
    F: FnMut(),
{
    let mut sum = Duration::default();
    for _ in 0 .. n {
        sum += elapsed(&mut f);
    }
    sum / n
}

fn or_timeout<F, R>(timeout: Duration, mut f: F) -> Result<R, RecvTimeoutError> 
where
    F: FnMut() -> R + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(f()).unwrap();
    });

    rx.recv_timeout(timeout)
}

#[allow(dead_code)]
fn exercise_6_10() {
    println!("6.10");

    const TIMEOUT: Duration = Duration::from_secs(5);
    const SAMPLE: u32 = 100; 

    println!("Sample size = {}", SAMPLE);

    for n in (10..).step_by(10) {
        for k in 3 ..= 4 {
            let recv_result = or_timeout(TIMEOUT, move || average_elapsed(SAMPLE, || {
                let (vars, csp) = unit_map_init(n, k);
                let is_ok = backtracking_search(vars, &csp).is_ok();
                // not sure if this prevents the code from being optimized away
                let _ = write!(&mut std::io::sink(), "{}", is_ok);
            }));

            if let Ok(duration) = recv_result {
                println!("n = {}, k = {}, average time: {:?}", n, k, duration);
            } else {
                println!("n = {}, k = {}, timeout after {:?}", n, k, TIMEOUT);
                return;
            }
        }
    }    
}

fn exercise_6_11() {
    println!("6.11");

    const RED: u32 = 0;
    const GREEN: u32 = 1;

    let (wa, nt, sa, q, nsw, v, _t) = (0, 1, 2, 3, 4, 5, 6);
    let (mut vars, csp) = map_init(7, 3, &[
        (wa, nt),
        (nt, q),
        (q, nsw),
        (nsw, v),
        (sa, wa),
        (sa, nt),
        (sa, q),
        (sa, nsw),
        (sa, v),
    ]);
    vars[wa as usize] = Some(GREEN).into_iter().collect();
    vars[v as usize] = Some(RED).into_iter().collect();

    assert!(ac3_total(&mut vars, &csp).is_err());
}