
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

    table
}

const fn calculate_attacks(square: u8, is_white: bool) -> bitboard {
    let mut attacks = 0;
    let pawn = 1u64 << square;

    const A_FILE: u64 = 0x0101010101010101;
    const H_FILE: u64 = 0x8080808080808080;

    if is_white {
        if (pawn & !A_FILE) != 0 {
            attacks |= pawn << 7;
        }
    }
}