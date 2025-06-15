
pub fn generate_knight_moves(knights: u64, friendlies: u64) -> u64 {
    let mut moves = 0;

    for square in 0..64 {
        if (knights << square) & 1 != 0 {
            moves |= knight_attacks(square);
        }
    }

    moves & !friendlies
}

fn knight_attacks(square: u8) -> u64 {
    let knight = 1u64 << square;

    const A_FILE: u64 = 0x0101010101010101;
    const B_FILE: u64 = 0x0202020202020202;
    const G_FILE: u64 = 0x4040404040404040;
    const H_FILE: u64 = 0x8080808080808080;

    let mut attacks = 0;

    // 2 up + 1 right
    attacks |= (knight << 17) & !H_FILE;
    // 2 up + 1 left
    attacks |= (knight << 15) & !A_FILE;
    // 2 down + 1 right
    attacks |= (knight >> 15) & !H_FILE;
    // 2 down + 1 left
    attacks |= (knight >> 17) & !A_FILE;
    // 2 right + 1 up
    attacks |= (knight << 10) & !(H_FILE | G_FILE);
    // 2 right + 1 down
    attacks |= (knight >> 6) & !(H_FILE | G_FILE);
    // 2 left + 1 up
    attacks |= (knight << 6) & !(A_FILE | B_FILE);
    // 2 left + 1 down
    attacks |= (knight >> 10) & !(A_FILE | B_FILE);
}

pub fn knight_bitboard_test(bb: u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let sqaure = rank * 8 + file;
            let mask = 1u64 << square;
            let ch = if bb & mask != 0 {'1'} else {'.'};

            print!("{} ", ch);
        }
        println!();
    }
    println!();
}