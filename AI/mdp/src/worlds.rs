pub mod two_terminals {
    use crate::{
        *,
        learn::*,
    };

    use std::collections::HashSet;

    /// (0, 0) is at bottom-left
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Pos {
        x: isize,
        y: isize,
    }

    impl Pos {
        pub const fn new(x: isize, y: isize) -> Self {
            Pos { x, y }
        }

        pub fn to_usize(self, width: usize) -> usize {
            (self.x + self.y * width as isize) as usize
        }

        pub fn from_usize(i: usize, width: usize) -> Self {
            Pos::new((i % width) as isize, (i / width) as isize)
        }

        fn neighbors(self) -> [Pos; 4] {
            let (x, y) = (self.x, self.y);
            [
                Pos::new(x - 1, y),
                Pos::new(x + 1, y),
                Pos::new(x, y - 1),
                Pos::new(x, y + 1),
            ]
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

    pub const DEFAULT_WIDTH: usize = 4;
    pub const DEFAULT_HEIGHT: usize = 3;

    const BLOCK: Pos = Pos::new(1, 1);
    const POS_FINAL: Pos = Pos::new(3, 2);
    const NEG_FINAL: Pos = Pos::new(3, 1);

    const P_FORWARD: Prob = 0.8;
    const P_SIDEWAY: Prob = 0.1;

    pub struct Map {
        discount: f64,
        width: usize,
        rewards: Vec<Util>,
        blocks: HashSet<Pos>,
        terminals: HashSet<Pos>,
    }

    impl Map {
        pub fn full(
            discount: f64,
            width: usize,
            rewards: Vec<Util>,
            blocks: &[Pos],
            terminals: &[Pos],
        ) -> Self {
            assert_eq!(
                rewards.len() % width,
                0,
                "Map::full: rewards must be complete"
            );
            let blocks = blocks.iter().cloned().collect();

            let terminals = terminals.iter().cloned().collect();

            Map {
                discount,
                width,
                rewards,
                blocks,
                terminals,
            }
        }

        pub fn new(penalty: Util) -> Self {
            let mut rewards = vec![penalty; DEFAULT_WIDTH * DEFAULT_HEIGHT];
            rewards[POS_FINAL.to_usize(DEFAULT_WIDTH)] = 1.0;
            rewards[NEG_FINAL.to_usize(DEFAULT_WIDTH)] = -1.0;
            rewards[BLOCK.to_usize(DEFAULT_WIDTH)] = 0.0;

            Self::full(
                1.0,
                DEFAULT_WIDTH,
                rewards,
                &[BLOCK],
                &[POS_FINAL, NEG_FINAL],
            )
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.rewards.len() / self.width()
        }

        fn in_bound(&self, pos: Pos) -> bool {
            pos.x >= 0
                && pos.x < self.width() as isize
                && pos.y >= 0
                && pos.y < self.height() as isize
                && !self.blocked(pos)
        }

        pub fn blocked(&self, pos: Pos) -> bool {
            self.blocks.contains(&pos)
        }

        pub fn walls(&self, pos: Pos) -> usize {
            if !self.in_bound(pos) {
                0
            } else {
                4 - pos
                    .neighbors()
                    .iter()
                    .filter(|&&n| self.in_bound(n))
                    .count()
            }
        }

        pub fn terminal(&self, pos: Pos) -> bool {
            self.terminals.contains(&pos)
        }

        pub fn trap(&self, pos: Pos) -> bool {
            self.blocked(pos) || self.terminal(pos)
        }
    }

    impl Default for Map {
        fn default() -> Self {
            Self::new(-0.04)
        }
    }

    impl MDP for Map {
        type State = Pos;
        type Action = Dir;

        fn states(&self) -> usize {
            self.width() * self.height()
        }

        fn discount(&self) -> f64 {
            self.discount
        }

        fn encode(&self, state: &Pos) -> usize {
            state.to_usize(self.width())
        }

        fn decode(&self, i: usize) -> Pos {
            Pos::from_usize(i, self.width())
        }

        fn reward(&self, state: &Pos) -> Util {
            self.rewards[self.encode(state)]
        }

        fn apply(&self, pos: &Pos, dir: &Dir) -> Vec<(Prob, Pos)> {
            if self.trap(*pos) {
                // illegal, result is assumed to be meaningless
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
            if self.trap(*state) {
                vec![]
            } else {
                vec![N, S, W, E]
            }
        }
    }

    impl Simulate for Map {
        fn start(&self) -> Pos {
            let mut rng = thread_rng();
            loop {
                let x = rng.gen_range(0, self.width() as isize);
                let y = rng.gen_range(0, self.height() as isize);
                let pos = Pos::new(x, y);
                if !self.trap(pos) {
                    return pos;
                }
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
            assert_eq!(map.encode(&state), i);

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
                // illegal, result is assumed to be meaningless
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

pub mod three_states {
    use crate::*;

    #[derive(Clone, Copy, PartialEq)]
    pub enum OTT {
        One,
        Two,
        Three,
    }

    use OTT::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Move {
        A,
        B,
    }

    use Move::*;

    impl OTT {
        fn to_usize(self) -> usize {
            match self {
                One => 0,
                Two => 1,
                Three => 2,
            }
        }

        fn from_usize(code: usize) -> Self {
            match code {
                0 => One,
                1 => Two,
                2 => Three,
                _ => unreachable!(),
            }
        }
    }

    pub struct Context {
        discount: f64,
    }

    impl Context {
        pub fn new(discount: f64) -> Self {
            Context { discount }
        }
    }

    impl MDP for Context {
        type State = OTT;
        type Action = Move;

        fn states(&self) -> usize {
            3
        }

        fn discount(&self) -> f64 {
            self.discount
        }

        fn reward(&self, state: &Self::State) -> Util {
            match state {
                One => -1.0,
                Two => -2.0,
                Three => 0.0,
            }
        }

        fn encode(&self, state: &Self::State) -> usize {
            state.to_usize()
        }

        fn decode(&self, code: usize) -> Self::State {
            Self::State::from_usize(code)
        }

        fn apply(&self, state: &Self::State, action: &Self::Action) -> Vec<(Prob, Self::State)> {
            match (state, action) {
                // should be illegal
                (Three, _) => vec![(1.0, Three)],
                (One, A) => vec![(0.8, Two), (0.2, One)],
                (Two, A) => vec![(0.8, One), (0.2, Two)],
                (ott, B) => vec![(0.1, Three), (0.9, *ott)],
            }
        }
    }

    impl SoloMDP for Context {
        fn actions(&self, state: &Self::State) -> Vec<Self::Action> {
            match state {
                One | Two => vec![A, B],
                _ => vec![],
            }
        }
    }
}
