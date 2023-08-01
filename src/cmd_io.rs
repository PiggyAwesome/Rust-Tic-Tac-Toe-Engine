use crate::board::*;
use std::io::stdin;

#[derive(Debug)]
pub enum Side {
    Player,
    Computer,
}

pub fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("no bitches?");

    return input;
}

pub fn print_board(board: Board) {
    for (index, row) in board.0.chunks(3).enumerate() {
        print!("  +---+---+---+\n{} |", (97 + index as u8) as char);
        for piece in row {
            if let Some(p) = piece {
                print!(" {p:?} |");
            } else {
                print!("   |")
            }
        }
        println!();
    }
    println!("  +---+---+---+\n    1   2   3");
}

fn prompt_player(board: Board) -> usize {
    println!("Choose a move (1-9): ");
    let received = read_line();
    let piece_move = received
        .chars()
        .next()
        .expect("Invalid move provided")
        .to_digit(10)
        .expect("Invalid move provided") as usize
        - 1;
    piece_move
}

pub fn init_game(first: Side) {
    // let board = Board([
    //     None,
    //     Some(Piece::X),
    //     Some(Piece::O),
    //     Some(Piece::X),
    //     Some(Piece::O),
    //     Some(Piece::X),
    //     None,
    //     None,
    //     None,
    // ]);
    // let nodes: Vec<Node> = board.search();
    // for node in nodes {
    //     println!("{}: {}", node.piece_move, node.score);
    // }
}
