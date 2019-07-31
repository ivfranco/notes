use markov::{HMM, HMMContext, Prob, Observation, State};

pub mod vaccum_world {
    use super::*;
    use bitflags::bitflags;

    #[derive(Clone, Copy, PartialEq)]
    enum Square {
        Empty,
        Block,
    }

    use Square::*;

    type Pos = (isize, isize);

    fn to_pos(idx: usize, width: usize) -> Pos {
        ((idx / width) as isize, (idx % width) as isize)
    }

    fn from_pos((y, x): Pos, width: usize) -> usize {
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
    }

    #[rustfmt::skip]
    const DEFAULT_MAP: [[u8; 16]; 4] = [
        [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1],
        [0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0],
        [0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1],
        [1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    ];

    pub struct Map {
        width: usize,
        squares: Vec<Square>,
    }

    impl Map {
        fn new(width: usize, squares: Vec<Square>) -> Self {
            assert_eq!(squares.len() % width, 0, "Map::new: map is not a square");
            Map { width, squares }
        }
        
        fn width(&self) -> usize {
            self.width
        }

        fn height(&self) -> usize {
            self.states() / self.width()
        }

        fn in_bound(&self, (y, x): Pos) -> bool {
            0 <= y && y < self.height() as isize &&
            0 <= x && x < self.width() as isize
        }

        fn is_empty(&self, pos: Pos) -> bool {
            self.in_bound(pos) && self.squares[from_pos(pos, self.width())] == Empty
        }

        fn sensor_at(&self, idx: usize) -> Sensor {
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

        fn neighbors(&self, idx: usize) -> Vec<usize> {
            self.sensor_at(idx)
                .neighbors(to_pos(idx, self.width()))
                .into_iter()
                .map(|pos| from_pos(pos, self.width()))
                .collect()

        }

        fn states(&self) -> usize {
            self.squares.len()
        }

        pub fn as_hmm(&self, error: Prob) -> HMM {
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
            for sensor in 0 .. 2usize.pow(Sensor::DIRS) {
                sensor_model.extend((0 .. self.states()).map(|state| {
                    let s = self.sensor_at(state);
                    let d = Sensor::from_bits_truncate(sensor as u8).diff(s) as i32;
                    (1.0 - error).powi(Sensor::DIRS as i32 - d) * error.powi(d)
                }));
            }

            HMM::new(trans, sensor_model)
        }

        pub fn uniform_prior(&self) -> Vec<Prob> {
            self.squares.iter().map(|&b| if b == Empty { 1.0 } else { 0.0 }).collect()
        }
    }

    impl Default for Map {
        fn default() -> Self {
            let squares = DEFAULT_MAP.iter()
                .flatten()
                .map(|&b| if b == 1 { Empty} else { Block })
                .collect();
            
            let width = DEFAULT_MAP[0].len();

            Map::new(width, squares)
        }
    }

    struct VaccumWorld<'a> {
        map: &'a Map,
        cleaner: Pos,
    }
}

fn main() {
    println!("Hello, world!");
}
