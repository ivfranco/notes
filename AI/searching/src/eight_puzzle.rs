use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use std::collections::HashSet;

type Tile = u32;
const EMPTY: Tile = 0;
const SIDE: usize = 3;
const TILES: usize = SIDE * SIDE;
const GOAL: [Tile; TILES] = [1, 2, 3, 4, 5, 6, 7, 8, EMPTY];

fn inversions(tiles: &[Tile]) -> usize {
    tiles
        .iter()
        .enumerate()
        .filter(|(i, t)| *i > 0 && **t != EMPTY)
        .map(|(i, t)| {
            (&tiles[..i])
                .iter()
                .filter(|s| **s != EMPTY && *s > t)
                .count()
        })
        .sum()
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Eight {
    tiles: [Tile; TILES],
}

impl Eight {
    pub fn new(tiles: [Tile; TILES]) -> Self {
        assert_eq!(
            tiles.iter().cloned().collect::<HashSet::<_>>(),
            GOAL.iter().cloned().collect::<HashSet<_>>()
        );
        Eight { tiles }
    }

    pub fn is_goal(&self) -> bool {
        self.tiles == GOAL
    }

    fn empty_idx(&self) -> usize {
        self.tiles.iter().position(|t| *t == EMPTY).unwrap()
    }

    pub fn solvable(&self) -> bool {
        let inversions = inversions(&self.tiles);
        dbg!(inversions);
        let empty_row = self.empty_idx() / SIDE;

        (inversions + empty_row) % 2 == 0
    }

    pub fn successors(&self) -> Vec<Eight> {
        let empty_idx = self.empty_idx();
        let mut swappable: Vec<usize> = vec![];

        // swap empty with a tile above
        if empty_idx >= SIDE {
            swappable.push(empty_idx - SIDE);
        }

        // swap empty with a left tile
        if empty_idx % SIDE != 0 {
            swappable.push(empty_idx - 1);
        }

        // swap empty with a right tile
        if empty_idx % SIDE != SIDE - 1 {
            swappable.push(empty_idx + 1);
        }

        // swap empty with a tile below
        if TILES - empty_idx >= SIDE {
            swappable.push(empty_idx + SIDE);
        }

        swappable
            .into_iter()
            .map(|i| {
                let mut succ = self.tiles;
                succ.swap(i, empty_idx);
                Eight { tiles: succ }
            })
            .collect()
    }
}

impl Distribution<Eight> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Eight {
        loop {
            let mut tiles = GOAL;
            tiles.shuffle(rng);
            let eight = Eight { tiles };
            if eight.solvable() {
                break eight;
            }
        }
    }
}

#[test]
fn solvable_test() {
    assert!(Eight::new(GOAL).solvable());
    assert!(!Eight::new([1, 2, 3, 4, 5, 6, 8, 7, EMPTY]).solvable());
}