#[derive(Copy, Clone)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color
}
