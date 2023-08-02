use crate::board::*;
use std::io::stdin;

#[derive(Debug, PartialEq)]
pub enum Side {
    Player,
    Computer,
}

pub fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error reading line.");

    return input;
}

pub fn print_board(board: &Board) {
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

fn prompt_player(board: &Board) -> usize {
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
    let mut board = Board::new();
    if first == Side::Computer {
        let Node {piece_move, score} = board.search();
        board.0[piece_move] = Some(Piece::X);
    }
    while !board.full() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print_board(&board);

        if board.winner().is_some() {
            println!("{} wins.", board.winner().unwrap());
            break;
        }
        let player_move = prompt_player(&board);
        if board.0[player_move].is_some() {
            panic!("Invalid move.");
        }
        board.0[player_move] = Some(board.turn());

        if board.full() {
            println!("The game is a draw.");
            break;
        };

        if board.winner().is_some() {
            println!("{} wins.", board.winner().unwrap());
            break;
        }

        let Node { piece_move, score } = board.search();
        board.0[piece_move] = Some(board.turn());

    }
}
