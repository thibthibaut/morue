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
    turn: Color,
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
    //         turn: Color::White,
    //     };

    //     return board;
    // }

    // TODO: Why not using From::String implementation to use casting
    pub fn from_fen(fen_string: &String) -> Result<Self, &'static str> {
        let splits: Vec<&str> = fen_string.split(' ').collect();

        if splits.len() != 6 {
            return Err("Invalid FEN string");
        }

        let ranks: Vec<&str> = splits[0].split('/').collect();

        if ranks.len() != 8 {
            return Err("Invalid FEN string");
        }

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
        let turn = match splits[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid FEN string: invalid active color, expecting w or b"),
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
                '-' => {}
                _ => return Err("Invalid FEN string: invalid catling rights"),
            }
        }

        // Parse en passant possibility
        let en_passant: Option<i32> = match splits[3] {
            "-" => None,
            x => {
                match square_from_algebric(&String::from(x)) {
                    Ok(x) => Some(x),
                    Err(_) => return Err("Invalid FEN string: invalid en passant string"),
                }
                // x.parse().unwrap_or(None);
            }
        };

        Ok(Board {
            pieces,
            turn,
            white_castling_rights,
            black_castling_rights,
            en_passant,
        })
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // Loop over pieces (TODO: Optimize this !)
        for (square, piece) in self.pieces.iter().enumerate() {
            if let Some(p) = piece {
                if p.color == self.turn {
                    generate_moves(p, square as i32, self, &mut moves);
                }
            }
        } // End loop over pieces

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
                if square == " " {
                    print!("{:^3}│", file + 8 * rank);
                    //print!("   |");
                } else {
                    print!(" {} │", square);
                }
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

    let mut square: i32;
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
