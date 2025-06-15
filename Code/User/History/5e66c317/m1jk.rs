
// Bitboard
pub type bitboard = u64;

// Precomputed pawn attacks, minus en passant
pub static ATTACKING: [bitboard; 64] = pawn_attacks();