use std::cmp::Ordering;

pub type Prob = f64;
pub type Util = f64;

pub trait State: Sized {
    fn to_usize(&self) -> usize;
    fn from_usize(i: usize) -> Self;
}

pub trait MDP {
    type State: State;
    type Action;

    fn states(&self) -> usize;
    fn discount(&self) -> Util;
    fn reward(&self, state: &Self::State) -> Util;
    fn actions(&self, state: &Self::State) -> Vec<Self::Action>;
    fn apply(&self, state: &Self::State, action: &Self::Action) -> Vec<(Prob, Self::State)>;
}

const ALMOST_ONE: f64 = 0.999_999_999;

pub fn value_iteration<M>(mdp: &M, error: Prob) -> Vec<Util>
where
    M: MDP,
{
    let mut utils = vec![0.0; mdp.states()];

    loop {
        let next_utils = next_utils(mdp, &utils);
        let max_norm = max_norm(&utils, &next_utils);
        // when discount >= 1, the loop will not terminate
        // the error threshold is loosened a little bit in that case
        let gamma = if mdp.discount() >= 1.0 {
            ALMOST_ONE
        } else {
            mdp.discount()
        };

        if max_norm < error * (1.0 - gamma) / gamma {
            return next_utils;
        } else {
            utils = next_utils;
        }
    }
}

fn next_utils<M, S>(mdp: &M, utils: &[Util]) -> Vec<Util>
where
    M: MDP<State = S>,
    S: State,
{
    let mut next_utils = vec![0.0; utils.len()];
    for (i, u) in next_utils.iter_mut().enumerate() {
        let state = S::from_usize(i);
        let max_action = mdp
            .actions(&state)
            .into_iter()
            .map(|action| {
                mdp.apply(&state, &action)
                    .into_iter()
                    .map(|(p, s)| p * utils[s.to_usize()])
                    .sum()
            })
            .max_by(|&a, &b| cmp_f64(a, b))
            .unwrap_or(0.0);

        *u = mdp.discount() * max_action + mdp.reward(&state);
    }
    next_utils
}

fn max_norm(utils: &[Util], next_utils: &[Util]) -> Util {
    utils
        .iter()
        .zip(next_utils)
        .map(|(a, b)| (a - b).abs())
        .max_by(|&a, &b| cmp_f64(a, b))
        .unwrap_or(0.0)
}

fn cmp_f64(a: f64, b: f64) -> Ordering {
    assert!(!a.is_nan());
    assert!(!b.is_nan());

    a.partial_cmp(&b).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    /// (0, 0) is at bottom-left
    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Pos {
        x: isize,
        y: isize,
    }

    impl Pos {
        const fn new(x: isize, y: isize) -> Self {
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

    #[derive(Clone, Copy)]
    enum Dir {
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

    struct Map {
        penalty: Util,
    }

    impl Map {
        fn new(penalty: Util) -> Self {
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
