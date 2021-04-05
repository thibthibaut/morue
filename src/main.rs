mod board;
use board::*;

fn main() {
    let board: Board = Board::new();

    let moves = board.generate_moves();

    for m in moves {
        println!("Move {:#?}", m);
    }

    board.display();

    // println!("Hello, world!");
}
