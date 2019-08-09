pub mod two_terminals {
    use crate::*;

    /// (0, 0) is at bottom-left
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Pos {
        x: isize,
        y: isize,
    }

    impl Pos {
        pub const fn new(x: isize, y: isize) -> Self {
            Pos { x, y }
        }
    }

    impl State for Pos {
        fn to_usize(&self) -> usize {
            (self.x + self.y * WIDTH) as usize
        }

        fn from_usize(i: usize) -> Self {
            let i = i as isize;
            Pos::new(i % WIDTH, i / WIDTH)
        }
    }

    impl std::fmt::Debug for Pos {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    

    #[derive(Debug, Clone, Copy)]
    pub enum Dir {
        N,
        S,
        W,
        E,
    }

    use Dir::*;

    impl Dir {
        fn offset(self) -> (isize, isize) {
            match self {
                N => (0, 1),
                S => (0, -1),
                W => (-1, 0),
                E => (1, 0),
            }
        }

        fn left(self) -> Self {
            match self {
                N => W,
                W => S,
                S => E,
                E => N,
            }
        }

        fn right(self) -> Self {
            match self {
                N => E,
                E => S,
                S => W,
                W => N,
            }
        }

        fn from(self, pos: Pos) -> Pos {
            let Pos { x, y } = pos;
            let (dx, dy) = self.offset();
            Pos::new(x + dx, y + dy)
        }
    }

    const WIDTH: isize = 4;
    const HEIGHT: isize = 3;

    const BLOCK: Pos = Pos::new(1, 1);
    const POS_FINAL: Pos = Pos::new(3, 2);
    const NEG_FINAL: Pos = Pos::new(3, 1);

    const P_FORWARD: Prob = 0.8;
    const P_SIDEWAY: Prob = 0.1;

    pub struct Map {
        penalty: Util,
    }

    impl Map {
        pub fn new(penalty: Util) -> Self {
            Map { penalty }
        }

        fn in_bound(&self, pos: Pos) -> bool {
            pos.x >= 0 && pos.x < WIDTH &&
            pos.y >= 0 && pos.y < HEIGHT &&
            // (1, 1) is blocked
            pos != BLOCK
        }
    }

    impl MDP for Map {
        type State = Pos;
        type Action = Dir;

        fn states(&self) -> usize {
            (WIDTH * HEIGHT) as usize
        }

        fn discount(&self) -> Util {
            1.0
        }

        fn reward(&self, state: &Pos) -> Util {
            match *state {
                POS_FINAL => 1.0,
                NEG_FINAL => -1.0,
                BLOCK => 0.0,
                _ => self.penalty,
            }
        }

        fn actions(&self, state: &Pos) -> Vec<Dir> {
            match *state {
                POS_FINAL | NEG_FINAL | BLOCK => vec![],
                _ => vec![N, S, W, E],
            }
        }

        fn apply(&self, pos: &Pos, dir: &Dir) -> Vec<(Prob, Pos)> {
            let mut p_stay = 0.0;
            let mut successors = vec![];

            for &(p, dir) in [
                (P_FORWARD, *dir),
                (P_SIDEWAY, dir.left()),
                (P_SIDEWAY, dir.right()),
            ]
            .iter()
            {
                let target = dir.from(*pos);
                if self.in_bound(target) {
                    successors.push((p, target));
                } else {
                    p_stay += p;
                }
            }

            if p_stay > 0.0 {
                successors.push((p_stay, *pos));
            }

            successors
        }
    }

    #[cfg(test)]
    const EPSILON: Prob = 0.001;

    #[test]
    fn state_action_test() {
        let map = Map::new(-0.04);

        for i in 0..map.states() {
            let state = <Pos as State>::from_usize(i);
            assert_eq!(state.to_usize(), i);

            for action in map.actions(&state) {
                let p_sum: Prob = map.apply(&state, &action).into_iter().map(|(p, _)| p).sum();

                assert!((p_sum - 1.0).abs() <= EPSILON);
            }
        }
    }

    #[test]
    fn utils_test() {
        let utils = [
            0.705, 0.655, 0.611, 0.388, 0.762, 0.0, 0.660, -1.0, 0.812, 0.868, 0.918, 1.0,
        ];
        let calculated_utils = value_iteration(&Map::new(-0.04), EPSILON);
        assert!(max_norm(&utils, &calculated_utils) <= EPSILON);
    }
}