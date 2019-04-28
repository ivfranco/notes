use std::collections::BTreeMap;

type Node = usize;

pub fn critical_path_len(edges: &[(Node, Node)]) -> usize {
    assert!(
        edges.iter().all(|(from, to)| from < to),
        "Error: Input edges are not acyclic"
    );
    assert!(!edges.is_empty());

    let mut graph: BTreeMap<Node, Vec<Node>> = BTreeMap::new();
    for (from, to) in edges {
        graph.entry(*from).or_insert_with(Vec::new).push(*to);
    }

    let mut solutions: BTreeMap<Node, usize> = BTreeMap::new();

    for (node, dests) in graph.iter().rev() {
        let longest_path = dests
            .iter()
            .map(|dest| *solutions.get(dest).unwrap_or(&1))
            .max()
            .unwrap_or(0)
            + 1;

        solutions.insert(*node, longest_path);
    }

    *solutions.values().max().unwrap()
}

#[test]
fn critical_path_test() {
    let edges = &[
        (0, 2),
        (0, 4),
        (0, 8),
        (1, 2),
        (1, 3),
        (1, 4),
        (1, 5),
        (1, 7),
        (1, 8),
        (2, 3),
        (2, 4),
        (2, 5),
        (2, 7),
        (2, 8),
        (3, 4),
        (3, 5),
        (3, 7),
        (3, 8),
        (4, 5),
        (4, 7),
        (4, 8),
        (5, 7),
        (5, 8),
        (7, 8),
    ];

    assert_eq!(critical_path_len(edges), 7);
}
