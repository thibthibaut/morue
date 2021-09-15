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
