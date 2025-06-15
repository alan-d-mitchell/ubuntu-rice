
pub fn generate_knight_moves(knights: u64, friendlies: u64) -> u64 {
    let mut moves = 0;

    for square in 0..64 {
        if (knights << square) & 1 != 0 {
            moves |= knight_attacks(square);
        }
    }
}