mod moves;
mod piece;
use moves::*;
use piece::*;

pub struct Board {
    // The board is 64 squares containing a piece or not
    pieces: [Option<Piece>; 64],

    // Store positions of whites pieces and black pieces for fast lookup
    // whitePiecesPos: Vec<i32>,
    // blackPiecesPos: Vec<i32>,
    side_to_play: Color,
}

impl Board {
    pub fn new() -> Self {
        let first_rank = [
            Kind::Rook,
            Kind::Knight,
            Kind::Bishop,
            Kind::Queen,
            Kind::King,
            Kind::Bishop,
            Kind::Knight,
            Kind::Rook,
        ];

        let mut init_pieces = [None; 64];

        // Initial position
        for i in 0..8 {
            init_pieces[i] = make_piece(Color::White, first_rank[i]);
            init_pieces[i + 7 * 8] = make_piece(Color::Black, first_rank[i]);
            init_pieces[i + 8] = make_piece(Color::White, Kind::Pawn);
            init_pieces[i + 6 * 8] = make_piece(Color::Black, Kind::Pawn);
        }

        let board: Board = Board {
            pieces: init_pieces,
            side_to_play: Color::White,
        };

        return board;
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // Loop over pieces (TODO: Optimize this !)
        for (square, piece) in self.pieces.iter().enumerate() {
            // If we indeed have a piece at that position
            if let Some(p) = piece {
                // Test if current piece if of color to play
                if p.color == self.side_to_play {
                    // Generate moves for each piece

                    match p.kind {
                        Kind::Pawn => {
                            let promotion_rank = if p.color == Color::White { 6 } else { 1 };
                            let start_rank = if p.color == Color::White { 1 } else { 6 };
                            let direction_offset: i32 =
                                if p.color == Color::White { 8 } else { -8 };

                            let square = square as i32;
                            let rank = square / 8;

                            // Let's see what is just if front of us
                            let target_square = (square + direction_offset) as usize;
                            if self.pieces[target_square].is_none() {
                                moves.push(Move::new(square, target_square as i32));
                            }

                            // Handle 2 square move on start rank
                            if rank == start_rank {
                                let target_square = (square + 2 * direction_offset) as usize;
                                if self.pieces[target_square].is_none() {
                                    moves.push(Move::new(square, target_square as i32));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        return moves;
    }

    pub fn display(&self) {
        println!(" ╭───┬───┬───┬───┬───┬───┬───┬───╮"); // Top border

        // we start displaying the last rank
        for rank in (0..8).rev() {
            print!("{}│", rank + 1); // Display rank number

            for file in 0..8 {
                let square = match self.pieces[file + 8 * rank] {
                    // If there is no piece here
                    None => " ",

                    // If it's some white piece
                    Some(Piece {
                        color: Color::White,
                        kind: k,
                    }) => match k {
                        Kind::Pawn => "♙",
                        Kind::Knight => "♘",
                        Kind::Bishop => "♗",
                        Kind::Rook => "♖",
                        Kind::King => "♔",
                        Kind::Queen => "♕",
                    },

                    // If it's some black piece
                    Some(Piece {
                        color: Color::Black,
                        kind: k,
                    }) => match k {
                        Kind::Pawn => "♟",
                        Kind::Knight => "♞",
                        Kind::Bishop => "♝",
                        Kind::Rook => "♜",
                        Kind::King => "♚",
                        Kind::Queen => "♛",
                    },
                };
                print!(" {} │", square);
            } // end iteration trhu files
            if rank > 0 {
                println!("\n ├───┼───┼───┼───┼───┼───┼───┼───┤");
            } else {
                println!(""); // line break
            }
        }

        println!(" ╰───┴───┴───┴───┴───┴───┴───┴───╯"); // Top border
        println!("   a   b   c   d   e   f   g   h");
    }
}
