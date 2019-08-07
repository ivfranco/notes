use std::cmp::Ordering;

pub type Prob = f64;
pub type Util = f64;

pub const DEFAULT_DISCOUNT: f64 = 0.999_999;

pub trait State: Sized {
    type Act: Action<Self>;

    fn to_usize(&self) -> usize;
    fn from_usize(i: usize) -> Self;
    fn actions(&self) -> Vec<Self::Act>;
}

pub trait Action<S> {
    fn apply(&self, state: &S) -> Vec<(Prob, S)>;
}

pub struct MDP {
    rewards: Vec<Util>,
}

impl MDP {
    pub fn new(rewards: Vec<Util>) -> Self {
        MDP { rewards }
    }

    fn states(&self) -> usize {
        self.rewards.len()
    }

    fn reward(&self, i: usize) -> Util {
        self.rewards[i]
    }
}

pub fn value_iteration<S>(mdp: &MDP, error: Prob, discount: f64) -> Vec<Util>
where
    S: State,
{
    let mut utils = vec![0.0; mdp.states()];

    loop {
        let next_utils = next_utils::<S>(mdp, &utils, discount);
        let max_norm = max_norm(&utils, &next_utils);
        if max_norm < error * (1.0 - discount) / discount {
            return next_utils;
        } else {
            utils = next_utils;
        }
    }
}

fn next_utils<S>(mdp: &MDP, utils: &[Util], discount: f64) -> Vec<Util>
where
    S: State,
{
    let mut next_utils = vec![0.0; utils.len()];
    for (i, u) in next_utils.iter_mut().enumerate() {
        let state = S::from_usize(i);
        let max_action = state
            .actions()
            .into_iter()
            .map(|action| {
                action
                    .apply(&state)
                    .into_iter()
                    .map(|(p, s)| p * utils[s.to_usize()])
                    .sum()
            })
            .max_by(|&a, &b| cmp_f64(a, b))
            .unwrap_or(0.0);

        *u = discount * max_action + mdp.reward(i);
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

    const WIDTH: isize = 4;
    const HEIGHT: isize = 3;

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

        fn in_bound(self) -> bool {
            self.x >= 0 && self.x < WIDTH &&
            self.y >= 0 && self.y < HEIGHT &&
            // (1, 1) is blocked
            self != BLOCK
        }
    }

    const BLOCK: Pos = Pos::new(1, 1);
    const POS_FINAL: Pos = Pos::new(3, 2);
    const NEG_FINAL: Pos = Pos::new(3, 1);

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

    const P_FORWARD: Prob = 0.8;
    const P_SIDEWAY: Prob = 0.1;

    impl Action<Pos> for Dir {
        fn apply(&self, pos: &Pos) -> Vec<(Prob, Pos)> {
            let mut p_stay = 0.0;
            let mut successors = vec![];

            for &(p, dir) in [
                (P_FORWARD, *self),
                (P_SIDEWAY, self.left()),
                (P_SIDEWAY, self.right()),
            ]
            .iter()
            {
                let target = dir.from(*pos);
                if target.in_bound() {
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

    impl State for Pos {
        type Act = Dir;

        fn to_usize(&self) -> usize {
            (self.x + self.y * WIDTH) as usize
        }

        fn from_usize(i: usize) -> Self {
            let i = i as isize;
            Pos::new(i % WIDTH, i / WIDTH)
        }

        fn actions(&self) -> Vec<Self::Act> {
            match *self {
                POS_FINAL | NEG_FINAL | BLOCK => vec![],
                _ => vec![N, S, W, E],
            }
        }
    }

    const EPSILON: Prob = 0.001;

    #[test]
    fn state_action_test() {
        for i in 0 .. (WIDTH * HEIGHT) as usize {
            let state = <Pos as State>::from_usize(i);
            assert_eq!(state.to_usize(), i);

            for action in state.actions() {
                let p_sum: Prob = action.apply(&state)
                    .into_iter()
                    .map(|(p, _)| p)
                    .sum();
                
                assert!((p_sum - 1.0).abs() <= EPSILON);
            }
        }
    }

    #[test]
    fn utils_test() {
        let rewards = (0 .. (WIDTH * HEIGHT) as usize)
            .map(|i| {
                match Pos::from_usize(i) {
                    POS_FINAL => 1.0,
                    NEG_FINAL => -1.0,
                    BLOCK => 0.0,
                    _ => -0.04,
                }
            })
            .collect();

        let utils = [0.705, 0.655, 0.611, 0.388, 0.762 , 0.0 , 0.660, -1.0, 0.812, 0.868, 0.918, 1.0];
        let calculated_utils = value_iteration::<Pos>(&MDP::new(rewards), EPSILON, DEFAULT_DISCOUNT);
        assert!(max_norm(&utils, &calculated_utils) <= EPSILON);
    }
}
