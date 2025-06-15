pub type bitboard = u64;

/// Precomputed king attack bitboards for each square (0..63).
/// King moves to all adjacent squares (max 8).
pub static KING_ATTACKS: [bitboard; 64] = generate_king_attacks();

const fn generate_king_attacks() -> [bitboard; 64] {
    let mut table = [0u64; 64];
    let mut sq = 0;

    while sq < 64 {
        table[sq] = calculate_attacks(sq as u8);
        sq += 1;
    }
    
    table
}

const fn calculate_attacks(square: u8) -> bitboard {
    let king = 1u64 << square;

    const NOT_A_FILE: u64 = 0xfefefefefefefefe;
    const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

    // Shift in all 8 directions:
    let mut attacks = 0;

    // North
    attacks |= king << 8;
    // South
    attacks |= king >> 8;

    // East
    attacks |= (king & NOT_H_FILE) << 1;
    // West
    attacks |= (king & NOT_A_FILE) >> 1;

    // Northeast
    attacks |= (king & NOT_H_FILE) << 9;
    // Northwest
    attacks |= (king & NOT_A_FILE) << 7;

    // Southeast
    attacks |= (king & NOT_H_FILE) >> 7;
    // Southwest
    attacks |= (king & NOT_A_FILE) >> 9;

    attacks
}

/// Generates all possible king moves given the king bitboard and occupied friendly squares.
/// Returns bitboard of legal moves (excluding captures if friendlies occupied).
pub fn generate_moves(king_bb: bitboard, friendlies: bitboard) -> bitboard {
    let mut moves = 0;
    let mut kings = king_bb;

    while kings != 0 {
        let sq = kings.trailing_zeros() as usize;
        moves |= KING_ATTACKS[sq];
        kings &= kings - 1;
    }

    // Remove squares occupied by friendlies
    moves & !friendlies
}