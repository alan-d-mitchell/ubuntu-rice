
// Bitboard type
pub type bitboard = u64;

// Precomputed knight attacks for all 64 squares
pub static attacks: [bitboard; 64] = knight_attacks();

// Generate knight attacks for all squares at compile time
const fn knight_attacks() -> [bitboard; 64] {
    let mut table = [0u64; 64]
}

