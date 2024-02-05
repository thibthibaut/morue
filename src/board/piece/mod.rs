use crate::board::Move;
use crate::Board;

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
}

pub fn make_piece(color: Color, kind: Kind) -> Option<Piece> {
    Some(Piece { color, kind })
}

impl Color {
    pub fn opposite(&self) -> Self {
        if *self == Color::Black {
            Color::White
        } else {
            Color::Black
        }
    }
}

pub fn generate_moves(p: &Piece, square: i32, board: &Board, moves: &mut Vec<Move>) {
    match p.kind {
        Kind::Pawn => generate_pawn_moves(p, square, board, moves),
        Kind::Knight => generate_knight_moves(p, square, board, moves),
        Kind::Rook => generate_rook_moves(p, square, board, moves),
        _ => (),
    }
}
fn generate_pawn_moves(p: &Piece, square: i32, board: &Board, moves: &mut Vec<Move>) {
    let promotion_rank = if p.color == Color::White { 6 } else { 1 };
    let start_rank = if p.color == Color::White { 1 } else { 6 };
    let direction_offset: i32 = if p.color == Color::White { 8 } else { -8 };

    let square = square as i32;
    let rank = square / 8;

    // Let's see what is just if front of us
    let target_square = square + direction_offset;
    // Break if we are on the last rank
    if target_square >= 64 {
        return;
    }

    // If there is no piece blocking the move
    if board.pieces[target_square as usize].is_none() {
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
            if board.pieces[target_square].is_none() {
                moves.push(Move::new(square, target_square as i32));
            }
        }
    }

    // Handle captures
    // Captures are +-1 from target square
    let opposite_color = board.turn.opposite();
    if (target_square % 8) != 0 {
        // check boundary
        let capture_square = target_square - 1;
        if let Some(p) = board.pieces[capture_square as usize] {
            if p.color == opposite_color {
                moves.push(Move::new(square, capture_square));
            }
        }
    }
    if (target_square % 8) != 7 {
        // check boundary
        let capture_square = target_square + 1;
        if let Some(p) = board.pieces[capture_square as usize] {
            if p.color == opposite_color {
                moves.push(Move::new(square, capture_square));
            }
        }
    }
    // End capture handling
}

fn generate_knight_moves(p: &Piece, square: i32, board: &Board, moves: &mut Vec<Move>) {
    // Table of possible knight moves according to file position
    let knight_moves: [&[i32]; 8] = [
        &[-6, -15, 10, 17],
        &[-6, -15, -17, 10, 15, 17],
        &[-6, -10, -15, -17, 6, 10, 15, 17],
        &[-6, -10, -15, -17, 6, 10, 15, 17],
        &[-6, -10, -15, -17, 6, 10, 15, 17],
        &[-6, -10, -15, -17, 6, 10, 15, 17],
        &[-10, -15, -17, 6, 15, 17],
        &[-10, -17, 6, 15],
    ];

    let file = square % 8;
    let current_moves = knight_moves[file as usize];

    for offset in current_moves.iter() {
        let target_square = square + offset;

        // Skip squares that are not on the board
        if target_square < 0 || target_square >= 64 {
            continue;
        }

        if let Some(target_piece) = board.pieces[target_square as usize] {
            // Skip if the piece is of the same color
            if target_piece.color == p.color {
                continue;
            }
            // Create a capture move
            moves.push(Move::new(square, target_square));
        } else {
            // The square is free, the knight can go there
            moves.push(Move::new(square, target_square));
        }
    }
}

fn explore_direction(p: &Piece, square: i32, board: &Board, moves: &mut Vec<Move>, step: i32) {
    let mut target_square = square + step;
    let mut previous_file = square % 8;

    while target_square >= 0 && target_square < 64 {
        // Stop when we wrap-around the files
        if (previous_file == 0 && (target_square % 8) == 7)
            || (previous_file == 7 && (target_square % 8) == 0)
        {
            break;
        }
        previous_file = target_square % 8;

        if let Some(target_piece) = board.pieces[target_square as usize] {
            if target_piece.color != p.color {
                // Create a capture move
                moves.push(Move::new(square, target_square));
                break;
            } else {
                // We hit a friend piece we need to stop here
                break;
            }
        } else {
            // The square is free we can go
            moves.push(Move::new(square, target_square));
        }
        target_square += step;
    }
}

fn generate_rook_moves(p: &Piece, square: i32, board: &Board, moves: &mut Vec<Move>) {
    // Exploring upwards (+8)
    explore_direction(p, square, board, moves, 8);
    // Exploring downwards (-8)
    explore_direction(p, square, board, moves, -8);
    // Exploring left (-1)
    explore_direction(p, square, board, moves, -1);
    // Exploring right (+1)
    explore_direction(p, square, board, moves, 1);
}
