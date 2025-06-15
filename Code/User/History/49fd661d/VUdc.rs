pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
    pub promotion: Option<Piece>,
    pub is_castling: bool,
    pub is_en_passant: bool,
}