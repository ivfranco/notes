use crate::{Actuator, Agent, Perceptor, Score, World};

#[derive(Debug, Clone, Copy)]
enum Pos {
    Left,
    Right,
}

impl Pos {
    fn swap(self) -> Self {
        use Pos::*;

        match self {
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Cleanliness {
    Dirt,
    Clean,
}

impl Cleanliness {
    fn is_clean(self) -> bool {
        self == Cleanliness::Clean
    }

    fn clean(&mut self) {
        *self = Cleanliness::Clean;
    }
}

#[derive(Debug, Clone)]
pub struct TwoSquare {
    left: Cleanliness,
    right: Cleanliness,
}

impl TwoSquare {
    fn new(left: Cleanliness, right: Cleanliness) -> Self {
        TwoSquare { left, right }
    }

    pub fn enumerate() -> Vec<Self> {
        use Cleanliness::*;
        vec![
            TwoSquare::new(Clean, Clean),
            TwoSquare::new(Clean, Dirt),
            TwoSquare::new(Dirt, Clean),
            TwoSquare::new(Dirt, Dirt),
        ]
    }
}

impl World for TwoSquare {
    fn measure(&self) -> Score {
        Score::from(self.left.is_clean()) + Score::from(self.right.is_clean())
    }
}

pub enum Action {
    Move,
    CleanLeft,
    CleanRight,
}

impl Actuator<TwoSquare> for Action {
    fn apply(&self, world: &mut TwoSquare) {
        use Action::*;
        match self {
            CleanLeft => world.left.clean(),
            CleanRight => world.right.clean(),
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct ReflexCleaner {
    pos: Pos,
}

impl ReflexCleaner {
    fn new(pos: Pos) -> Self {
        ReflexCleaner { pos }
    }

    pub fn enumerate() -> Vec<Self> {
        use Pos::*;
        vec![ReflexCleaner::new(Left), ReflexCleaner::new(Right)]
    }
}

impl Perceptor<TwoSquare> for ReflexCleaner {
    type Percept = Cleanliness;

    fn observe(&self, world: &TwoSquare) -> Self::Percept {
        match self.pos {
            Pos::Left => world.left,
            Pos::Right => world.right,
        }
    }
}

impl Agent<TwoSquare, Cleanliness> for ReflexCleaner {
    type A = Action;

    fn step(&mut self, percept: Cleanliness) -> (Self::A, Score) {
        use Action::*;
        use Pos::*;

        if percept == Cleanliness::Clean {
            self.pos = self.pos.swap();
            (Move, 0)
        } else {
            match self.pos {
                Left => (CleanLeft, 0),
                Right => (CleanRight, 0),
            }
        }
    }
}
