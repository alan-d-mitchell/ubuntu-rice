
// Bitboard
pub type bitboard = u64;

// Generates all possible bishop moves in a given position
// Returns a bitboard of these moves
pub fn bishop_moves(bishop_bb: bitboard, occupied: bitboard) -> bitboard {
    let mut moves = 0;

    for square in 0..64 {
        if (bishop_bb & (1u64 << square)) != 0 {
            moves |= bishop_attacks(square, occupied);
        }
    }

    moves
}