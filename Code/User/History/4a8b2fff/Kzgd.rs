pub struct GameState {
    pub captured_piece: Option<(Piece, Color)>,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl GameState {
    pub fn save(&mut self, board: &Board, mov: Move) { /* ... */ }
    pub fn restore_state(&self, board: &mut Board) { /* ... */ }
}
