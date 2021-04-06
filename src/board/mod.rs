mod moves;
mod piece;
use moves::*;
use piece::*;

struct CastlingRights {
    long_side: bool,
    short_side: bool,
}

pub struct Board {
    // The board is 64 squares containing a piece or not
    pieces: [Option<Piece>; 64],

    // Store positions of whites pieces and black pieces for fast lookup
    // whitePiecesPos: Vec<i32>,
    // blackPiecesPos: Vec<i32>,
    side_to_play: Color,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
    en_passant: Option<i32>,
}

impl Board {
    // pub fn new() -> Self {
    //     let first_rank = [
    //         Kind::Rook,
    //         Kind::Knight,
    //         Kind::Bishop,
    //         Kind::Queen,
    //         Kind::King,
    //         Kind::Bishop,
    //         Kind::Knight,
    //         Kind::Rook,
    //     ];

    //     let mut init_pieces = [None; 64];

    //     // Initial position
    //     for i in 0..8 {
    //         init_pieces[i] = make_piece(Color::White, first_rank[i]);
    //         init_pieces[i + 7 * 8] = make_piece(Color::Black, first_rank[i]);
    //         init_pieces[i + 8] = make_piece(Color::White, Kind::Pawn);
    //         init_pieces[i + 6 * 8] = make_piece(Color::Black, Kind::Pawn);
    //     }

    //     let board: Board = Board {
    //         pieces: init_pieces,
    //         side_to_play: Color::White,
    //     };

    //     return board;
    // }

    pub fn from_fen(fen_string: &String) -> Self {
        let splits: Vec<&str> = fen_string.split(' ').collect();
        //TODO: Test len of splits

        let ranks: Vec<&str> = splits[0].split('/').collect();
        //TODO: Test len of ranks

        let mut pieces = [None; 64];

        for (rank, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - rank;
            let mut file: usize = 0;
            for c in rank_str.chars() {
                let piece = match c {
                    'p' => make_piece(Color::Black, Kind::Pawn),
                    'n' => make_piece(Color::Black, Kind::Knight),
                    'b' => make_piece(Color::Black, Kind::Bishop),
                    'r' => make_piece(Color::Black, Kind::Rook),
                    'k' => make_piece(Color::Black, Kind::King),
                    'q' => make_piece(Color::Black, Kind::Queen),
                    'P' => make_piece(Color::White, Kind::Pawn),
                    'N' => make_piece(Color::White, Kind::Knight),
                    'B' => make_piece(Color::White, Kind::Bishop),
                    'R' => make_piece(Color::White, Kind::Rook),
                    'K' => make_piece(Color::White, Kind::King),
                    'Q' => make_piece(Color::White, Kind::Queen),
                    x => {
                        let num_blanks = x.to_digit(10).unwrap();
                        file += (num_blanks - 1) as usize;
                        None
                    }
                };
                pieces[rank * 8 + file] = piece;
                file += 1;
            }
        }

        // Parse side to play
        let side_to_play = match splits[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => Color::White,
        };

        // Parse castling availability
        let mut white_castling_rights = CastlingRights {
            long_side: false,
            short_side: false,
        };
        let mut black_castling_rights = CastlingRights {
            long_side: false,
            short_side: false,
        };

        for castle_char in splits[2].chars() {
            // println!("{}", castle_char);
            match castle_char {
                'K' => white_castling_rights.short_side = true,
                'Q' => white_castling_rights.long_side = true,
                'k' => black_castling_rights.short_side = true,
                'q' => black_castling_rights.long_side = true,
                _ => {}
            }
        }

        // Parse en passant possibility
        let en_passant: Option<i32> = match splits[3] {
            "_" => None,
            x => {
                println!("{}", x);
                Some(square_from_algebric(&String::from(x)).unwrap())
                // x.parse().unwrap_or(None);
            }
        };

        println!("En passant {:?}", en_passant);
        Board {
            pieces,
            side_to_play,
            white_castling_rights,
            black_castling_rights,
            en_passant,
        }
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

                            // If there is no piece blocking the move
                            if self.pieces[target_square].is_none() {
                                let target_square = target_square as i32;
                                // If we're on the last rank
                                if rank == promotion_rank {
                                    moves.push(Move::new_with_promotion(
                                        square,
                                        target_square,
                                        Some(Kind::Queen),
                                    ));
                                    moves.push(Move::new_with_promotion(
                                        square,
                                        target_square,
                                        Some(Kind::Rook),
                                    ));
                                    moves.push(Move::new_with_promotion(
                                        square,
                                        target_square,
                                        Some(Kind::Bishop),
                                    ));
                                    moves.push(Move::new_with_promotion(
                                        square,
                                        target_square,
                                        Some(Kind::Knight),
                                    ));
                                } else {
                                    // Normal case
                                    moves.push(Move::new(square, target_square));
                                }

                                // Handle 2 square move on start rank
                                if rank == start_rank {
                                    let target_square = (square + 2 * direction_offset) as usize;
                                    if self.pieces[target_square].is_none() {
                                        moves.push(Move::new(square, target_square as i32));
                                    }
                                }
                            }
                        } // End of pawn move
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

fn square_from_algebric(algebric: &String) -> Result<i32, &'static str> {
    // TODO: Refactor this
    if algebric.len() != 2 {
        return Err("Algebric notation does not contain 2 characters");
    }

    let mut square: i32 = 0;
    let values: Vec<char> = algebric.chars().collect();
    let file = values[0].to_digit(18);

    if let Some(f) = file {
        if f < 10 {
            return Err("File falue is invalid");
        }
        square = (f as i32) - 10;
    } else {
        return Err("File falue is invalid");
    }

    let rank = values[1].to_digit(10);

    match rank {
        None => Err("Rank falue is invalid"),
        Some(r) => Ok(((r * 8) as i32) + square),
    }
}
