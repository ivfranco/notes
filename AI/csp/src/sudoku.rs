use crate::{Csp, Var, Diff};
use petgraph::prelude::*;

const SIDE: usize = 9;
const VARS: usize = SIDE * SIDE;

pub fn sudoku_init() -> (Vec<Var<u8>>, Csp<Diff>) {
    let mut csp = Csp::new();
    for _ in 0 .. VARS {
        csp.add_node(());
    }

    for r in 0 .. SIDE {
        for ci in 0 .. SIDE {
            for cj in ci + 1 .. SIDE {
                let ni = NodeIndex::new(r * SIDE + ci);
                let nj = NodeIndex::new(r * SIDE + cj);
                csp.update_edge(ni, nj, Diff);

                let ni = NodeIndex::new(ci * SIDE + r);
                let nj = NodeIndex::new(cj * SIDE + r);
                csp.update_edge(ni, nj, Diff);
            }
        }
    }

    for zone in Zones::new(SIDE) {
        for (i, &zi) in zone.iter().enumerate() {
            for &zj in &zone[i + 1 ..] {
                let ni = NodeIndex::new(zi);
                let nj = NodeIndex::new(zj);
                csp.update_edge(ni, nj, Diff);
            }
        }
    }

    let vars = vec![(1 ..= 9).collect(); VARS];
    (vars, csp)
}

struct Zones {
    side: usize,
    top: usize,
    left: usize,
}

impl Zones {
    fn new(side: usize) -> Self {
        let zones = Zones {
            side,
            top: 0,
            left: 0,
        };
        assert_eq!(zones.sqrt().pow(2), side, "Zones::new: side length is not a square number");
        zones
    }

    fn sqrt(&self) -> usize {
        (self.side as f64).sqrt().round() as usize
    }

    fn terminal(&self) -> bool {
        self.top == self.side
    }

    fn zone(&self) -> Vec<usize> {
        let mut indices = vec![];
        for r in self.top .. self.top + self.sqrt() {
            for c in self.left .. self.left + self.sqrt() {
                let i = r * self.side + c;
                indices.push(i);
            }
        }
        indices
    } 
}

impl Iterator for Zones {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.terminal() {
            None
        } else {
            let zone = self.zone();
            self.left = (self.left + self.sqrt()) % self.side;
            if self.left == 0 {
                self.top += self.sqrt();
            }
            Some(zone)
        }
    }
}

#[test]
fn zones_test() {
    assert_eq!(Zones::new(4).collect::<Vec<_>>(), &[
        vec![0, 1, 4, 5],
        vec![2, 3, 6, 7],
        vec![8, 9, 12, 13],
        vec![10, 11, 14, 15],
    ]);
}

#[test]
fn sudoku_test() {
    use crate::backtracking_search;
    use std::collections::HashSet;

    let (vars, csp) = sudoku_init();
    let assignment = backtracking_search(vars, &csp).expect("sudoku test: Inconsistency on empty board");

    let one_to_nine: HashSet<u8> = (1 ..= 9).collect();
    for rc in 0 .. SIDE {
        let row: HashSet<u8> = (0 .. SIDE).map(|i| assignment[rc * SIDE + i]).collect();
        let col: HashSet<u8> = (0 .. SIDE).map(|i| assignment[i * SIDE + rc]).collect();
        assert_eq!(row, one_to_nine);
        assert_eq!(col, one_to_nine);
    }
    for zone in Zones::new(SIDE) {
        let numbers: HashSet<u8> = zone.into_iter().map(|i| assignment[i]).collect();
        assert_eq!(numbers, one_to_nine);
    }
}