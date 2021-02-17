mod piece;
use piece::*;
 
pub struct Board {

    // The board is 64 squares containing a piece or not 
    pieces : [Option<Piece>; 64]

}

fn make_piece(color: Color, kind: Kind) -> Option<Piece> {
    Some(Piece {
        color,
        kind
    })
}

impl Board {
    pub fn new() -> Self {


        let first_rank = [Kind::Rook, Kind::Knight, Kind::Bishop, Kind::King, Kind::Queen, Kind::Bishop, Kind::Knight, Kind::Rook];
        let mut init_pieces = [None; 64];

        for i in 0..8 {
            init_pieces[i] = make_piece(Color::White, first_rank[i]);
        }
        for i in 8..16 {
            init_pieces[i] = make_piece(Color::White, Kind::Pawn);
        }

        let board : Board  = Board {
            pieces : init_pieces
        };

        return board
    }

    pub fn display(&self) {


        // we start by the last rank
        for rank in (0..8).rev() {
            for file in 0..8 {

                let square = match self.pieces[file + 8* rank ] {

                    // If there is no piece here
                    None => {
                        match (file+rank) % 2 {
                            0 => "◻",
                            1 => "◼",
                            _ => "?"
                        }
                    }

                    // If it's some white piece
                    Some(Piece { color: Color::White, kind: k }) => {
                        match k {
                            Kind::Pawn => "♙",
                            Kind::Knight => "♘",
                            Kind::Bishop => "♗",
                            Kind::Rook => "♖",
                            Kind::King => "♔",
                            Kind::Queen => "♕",
                        }
                    }

                    // If it's a black piece
                    //TODO
                    _ => "?"
                };

                print!("{} ", square);

            }// end iteration trhu files
            println!(""); // line break
        }

    }

}
