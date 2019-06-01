use crate::utils::possible_dests;
use pathfinding::prelude::astar;
use rand::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cleanliness {
    Dirty,
    Clean,
}

use Cleanliness::*;

impl Cleanliness {
    fn clean(&mut self) {
        *self = Clean;
    }

    fn char(self) -> char {
        match self {
            Clean => '.',
            Dirty => '%',
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Room {
    cleaner: usize,
    width: usize,
    squares: Vec<Cleanliness>,
}

impl Room {
    pub fn new(
        (cleaner_x, cleaner_y): (usize, usize),
        width: usize,
        squares: Vec<Cleanliness>,
    ) -> Self {
        assert!(
            cleaner_x < width && cleaner_y < squares.len() / width,
            "Room new: initial position of cleaner out of bound"
        );

        let cleaner = cleaner_y * width + cleaner_x;

        Room {
            cleaner,
            width,
            squares,
        }
    }

    pub fn new_random(side: usize, dirt_ratio: f64) -> Self {
        let mut rng = thread_rng();

        let cleaner_x = rng.gen_range(0, side);
        let cleaner_y = rng.gen_range(0, side);

        let squares = std::iter::from_fn(|| {
            if random::<f64>() < dirt_ratio {
                Some(Dirty)
            } else {
                Some(Clean)
            }
        })
        .take(side * side)
        .collect();

        Room::new((cleaner_x, cleaner_y), side, squares)
    }

    fn clean(&self) -> Self {
        let mut clone = self.clone();
        clone.squares[self.cleaner].clean();
        clone
    }

    pub fn is_goal(&self) -> bool {
        self.squares.iter().all(|square| *square == Clean)
    }

    // each dirty square requires two moves to clean (move to, suck)
    // except the square under the cleaner which can be cleaned in one action
    pub fn heuristic(&self) -> usize {
        let dirty_squares = self
            .squares
            .iter()
            .filter(|square| **square == Dirty)
            .count();

        (dirty_squares * 2).saturating_sub(1)
    }

    pub fn successors(&self) -> Vec<Self> {
        let mut succs = vec![self.clean()];

        succs.extend(
            possible_dests(self.cleaner, self.width, self.squares.len())
                .into_iter()
                .map(|cleaner| Room {
                    cleaner,
                    width: self.width,
                    squares: self.squares.clone(),
                }),
        );

        succs
    }

    pub fn solve(&self) -> (Vec<Self>, usize) {
        astar(
            self,
            |state| state.successors().into_iter().zip(std::iter::repeat(1)),
            Room::heuristic,
            Room::is_goal,
        )
        .unwrap()
    }
}

impl std::fmt::Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for chunk in self.squares.chunks_exact(self.width) {
            let line: String = chunk.iter().cloned().map(Cleanliness::char).collect();
            writeln!(f, "{}", line)?;
        }
        writeln!(
            f,
            "Cleaner: ({}, {})",
            self.cleaner % self.width,
            self.cleaner / self.width
        )
    }
}
