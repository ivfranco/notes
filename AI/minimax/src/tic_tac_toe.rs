use crate::{Node, State};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Copy, PartialEq)]
enum Piece {
    X,
    O,
    Empty,
}

use Piece::*;

impl Piece {
    fn char(self) -> char {
        match self {
            X => 'x',
            O => 'o',
            Empty => '.',
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' | 'X' => Ok(X),
            'o' | 'O' => Ok(O),
            '.' | ' ' => Ok(Empty),
            _ => Err("TryFrom<char> for Piece: invalid character for pieces"),
        }
    }
}

pub struct TicTacToe {
    pieces: [Piece; 9],
}

impl TicTacToe {
    pub fn init() -> Self {
        TicTacToe { pieces: [Empty; 9] }
    }

    pub fn parse(s: &str) -> Self {
        assert_eq!(s.len(), 9);

        let mut pieces = [Empty; 9];
        for (i, c) in s.chars().enumerate() {
            pieces[i] = c.try_into().unwrap();
        }

        TicTacToe { pieces }
    }

    fn unblocked(&self) -> ((i32, i32, i32), (i32, i32, i32)) {
        let mut x1 = 0;
        let mut x2 = 0;
        let mut x3 = 0;

        let mut o1 = 0;
        let mut o2 = 0;
        let mut o3 = 0;

        for seq in &[
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ] {
            let xs = seq.iter().filter(|p| self.pieces[**p] == X).count();
            let os = seq.iter().filter(|p| self.pieces[**p] == O).count();

            if os == 0 {
                match xs {
                    1 => x1 += 1,
                    2 => x2 += 1,
                    3 => x3 += 1,
                    _ => (),
                }
            }

            if xs == 0 {
                match os {
                    1 => o1 += 1,
                    2 => o2 += 1,
                    3 => o3 += 1,
                    _ => (),
                }
            }
        }

        ((x1, x2, x3), (o1, o2, o3))
    }

    pub fn evaluate(&self) -> i32 {
        let ((x1, x2, _), (o1, o2, _)) = self.unblocked();
        x1 + 3 * x2 - (o1 + 3 * o2)
    }

    fn next_piece(&self) -> Piece {
        let xs = self.pieces.iter().filter(|p| **p == X).count();
        let os = self.pieces.iter().filter(|p| **p == O).count();

        if xs > os {
            O
        } else {
            X
        }
    }

    fn filled(&self) -> bool {
        self.pieces.iter().find(|p| **p == Empty).is_none()
    }
}

impl std::fmt::Debug for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let rows = (&self.pieces)
            .chunks_exact(3)
            .map(|chunk| chunk.iter().map(|p| p.char()).collect::<String>())
            .collect::<Vec<_>>();

        write!(f, "{}", rows.join("/"))
    }
}

impl State for TicTacToe {
    type Action = usize;

    fn actions(&self) -> Vec<Self::Action> {
        self.pieces
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Empty)
            .map(|(i, _)| i)
            .collect()
    }

    fn result(&self, action: &Self::Action) -> Node<Self> {
        // MAX puts x pieces on the board, MIN puts o pieces on the board
        let mut pieces = self.pieces;

        match self.next_piece() {
            X => {
                pieces[*action] = X;
                Node::min(TicTacToe { pieces })
            }
            O => {
                pieces[*action] = O;
                Node::max(TicTacToe { pieces })
            }
            _ => unreachable!(),
        }
    }

    fn utility(&self) -> Option<f64> {
        let ((_, _, x3), (_, _, o3)) = self.unblocked();

        if x3 >= 1 {
            Some(1.0)
        } else if o3 >= 1 {
            Some(-1.0)
        } else if self.filled() {
            Some(0.0)
        } else {
            None
        }
    }
}
