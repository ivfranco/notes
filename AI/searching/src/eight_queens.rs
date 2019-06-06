use crate::local_search::Local;
use crate::utils::diff;
use std::hash::{Hash, Hasher};

use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};

type Queen = usize;
const N_QUEENS: usize = 8;

#[derive(Clone)]
pub struct Queens {
    columns: [Queen; N_QUEENS],
}

impl PartialEq for Queens {
    fn eq(&self, other: &Self) -> bool {
        self.columns == other.columns
    }
}

impl Eq for Queens {}

impl Hash for Queens {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.columns.hash(state);
    }
}

impl Queens {
    pub fn new(columns: [Queen; N_QUEENS]) -> Self {
        assert!(columns.iter().all(|queen| *queen < N_QUEENS));
        Queens { columns }
    }

    /// unordered pairs of queens attacking each other
    pub fn heuristic(&self) -> usize {
        let mut attacks = 0;

        for (x0, &y0) in self.columns.iter().enumerate() {
            for (x1, &y1) in self.columns.iter().enumerate().skip(x0 + 1) {
                if y0 == y1 || diff(x0, x1) == diff(y0, y1) {
                    attacks += 1;
                }
            }
        }

        attacks
    }

    pub fn is_goal(&self) -> bool {
        self.heuristic() == 0
    }

    pub fn successors(&self) -> Vec<Self> {
        let mut succs = vec![];

        for i in 0..N_QUEENS {
            for j in 0..N_QUEENS {
                if self.columns[i] != j {
                    let mut succ = self.columns;
                    succ[i] = j;
                    succs.push(Queens::new(succ));
                }
            }
        }

        succs
    }

    pub fn solve(&self) -> Self {
        use pathfinding::prelude::bfs;

        let path = bfs(self, Queens::successors, Queens::is_goal).unwrap();

        path.last().unwrap().clone()
    }
}

impl Local for Queens {
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

impl Distribution<Queens> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Queens {
        let mut columns = [0; N_QUEENS];

        for col in columns.iter_mut() {
            *col = rng.gen_range(0, N_QUEENS);
        }

        Queens { columns }
    }
}
