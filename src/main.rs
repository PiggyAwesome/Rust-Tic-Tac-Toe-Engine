#![allow(unused)]
mod board;
mod cmd_io;
use board::{
    Board,
    Piece::{O, X},
};

use cmd_io::*;

fn main() {
    // let board = Board([Some(X); 9]);
    // print_board(board);

    println!("Do you want to go first?");

    let first = read_line();
    let first = if first.starts_with("y") {
        Side::Player
    } else {
        Side::Computer
    };

    init_game(first);
}
