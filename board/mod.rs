mod piece;
use piece::*;
 
pub struct Board {

    pieces : [Piece; 2]

}

fn make_piece(color: PieceColor, kind: PieceKind) -> Piece {
    Piece {
        color,
        kind
    }
}

impl Board {
    pub fn new() -> Self {


        let init_pieces = [make_piece( PieceColor::None, PieceKind::None ); 2];

        let board : Board  = Board {
            pieces : init_pieces
        };

        return board
    }

}
