use crate::{Actuator, Agent, Perceptor, Score, World};
use rand::distributions::{Distribution, Standard};
use rand::prelude::*;

use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(PartialEq, Clone, Copy)]
pub enum Square {
    Obstacle,
    Dirty,
    Clean,
}

use Square::*;

impl Square {
    fn char(self) -> char {
        match self {
            Obstacle => '#',
            Dirty => '%',
            Clean => ' ',
        }
    }
}

impl Distribution<Square> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Square {
        let n = rng.gen_range(0, 8);
        if n == 0 {
            Obstacle
        } else if n < 4 {
            Dirty
        } else {
            Clean
        }
    }
}

type Bumped = bool;
type Pos = (isize, isize);

#[derive(Clone)]
pub struct Rectangle {
    side: isize,
    bumped: Bumped,
    squares: HashMap<Pos, Square>,
    bot: Pos,
}

impl Rectangle {
    pub fn new(side: u8) -> Self {
        let side = isize::from(side);
        let mut squares = HashMap::new();
        for x in 0..side {
            for y in 0..side {
                squares.insert((x, y), random());
            }
        }

        let mut rng = thread_rng();
        let bot = (rng.gen_range(0, side), rng.gen_range(0, side));
        squares.insert(bot, Clean);

        Rectangle {
            squares,
            bot,
            side,
            bumped: false,
        }
    }

    fn in_bound(&self, x: isize, y: isize) -> bool {
        match self.squares.get(&(x, y)) {
            None | Some(Obstacle) => false,
            _ => true,
        }
    }

    fn move_to(&mut self, nx: isize, ny: isize) {
        if self.in_bound(nx, ny) {
            self.bot = (nx, ny);
            self.bumped = false;
        } else {
            self.bumped = true;
        }
    }

    fn clean(&mut self) {
        self.squares.insert(self.bot, Clean);
    }

    fn square_under_bot(&self) -> Square {
        *self.squares.get(&self.bot).unwrap()
    }
}

impl World for Rectangle {
    fn measure(&self) -> Score {
        i32::try_from(
            self.squares
                .values()
                .filter(|square| **square == Clean)
                .count(),
        )
        .unwrap_or(i32::max_value())
    }
}

impl std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..self.side {
            let line: String = (0..self.side)
                .map(|x| self.squares.get(&(x, y)).unwrap().char())
                .collect();
            writeln!(f, "{}", line)?;
        }
        writeln!(f, "Bot position: {:?}", self.bot)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use Dir::*;

impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        *[Up, Down, Left, Right].choose(rng).unwrap()
    }
}

impl Dir {
    fn enumerate() -> [Dir; 4] {
        [Up, Down, Left, Right]
    }

    fn walk(self, (x, y): Pos) -> Pos {
        match self {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Move(Dir),
    Clean,
}

impl Actuator<Rectangle> for Action {
    fn apply(&self, world: &mut Rectangle) {
        use Action::*;

        match self {
            Move(dir) => {
                let (nx, ny) = dir.walk(world.bot);
                world.move_to(nx, ny);
            }
            Clean => world.clean(),
        }
    }
}

#[derive(Default)]
pub struct RandomCleaner {}

impl RandomCleaner {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Perceptor<Rectangle> for RandomCleaner {
    type Percept = Square;

    fn observe(&self, world: &Rectangle) -> Self::Percept {
        world.square_under_bot()
    }
}

impl Agent<Rectangle, Square> for RandomCleaner {
    type A = Action;

    fn step(&mut self, percept: Square) -> Self::A {
        if percept == Dirty {
            Action::Clean
        } else {
            Action::Move(random())
        }
    }
}

pub struct BumpCleaner {
    visited: HashMap<Pos, Square>,
    pos: Pos,
    last_action: Action,
}

impl BumpCleaner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        BumpCleaner {
            visited: HashMap::new(),
            pos: (0, 0),
            last_action: Action::Clean,
        }
    }
}

impl BumpCleaner {
    fn update_map(&mut self, bumped: Bumped) {
        self.visited.insert(self.pos, Clean);
        if let Action::Move(dir) = self.last_action {
            if bumped {
                self.visited.insert(dir.walk(self.pos), Obstacle);
            } else {
                self.pos = dir.walk(self.pos);
            }
        }
    }


    fn choose_dir(&self) -> Dir {
        Dir::enumerate()
            .iter()
            .cloned()
            .find(|dir| self.visited.get(&dir.walk(self.pos)).is_none())
            .unwrap_or_else(|| {
                let passable: Vec<_> = Dir::enumerate()
                    .iter()
                    .filter(|dir| self.visited.get(&dir.walk(self.pos)) != Some(&Obstacle))
                    .cloned()
                    .collect();
                *passable.choose(&mut thread_rng()).unwrap()
            })
    }
}


impl Perceptor<Rectangle> for BumpCleaner {
    type Percept = (Bumped, Square);

    fn observe(&self, world: &Rectangle) -> (Bumped, Square) {
        (world.bumped, world.square_under_bot())
    }
}

impl Agent<Rectangle, (Bumped, Square)> for BumpCleaner {
    type A = Action;

    fn step(&mut self, (bumped, square): (Bumped, Square)) -> Self::A {
        self.update_map(bumped);
        let action = match square {
            Dirty => Action::Clean,
            _ => Action::Move(self.choose_dir()),
        };

        self.last_action = action;
        action
    }
}