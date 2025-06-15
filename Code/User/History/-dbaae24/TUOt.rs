
// Bitboard type
pub type bitboard = u64;

// Precomputed knight attacks for all 64 squares
pub static attacking: [bitboard; 64] = knight_attacks();

// Generate knight attacks for all squares at compile time
const fn knight_attacks() -> [bitboard; 64] {
    let mut table = [0u64; 64];
    let mut sq = 0;
    
    while sq < 64 {
        table[sq] = calculate_attacks(sq as u8);
        sq += 1;
    }

    table
}

// Calculate knight attacks for a single square
const fn calculate_attacks(square: u8) -> bitboard {
    let knight = 1u64 << square;

    const A_FILE: u64 = 0x0101010101010101;
    const B_FILE: u64 = 0x0202020202020202;
    const G_FILE: u64 = 0x4040404040404040;
    const H_FILE: u64 = 0x8080808080808080;

    let mut attacks = 0;

    // 2 up + 1 right
    attacks |= (knight & !H_FILE) << 17;
    // 2 up + 1 left
    attacks |= (knight & !A_FILE) << 15;
    // 2 down + 1 right
    attacks |= (knight & !H_FILE) >> 15;
    // 2 down + 1 left
    attacks |= (knight & !A_FILE) >> 17;
    // 2 right + 1 up
    attacks |= (knight & !(H_FILE | G_FILE)) << 10;
    // 2 right + 1 down
    attacks |= (knight & !(H_FILE | G_FILE)) >> 6;
    // 2 left + 1 up
    attacks |= (knight & !(A_FILE | B_FILE)) << 6;
    // 2 left + 1 down
    attacks |= (knight & !(A_FILE | B_FILE)) >> 10;

    attacks
}

// Generate knight moves from given knights bitboard, excluding friendlies
pub fn generate_moves(knights: bitboard, friendlies: bitboard) -> bitboard {
    let mut moves = 0;

    for square in 0..64 {
        if (knights & (u64 << square)) != 0 {
            moves |= attacking[square];
        }
    }

    moves & !friendlies
}

pub fn print_bitboard(bb: bitboard) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let sq = rank * 8 + file;
            let mask = 1u64 << sq;

            print!("{} ", if bb & mask != 0 {'1'} else {'.'});
        }
        println!();
    }
    println!();
}

