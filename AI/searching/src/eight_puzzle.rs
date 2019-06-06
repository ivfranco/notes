use crate::{
    local_search::Local,
    utils::{diff, possible_dests},
};

use pathfinding::prelude::astar;
use std::collections::HashSet;

use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};

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

fn manhattan_distance(side: usize, from: usize, to: usize) -> usize {
    let (from_x, from_y) = (from % side, from / side);
    let (to_x, to_y) = (to % side, to / side);

    diff(from_x, to_x) + diff(from_y, to_y)
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
        inversions % 2 == 0
    }

    pub fn successors(&self) -> Vec<Eight> {
        let empty_idx = self.empty_idx();

        possible_dests(empty_idx, SIDE, TILES)
            .into_iter()
            .map(|i| {
                let mut succ = self.tiles;
                succ.swap(i, empty_idx);
                Eight { tiles: succ }
            })
            .collect()
    }

    pub fn heuristic(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if *t == EMPTY {
                    0
                } else {
                    manhattan_distance(SIDE, i, *t as usize - 1)
                }
            })
            .sum()
    }

    pub fn solve(&self) -> Option<(Vec<Self>, usize)> {
        astar(
            self,
            |puzzle| puzzle.successors().into_iter().map(|succ| (succ, 1)),
            Eight::heuristic,
            Eight::is_goal,
        )
    }
}

impl std::fmt::Debug for Eight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for chunk in self.tiles.chunks_exact(3) {
            for tile in chunk {
                if *tile == EMPTY {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", tile)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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

impl Local for Eight {
    fn successors(&self) -> Vec<Self> {
        self.successors()
    }

    fn heuristic(&self) -> f64 {
        self.heuristic() as f64
    }

    fn successful(&self) -> bool {
        self.is_goal()
    }
}

#[test]
fn solvable_test() {
    assert!(Eight::new(GOAL).solvable());
    assert!(!Eight::new([1, 2, 3, 4, 5, 6, 8, 7, EMPTY]).solvable());
}
