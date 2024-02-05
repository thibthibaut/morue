mod board;
use board::*;

fn main() {
    // let board: Board = Board::new();
    let board = Board::from_fen(&String::from(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // "8/8/8/8/3Q4/8/8/8 w - - 0 1",
                                                                    //"8/8/2rq4/8/3Q4/2b5/4PPP1/8 w - - 0 1",
                                                                    // "8/4P3/8/8/8/8/3P4/8 w - - 0 1",
                                                                    // "8/1p6/1p6/1p6/8/8/8/8 w - - 0 1",
                                                                    // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                                                                    //"rnbqkbn1/ppp2ppr/8/2Ppp2p/8/4P3/PP1PKPPP/RNBQ1BNR w q d6 0 6",
                                                                    // "rnbqkbnr/1pppppp1/p7/6Pp/8/8/PPPPPP1P/RNBQKBNR w KQkq a6 0 3",
                                                                    // "rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq _ 0 3",
                                                                    //"8/8/8/8/8/5p2/4P3/8 w - - 0 1",
                                                                    // "8/8/8/8/8/7p/7P/8 w - - 0 1",
                                                                    // "8/8/8/8/8/6p1/7P/8 w - - 0 1"
                                                                    // "8/8/8/p7/P7/8/8/8 w - - 0 1",
                                                                    // "8/8/8/1p6/P7/8/8/8 b - - 0 1",
                                                                    // "8/8/8/8/8/8/4p3/3P4 b - - 0 1",
                                                                    //"8/8/8/3p4/2PP4/8/8/8 b - - 0 1"
    ))
    .unwrap();

    let moves = board.generate_moves();

    for m in moves {
        println!("{:#?}", m);
    }

    board.display();

    // println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

    #[test]
    fn test_starting_pos() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(&String::from(fen)).unwrap();
        let moves = board.generate_moves();

        // Check if the number of generated moves is 20
        assert_eq!(moves.len(), 20, "The number of moves should be 20");
    }

    #[test]
    fn test_knight_moves() {
        let fen = "N6N/8/6r1/N2N4/5N2/8/8/N6N w KQkq - 0 1";
        let board = Board::from_fen(&String::from(fen)).unwrap();
        let moves = board.generate_moves();
        assert_eq!(moves.len(), 26, "The number of knight moves should be 26");
    }
    #[test]
    fn test_rook_moves_blocked() {
        let fen =
            "RPNNNNPR/PNNNNNNP/NNNNNNNN/NNNNNNNN/NNNNNNNN/NNNNNNNN/NNNNNNNN/RNNNNNNR w KQkq - 0 1";
        let board = Board::from_fen(&String::from(fen)).unwrap();
        let moves = board.generate_moves();
        for m in &moves {
            println!("{:#?}", m);
        }
        assert_eq!(moves.len(), 0, "The number of rook moves should be 0");
    }

    #[test]
    fn test_one_rook_moves() {
        let fen = "8/8/8/8/8/8/8/7R w KQkq - 0 1";
        let board = Board::from_fen(&String::from(fen)).unwrap();
        let moves = board.generate_moves();
        for m in &moves {
            println!("{:#?}", m);
        }
        assert_eq!(moves.len(), 14, "The number of rook moves should be 14");
    }
    #[test]
    fn test_rook_capture_moves() {
        let fen = "8/8/3n4/8/1n1R1n2/8/3n4/8 w KQkq - 0 1";
        let board = Board::from_fen(&String::from(fen)).unwrap();
        let moves = board.generate_moves();
        for m in &moves {
            println!("{:#?}", m);
        }
        assert_eq!(moves.len(), 8, "The number of rook moves should be 8");
    }
}
