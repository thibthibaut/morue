mod board;
use board::*;

fn main() {
    // let board: Board = Board::new();
    let board: Board = Board::from_fen(&String::from(
        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // "8/8/8/8/3Q4/8/8/8 w - - 0 1",
        "8/8/2rq4/8/3Q4/2b5/4PPP1/8 w - - 0 1",
    ));

    let moves = board.generate_moves();

    for m in moves {
        println!("Move {:#?}", m);
    }

    board.display();

    // println!("Hello, world!");
}
