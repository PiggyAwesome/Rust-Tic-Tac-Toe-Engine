use std::f32::INFINITY;

use crate::cmd_io::print_board;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    X,
    O,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::O => "O",
            Self::X => "X",
        };
        write!(f, "{result}")
    }
}

// pub fn print_piece(piece: Option<Piece>) -> String {
//     // 🕊️
//     match piece {
//         Piece::X => "X".to_string(),
//         Piece::O => "O".to_string(),
//     }
// }

#[derive(Clone, Copy)]
pub struct Board(pub [Option<Piece>; 9]);

#[derive(Clone, Copy)]
pub struct Node {
    pub score: i32,
    pub piece_move: usize,
}

impl Node {
    fn new(piece_move: usize, score: i32) -> Node {
        Node { piece_move, score }
    }
}

impl Board {
    pub fn new() -> Board {
        Board([None; 9])
    }

    pub fn full(&self) -> bool {
        !self.0.contains(&None)
    }

    pub fn winner(&self) -> Option<Piece> {
        match self.0 {
            [Some(a), Some(b), Some(c), _, _, _, _, _, _] if a == b && b == c => Some(a),
            [_, _, _, Some(a), Some(b), Some(c), _, _, _] if a == b && b == c => Some(a),
            [_, _, _, _, _, _, Some(a), Some(b), Some(c)] if a == b && b == c => Some(a),
            [Some(a), _, _, Some(b), _, _, Some(c), _, _] if a == b && b == c => Some(a),
            [_, Some(a), _, _, Some(b), _, _, Some(c), _] if a == b && b == c => Some(a),
            [_, _, Some(a), _, _, Some(b), _, _, Some(c)] if a == b && b == c => Some(a),
            [Some(a), _, _, _, Some(b), _, _, _, Some(c)] if a == b && b == c => Some(a),
            [_, _, Some(a), _, Some(b), _, Some(c), _, _] if a == b && b == c => Some(a),
            _ => None,
        }
    }

    pub fn open(&self) -> Vec<usize> {
        let mut open_squares: Vec<usize> = vec![];
        for (index, square) in self.0.iter().enumerate() {
            if square.is_none() {
                open_squares.push(index)
            }
        }
        open_squares
    }

    pub fn turn(&self) -> Piece {
        if self.open().len() % 2 == 0 {
            Piece::O
        } else {
            Piece::X
        }
    }

    pub fn evaluate(&self) -> i32 {
        match self.winner() {
            Some(Piece::X) => 1,
            Some(Piece::O) => -1,
            None => 0,
        }
    }

    fn minimax(&self) -> i32 {
        if self.full() || self.winner().is_some() {
            return self.evaluate();
        }
        let mut board = self.clone();
        let maximizing = self.turn() == Piece::X;
        if maximizing {
            // X to move
            let mut best: i32 = -1;
            for piece_move in board.open() {
                // Generate new board position after playing move.
                board.0[piece_move] = Some(Piece::X);
                // Determine score
                let score = board.minimax();
                best = best.max(score);
                // Undo move
                board.0[piece_move] = None;
            }
            return best;
        } else {
            // O to move
            let mut best: i32 = 1;
            for piece_move in board.open() {
                // Generate new position after playing move
                board.0[piece_move] = Some(Piece::O);
                // Determine score
                let score = board.minimax();
                best = best.min(score);
                // Undo move
                board.0[piece_move] = None;
            }
            return best;
        }
    }

    pub fn search(&self) -> Node {
        let mut board = self.clone();
        let mut nodes: Vec<Node> = vec![];
        for piece_move in self.open() {
            board.0[piece_move] = Some(board.turn());
            nodes.push(Node::new(piece_move, board.minimax()));
            board.0[piece_move] = None;
        }
        nodes.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        nodes[0]
    }
}
