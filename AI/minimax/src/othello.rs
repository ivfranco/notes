use crate::{Node, State};
use std::convert::{TryFrom, TryInto};

const SIDE: usize = 8;
const DISKS: usize = SIDE * SIDE;

type Pos = usize;

fn pos(x: isize, y: isize) -> Pos {
    assert!(x >= 0 && y >= 0);
    (x as Pos) + (y as Pos) * SIDE
}

fn unpos(pos: Pos) -> (isize, isize) {
    ((pos % SIDE) as isize, (pos / SIDE) as isize)
}

fn in_bound(x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && x < (SIDE as isize) && y < (SIDE as isize)
}

fn corners(pos: Pos) -> bool {
    let (x, y) = unpos(pos);
    let (x, y) = (x as usize, y as usize);

    (x == 0 || x == SIDE - 1) && (y == 0 || y == SIDE - 1)
}

fn sides(pos: Pos) -> bool {
    let (x, y) = unpos(pos);
    let (x, y) = (x as usize, y as usize);

    x == 0 || y == 0 || x == SIDE - 1 || y == SIDE - 1
}

#[derive(Clone, Copy, PartialEq)]
enum Player {
    Black,
    White,
}

use Player::*;

impl Player {
    fn other(self) -> Self {
        match self {
            Black => White,
            White => Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Disk {
    Occupied(Player),
    Empty,
}

use Disk::*;

impl Disk {
    fn char(self) -> char {
        match self {
            Occupied(Black) => '●',
            Occupied(White) => '○',
            Empty => '□',
        }
    }
}

impl TryFrom<char> for Disk {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Disk, Self::Error> {
        match c {
            '●' | '.' => Ok(Occupied(Black)),
            '○' | 'o' => Ok(Occupied(White)),
            '□' | ' ' => Ok(Empty),
            _ => Err("TryFrom<char> for Disk: invalid character"),
        }
    }
}

#[derive(Clone)]
struct Othello {
    next_player: Player,
    board: [Disk; DISKS],
}

impl PartialEq for Othello {
    fn eq(&self, other: &Self) -> bool {
        self.next_player == other.next_player
            && self
                .board
                .iter()
                .zip(other.board.iter())
                .all(|(d0, d1)| d0 == d1)
    }
}

impl Othello {
    fn init() -> Self {
        let mut board = [Empty; DISKS];

        board[pos(3, 3)] = Occupied(White);
        board[pos(4, 4)] = Occupied(White);
        board[pos(3, 4)] = Occupied(Black);
        board[pos(4, 3)] = Occupied(Black);

        Othello {
            next_player: Black,
            board,
        }
    }

    #[allow(dead_code)]
    fn parse(s: &str, next_player: Player) -> Self {
        let mut board = [Empty; DISKS];

        for (y, line) in s.lines().enumerate() {
            assert!(y < SIDE);
            for (x, c) in line.chars().enumerate() {
                assert!(x < SIDE);
                let pos = pos(x as isize, y as isize);
                board[pos] = c.try_into().unwrap();
            }
        }

        Othello { board, next_player }
    }

    fn get(&self, x: isize, y: isize) -> Disk {
        self.board[pos(x, y)]
    }

    fn get_pos(&self, pos: Pos) -> Disk {
        let (x, y) = unpos(pos);
        self.get(x, y)
    }

    fn flippable_towards(
        &self,
        (mut x, mut y): (isize, isize),
        (dx, dy): (isize, isize),
    ) -> Vec<Pos> {
        let mut path = vec![];
        let other = self.next_player.other();

        x += dx;
        y += dy;

        while in_bound(x, y) && self.get(x, y) == Occupied(other) {
            path.push(pos(x, y));
            x += dx;
            y += dy;
        }

        if in_bound(x, y) && self.get(x, y) == Occupied(self.next_player) {
            path
        } else {
            vec![]
        }
    }

    fn flippable(&self, placed: Pos) -> Vec<Pos> {
        const DIRS: [(isize, isize); 8] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        DIRS.iter()
            .flat_map(|dir| self.flippable_towards(unpos(placed), *dir))
            .collect()
    }

    fn valid(&self, placed: Pos) -> bool {
        !self.flippable(placed).is_empty()
    }

    fn place(&self, placed: Pos) -> Self {
        let mut flipped = self.flippable(placed);
        assert!(!flipped.is_empty());
        flipped.push(placed);

        let mut board = self.board;
        for pos in flipped {
            board[pos] = Occupied(self.next_player);
        }

        Othello {
            next_player: self.next_player.other(),
            board,
        }
    }
}

impl std::fmt::Debug for Othello {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for chunk in (&self.board).chunks_exact(SIDE) {
            let row = chunk.iter().map(|disk| disk.char()).collect::<String>();
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Pass,
    Place(Pos),
}

use Move::*;

pub struct OthelloEnv {
    othello: Othello,
    depth_limit: u32,
}

impl OthelloEnv {
    pub fn init(depth_limit: u32) -> Self {
        OthelloEnv {
            othello: Othello::init(),
            depth_limit,
        }
    }
}

impl State for OthelloEnv {
    type Action = Move;

    fn actions(&self) -> Vec<Self::Action> {
        let mut moves: Vec<_> = (0..DISKS)
            .filter(|p| self.othello.get_pos(*p) == Empty && self.othello.valid(*p))
            .map(Place)
            .collect();

        if moves.is_empty() {
            moves.push(Pass);
        }

        moves
    }

    fn result(&self, action: &Self::Action) -> Node<Self> {
        let othello = match action {
            Pass => {
                let mut othello = self.othello.clone();
                othello.next_player = othello.next_player.other();
                othello
            }
            Place(p) => self.othello.place(*p),
        };

        let env = OthelloEnv {
            othello,
            depth_limit: self.depth_limit - 1,
        };

        match env.othello.next_player {
            Black => Node::max(env),
            White => Node::min(env),
        }
    }

    fn utility(&self) -> Option<f64> {
        if self.depth_limit > 0 {
            return None;
        }

        let mut black_eval: i32 = 0;
        let mut white_eval: i32 = 0;

        for (i, d) in self.othello.board.iter().enumerate() {
            let eval = if corners(i) {
                5
            } else if sides(i) {
                3
            } else {
                1
            };

            match d {
                Occupied(Black) => black_eval += eval,
                Occupied(White) => white_eval += eval,
                _ => (),
            }
        }

        Some(f64::from(black_eval - white_eval))
    }
}

#[test]
fn flip_test() {
    let before_board = "\
□      □
   ●○   
  ○●●○  
  ○●○●  
  ○●●   
   ●●   
        
□      □";

    let after_board = "\
□      □
   ●○   
  ○●●○  
  ○●○○  
  ○○○○  
   ●●   
        
□      □";

    let before = Othello::parse(before_board, White);
    let after = Othello::parse(after_board, Black);
    assert_eq!(before.place(pos(5, 4)), after);
}

#[test]
fn search_test() {
    use crate::minimax;

    if let Place(pos) = minimax(OthelloEnv::init(5)) {
        assert!(Othello::init().valid(pos));
    } else {
        panic!("othello::search_test: Pass");
    }
}