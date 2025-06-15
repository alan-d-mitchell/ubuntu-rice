
// Bitboard
pub type bitboard = u64;

// Precomputed pawn attacks, minus en passant
pub static WHITE_ATTACKING: [bitboard; 64] = pawn_attacks(true); // true for white
pub static BLACK_ATTACKING: [bitboard; 64] = pawn_attacks(false); // false for black

const fn pawn_attacks(is_white: bool) -> [bitboard; 64] {
    let mut table = [0u64; 64];
    let mut sq = 0;

    while sq < 64 {
        table[sq] = calculate_attacks(sq as u8, is_white);
        sq += 1;
    }
}