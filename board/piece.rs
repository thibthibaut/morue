#[derive(Copy, Clone)]
pub enum PieceKind {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Copy, Clone)]
pub enum PieceColor {
    None,
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor
}
