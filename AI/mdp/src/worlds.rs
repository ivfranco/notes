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

        pub fn to_usize(self) -> usize {
            (self.x + self.y * WIDTH) as usize
        }

        pub fn from_usize(i: usize) -> Self {
            let i = i as isize;
            Pos::new(i % WIDTH, i / WIDTH)
        }
    }

    impl std::fmt::Debug for Pos {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
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

    impl Default for Dir {
        fn default() -> Self {
            N
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

        fn terminal(&self, pos: Pos) -> bool {
            match pos {
                POS_FINAL | NEG_FINAL | BLOCK => true,
                _ => false,
            }
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

        fn encode(&self, state: &Pos) -> usize {
            state.to_usize()
        }

        fn decode(&self, i: usize) -> Pos {
            Pos::from_usize(i)
        }

        fn reward(&self, state: &Pos) -> Util {
            match *state {
                POS_FINAL => 1.0,
                NEG_FINAL => -1.0,
                BLOCK => 0.0,
                _ => self.penalty,
            }
        }

        fn apply(&self, pos: &Pos, dir: &Dir) -> Vec<(Prob, Pos)> {
            if self.terminal(*pos) {
                return vec![];
            }

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

    impl SoloMDP for Map {
        fn actions(&self, state: &Pos) -> Vec<Dir> {
            if self.terminal(*state) {
                vec![]
            } else {
                vec![N, S, W, E]
            }
        }
    }

    #[cfg(test)]
    const EPSILON: Prob = 0.001;

    #[test]
    fn state_action_test() {
        let map = Map::new(-0.04);

        for i in 0..map.states() {
            let state = map.decode(i);
            assert_eq!(state.to_usize(), i);

            for action in map.actions(&state) {
                let p_sum: Prob = map.apply(&state, &action).into_iter().map(|(p, _)| p).sum();
                assert!((p_sum - 1.0).abs() <= EPSILON);
            }
        }
    }

    #[test]
    fn utils_test() {
        #[rustfmt::skip]
        let utils = [
            0.705, 0.655, 0.611, 0.388, 
            0.762, 0.0, 0.660, -1.0, 
            0.812, 0.868, 0.918, 1.0,
        ];
        let calculated_utils = value_iteration(&Map::new(-0.04), EPSILON);
        assert!(max_norm(&utils, &calculated_utils) <= EPSILON);
    }

    #[test]
    fn policy_test() {
        #[rustfmt::skip]
        let policy = [
            Some(N), Some(E), Some(N), Some(W),
            Some(N), None, Some(N), None,
            Some(E), Some(E), Some(E), None, 
        ];

        let mut penalty = -0.42;
        while penalty < -0.085 {
            let map = Map::new(penalty);
            let utils = value_iteration(&map, EPSILON);
            assert_eq!(policy_from(&map, &utils), policy);
            let calculated_policy = policy_iteration(&map);
            assert_eq!(calculated_policy, policy);
            penalty += 0.01;
        }
    }
}

pub mod simple_game {
    use crate::{Player::*, *};

    const DEFAULT_WIDTH: usize = 4;

    #[derive(Clone, Copy)]
    pub struct Players {
        player_a: usize,
        player_b: usize,
    }

    impl Players {
        fn new(player_a: usize, player_b: usize) -> Self {
            Players { player_a, player_b }
        }
    }

    impl std::fmt::Debug for Players {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.player_a, self.player_b)
        }
    }

    #[derive(Clone, Copy)]
    pub struct Goto(Player, usize);

    pub struct Board {
        width: usize,
    }

    impl Board {
        pub fn new(width: usize) -> Board {
            assert!(width >= 3);

            Board { width }
        }

        fn terminal(&self, players: Players) -> bool {
            players.player_b == 0 || players.player_a == self.width - 1
        }

        pub fn valid(&self, players: Players) -> bool {
            let (a, b) = (players.player_a, players.player_b);
            a != b && !(a == self.width - 1 && b == 0)

        }

        fn reward(&self, players: Players) -> Util {
            match (players.player_a, players.player_b) {
                _ if !self.valid(players) => 0.0,
                (_, 0) => -1.0,
                (a, _) if a == self.width - 1 => 1.0,
                _ => 0.0,
            }
        }

        fn encode(&self, state: Players) -> usize {
            self.width * state.player_a + state.player_b
        }

        fn decode(&self, code: usize) -> Players {
            Players::new(code / self.width, code % self.width)
        }

        fn actions(&self, state: Players, player: Player) -> Vec<Goto> {
            if self.terminal(state) || !self.valid(state) {
                return vec![];
            }

            let (pa, pb) = (state.player_a, state.player_b);
            let (pm, ps) = if player == Player::Maxer {
                (pa, pb)
            } else {
                (pb, pa)
            };

            let mut gotos = vec![];

            if let Some(left) = (0..pm).rev().find(|&i| i != ps) {
                gotos.push(Goto(player, left));

            }
            if let Some(right) = (pm + 1..self.width).find(|&i| i != ps) {
                gotos.push(Goto(player, right));
            }

            gotos
        }

        fn apply(&self, state: Players, action: Goto) -> Vec<(Prob, Players)> {
            let next = match action {
                Goto(Maxer, i) => Players::new(i, state.player_b),
                Goto(Miner, i) => Players::new(state.player_a, i),
            };

            vec![(1.0, next)]
        }
    }

    impl Default for Board {
        fn default() -> Self {
            Board::new(DEFAULT_WIDTH)
        }
    }

    impl MDP for Board {
        type State = Players;
        type Action = Goto;

        fn states(&self) -> usize {
            self.width.pow(2)
        }

        fn reward(&self, state: &Self::State) -> Util {
            self.reward(*state)
        }

        fn discount(&self) -> f64 {
            1.0
        }

        fn encode(&self, state: &Self::State) -> usize {
            self.encode(*state)
        }

        fn decode(&self, code: usize) -> Self::State {
            self.decode(code)
        }

        fn apply(&self, state: &Self::State, action: &Self::Action) -> Vec<(Prob, Self::State)> {
            self.apply(*state, *action)
        }
    }

    impl ZeroSumMDP for Board {
        fn actions(&self, state: &Self::State, player: Player) -> Vec<Self::Action> {
            self.actions(*state, player)
        }
    }

}
