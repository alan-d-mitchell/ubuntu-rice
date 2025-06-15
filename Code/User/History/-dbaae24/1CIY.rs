
// Bitboard type
pub type bitboard = u64;

// Precomputed knight attacks for all 64 squares
pub static attacks: [bitboard; 64] = knight_attacks();

// Generate knight attacks for all squares at compile time
const fn knight_attacks() -> [bitboard; 64] {
    let mut table = [0u64; 64];
    let mut sq = 0;
    
    while sq < 64 {
        table[sq] = calculate_attacks(sq as u8);
        sq += 1;
    }
}

