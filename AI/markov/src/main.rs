use bitflags::bitflags;
use markov::{normalize, HMMContext, Observation, Prob, State, HMM};
use rand::prelude::*;

fn main() {
    exercise_15_7();
    exercise_15_12();
    exercise_15_14();
}

pub mod vaccum_world {
    use super::*;

    #[derive(Clone, Copy, PartialEq)]
    enum Square {
        Empty,
        Block,
    }

    use Square::*;

    pub type Pos = (isize, isize);

    pub fn to_pos(idx: State, width: usize) -> Pos {
        ((idx / width) as isize, (idx % width) as isize)
    }

    pub fn from_pos((y, x): Pos, width: usize) -> State {
        (y as usize) * width + x as usize
    }

    bitflags! {
        struct Sensor: u8 {
            const N = 0b0001;
            const S = 0b0010;
            const W = 0b0100;
            const E = 0b1000;
        }
    }

    impl Sensor {
        const DIRS: u32 = 4;

        fn neighbors(self, (y, x): Pos) -> Vec<Pos> {
            [
                (-1, 0, Sensor::N),
                (1, 0, Sensor::S),
                (0, -1, Sensor::W),
                (0, 1, Sensor::E),
            ]
            .iter()
            .filter_map(|&(dy, dx, s)| {
                if self.contains(s) {
                    Some((y + dy, x + dx))
                } else {
                    None
                }
            })
            .collect()
        }

        fn diff(self, other: Self) -> u32 {
            (self ^ other).bits().count_ones()
        }

        fn observation(self) -> Observation {
            self.bits() as Observation
        }

        fn random_walk(self, (y, x): Pos, trans: [Prob; 4]) -> Pos {
            let mut rng = thread_rng();

            let pairs: Vec<_> = [
                (-1, 0, Sensor::N),
                (1, 0, Sensor::S),
                (0, -1, Sensor::W),
                (0, 1, Sensor::E),
            ]
            .iter()
            .zip(trans.iter())
            .filter_map(|(&(dy, dx, d), &p)| {
                if self.contains(d) {
                    Some(((y + dy, x + dx), p))
                } else {
                    None
                }
            })
            .collect();

            if pairs.iter().map(|(_, p)| *p).sum::<f64>() == 0.0 {
                // when the agent is stuck (e.g. (0, 15) in DEFAULT_MAP), naturally it will stay there forever
                // this corner case must be treated but is not described in the text of AIMA
                (y, x)
            } else {
                pairs.choose_weighted(&mut rng, |(_, p)| *p).unwrap().0
            }
        }
    }

    pub struct Map {
        width: usize,
        squares: Vec<Square>,
    }

    impl Map {
        fn new(width: usize, squares: Vec<Square>) -> Self {
            assert_eq!(squares.len() % width, 0, "Map::new: map is not a square");
            Map { width, squares }
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.states() / self.width()
        }

        fn in_bound(&self, (y, x): Pos) -> bool {
            0 <= y && y < self.height() as isize && 0 <= x && x < self.width() as isize
        }

        pub fn is_empty(&self, pos: Pos) -> bool {
            self.in_bound(pos) && self.squares[from_pos(pos, self.width())] == Empty
        }

        fn sensor_at(&self, idx: State) -> Sensor {
            if self.squares[idx] == Block {
                return Sensor::empty();
            }

            let (y, x) = to_pos(idx, self.width());
            [
                (-1, 0, Sensor::N),
                (1, 0, Sensor::S),
                (0, -1, Sensor::W),
                (0, 1, Sensor::E),
            ]
            .iter()
            .filter_map(|&(dy, dx, s)| {
                if self.is_empty((y + dy, x + dx)) {
                    Some(s)
                } else {
                    None
                }
            })
            .fold(Sensor::empty(), |sensor, s| sensor | s)
        }

        fn sensor_at_pos(&self, pos: Pos) -> Sensor {
            self.sensor_at(from_pos(pos, self.width()))
        }

        fn neighbors(&self, idx: State) -> Vec<State> {
            self.sensor_at(idx)
                .neighbors(to_pos(idx, self.width()))
                .into_iter()
                .map(|pos| from_pos(pos, self.width()))
                .collect()
        }

        fn states(&self) -> usize {
            self.squares.len()
        }

        pub fn to_hmm(&self, error: Prob) -> HMM {
            let mut trans = Vec::with_capacity(self.states().pow(2));
            for (idx, &square) in self.squares.iter().enumerate() {
                let mut trans_row = vec![0.0; self.states()];
                if square == Empty {
                    let neighbors = self.neighbors(idx);
                    for &neighbor in &neighbors {
                        trans_row[neighbor] = 1.0 / neighbors.len() as f64;
                    }
                }
                trans.extend(trans_row);
            }

            let mut sensor_model = Vec::with_capacity(self.states() * 2usize.pow(Sensor::DIRS));
            for sensor in 0..2usize.pow(Sensor::DIRS) {
                let noisy = Sensor::from_bits_truncate(sensor as u8);
                sensor_model.extend((0..self.states()).map(|state| {
                    let perfect = self.sensor_at(state);
                    let diff = noisy.diff(perfect) as i32;
                    (1.0 - error).powi(Sensor::DIRS as i32 - diff) * error.powi(diff)
                }));
            }

            HMM::new(trans, sensor_model)
        }

        pub fn uniform_prior(&self) -> Vec<Prob> {
            let mut prior: Vec<_> = self
                .squares
                .iter()
                .map(|&b| if b == Empty { 1.0 } else { 0.0 })
                .collect();

            normalize(&mut prior);
            prior
        }
    }

    /// Figure 15.7, p582, AIMA e3
    #[rustfmt::skip]
    const DEFAULT_MAP: [[u8; 16]; 4] = [
        [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1],
        [0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0],
        [0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1],
        [1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    ];

    impl Default for Map {
        fn default() -> Self {
            let squares = DEFAULT_MAP
                .iter()
                .flatten()
                .map(|&b| if b == 1 { Empty } else { Block })
                .collect();

            let width = DEFAULT_MAP[0].len();

            Map::new(width, squares)
        }
    }

    pub struct VaccumWorld<'a> {
        map: &'a Map,
        cleaner: Pos,
        error: Prob,
        trans: [Prob; 4],
    }

    impl<'a> VaccumWorld<'a> {
        /// trans: the probability of taking a direction if not blocked, in order of [N, S, W, E]
        pub fn new(map: &'a Map, cleaner: Pos, error: Prob, trans: [Prob; 4]) -> Self {
            assert!(map.is_empty(cleaner));
            assert!((trans.iter().sum::<f64>() - 1.0).abs() <= 0.001);

            VaccumWorld {
                map,
                cleaner,
                error,
                trans,
            }
        }

        pub fn cleaner(&self) -> Pos {
            self.cleaner
        }

        /// wander around with the given transition model
        pub fn wander(&mut self) {
            let sensor = self.map.sensor_at_pos(self.cleaner);
            self.cleaner = sensor.random_walk(self.cleaner, self.trans);
        }

        pub fn observe(&self) -> Observation {
            let mut sensor = self.map.sensor_at_pos(self.cleaner);
            // toggle sensor result at each direction with p = error
            for &d in [Sensor::N, Sensor::S, Sensor::W, Sensor::E].iter() {
                if random::<Prob>() <= self.error {
                    sensor.toggle(d);
                }
            }
            sensor.observation()
        }
    }

    #[test]
    fn wander_test() {
        let map = Map::default();
        let mut world = VaccumWorld::new(&map, (0, 0), 0.1, [0.25; 4]);
        world.wander();

        assert_eq!(world.cleaner, (0, 1));
    }
}

use vaccum_world::*;

fn manhattan((y0, x0): Pos, (y1, x1): Pos) -> isize {
    (y1 - y0).abs() + (x1 - x0).abs()
}

fn exercise_15_7() {
    println!("15.7");

    let mut rng = thread_rng();

    let map = Map::default();
    for &error in &[0.01, 0.05, 0.1, 0.2] {
        let hmm = map.to_hmm(error);
        let mut context = HMMContext::new(&hmm, map.uniform_prior());
        // a uniform random square in NW quadrant
        let cleaner = loop {
            let y = rng.gen_range(0, map.height() as isize / 2);
            let x = rng.gen_range(0, map.width() as isize / 2);
            if map.is_empty((y, x)) {
                break (y, x);
            }
        };
        // a transition model in which the cleaner is more likely to go south east
        let trans = [0.2, 0.3, 0.2, 0.3];
        let mut world = VaccumWorld::new(&map, cleaner, error, trans);

        const T: usize = 100;

        for _ in 0..T {
            world.wander();
            context.observe(world.observe());
        }

        let filtered = context.filter(T).unwrap();
        let weighted_estimate_error = filtered
            .iter()
            .enumerate()
            .map(|(s, p)| manhattan(world.cleaner(), to_pos(s, map.width())) as f64 * p)
            .sum::<f64>();

        println!(
            "localization error at ε = {}: {}",
            error, weighted_estimate_error
        );
    }
}

fn exercise_15_12() {
    println!("15.12");

    let mut sig_t = 1f64.powi(2);
    let sig_x = 2f64.powi(2);
    let sig_z = 1f64.powi(2);

    for t in 0..20 {
        println!("σ{}^2 = {}", t, sig_t);
        sig_t = (sig_t + sig_x) * sig_z / (sig_t + sig_x + sig_z);
    }
}

fn exercise_15_14() {
    bitflags! {
        struct Class: usize {
            const RED = 0b01;
            const SLEEP = 0b10;
        }
    }

    println!("15.14");

    #[rustfmt::skip]
    let trans = vec![
        0.7, 0.3,
        0.2, 0.8,
    ];

    let p_red = [0.7, 0.2];
    let p_sleep = [0.3, 0.1];

    let mut sensor = vec![];

    for b in 0..=Class::all().bits() {
        let o = Class::from_bits_truncate(b);
        let (f_red, t_red) = if o.contains(Class::RED) {
            (p_red[0], p_red[1])
        } else {
            (1.0 - p_red[0], 1.0 - p_red[1])
        };
        let (f_sleep, t_sleep) = if o.contains(Class::SLEEP) {
            (p_sleep[0], p_sleep[1])
        } else {
            (1.0 - p_sleep[0], 1.0 - p_sleep[1])
        };

        sensor.push(f_red * f_sleep);
        sensor.push(t_red * t_sleep);
    }

    let hmm = HMM::new(trans, sensor);
    let mut context = HMMContext::new(&hmm, vec![0.3, 0.7]);
    context.observe(Class::empty().bits());
    context.observe(Class::RED.bits());
    context.observe(Class::all().bits());

    let smooth = context.smooth();
    #[allow(clippy::needless_range_loop)]
    for t in 1..=3 {
        println!(
            "P(EnoughSleep{0} | e1:{0}) = {1:?}",
            t,
            context.filter(t).unwrap()
        );
    }
    for (t, s) in smooth.iter().enumerate().skip(1) {
        println!("P(EnoughSleep{} | e1:3) = {:?}", t, s);
    }

    context.clear();
    for t in 1..=50 {
        context.observe(Class::all().bits());
        println!("{:?}", context.filter(t).unwrap());
    }
}
