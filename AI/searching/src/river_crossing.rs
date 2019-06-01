use pathfinding::prelude::*;
use std::collections::HashSet;

type Cost = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    This,
    Other,
}

use Side::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RiverSide {
    state: [i32; 4],
    boat: Side,
}

impl RiverSide {
    fn new(state: [i32; 4], boat: Side) -> Self {
        RiverSide { state, boat }
    }
    fn init(missionary: u32, cannibal: u32) -> Self {
        RiverSide::new([missionary as i32, cannibal as i32, 0, 0], This)
    }

    fn missionary_this(&self) -> u32 {
        self.state[0] as u32
    }

    fn cannibal_this(&self) -> u32 {
        self.state[1] as u32
    }

    fn missionary_other(&self) -> u32 {
        self.state[2] as u32
    }

    fn cannibal_other(&self) -> u32 {
        self.state[3] as u32
    }

    fn valid(&self, side: Side) -> bool {
        self.state.iter().all(|n| *n >= 0)
            && if side == This {
                self.missionary_this() >= self.cannibal_this()
            } else {
                self.missionary_other() >= self.cannibal_other()
            }
    }

    fn goal(&self) -> bool {
        self.missionary_this() == 0 && self.cannibal_this() == 0
    }

    fn successors(&self) -> HashSet<Self> {
        let mut successors = HashSet::new();

        if self.boat == This {
            for i in 0..=1 {
                let mut succ = self.state;
                succ[i] -= 1;
                succ[i + 2] += 1;
                successors.insert(RiverSide::new(succ, Other));
            }

            for (i, j) in &[(0, 0), (0, 1), (1, 1)] {
                let mut succ = self.state;
                succ[*i] -= 1;
                succ[*j] -= 1;
                succ[i + 2] += 1;
                succ[j + 2] += 1;
                successors.insert(RiverSide::new(succ, Other));
            }
        } else {
            for i in 2..=3 {
                let mut succ = self.state;
                succ[i] -= 1;
                succ[i - 2] += 1;
                successors.insert(RiverSide::new(succ, This));
            }

            for (i, j) in &[(2, 2), (2, 3), (3, 3)] {
                let mut succ = self.state;
                succ[*i] -= 1;
                succ[*j] -= 1;
                succ[i - 2] += 1;
                succ[j - 2] += 1;
                successors.insert(RiverSide::new(succ, This));
            }
        }

        successors.retain(|succ| succ.valid(self.boat));
        successors
    }

    fn heuristic(&self) -> Cost {
        let this_side = self.missionary_this() + self.cannibal_this();
        if this_side == 0 {
            0
        } else if this_side <= 2 {
            1
        } else {
            this_side - 1
        }
    }
}

impl std::fmt::Debug for RiverSide {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "this side: {}M + {}C, other side: {}M + {}C, boat on: {:?} side",
            self.missionary_this(),
            self.cannibal_this(),
            self.missionary_other(),
            self.cannibal_other(),
            self.boat,
        )
    }
}

pub fn solve_river_crossing(missionary: u32, cannibal: u32) -> Option<Vec<RiverSide>> {
    astar(
        &RiverSide::init(missionary, cannibal),
        |state| {
            state
                .successors()
                .into_iter()
                .map(|successor| (successor, 1))
        },
        RiverSide::heuristic,
        RiverSide::goal,
    )
    .map(|(path, _)| path)
}
