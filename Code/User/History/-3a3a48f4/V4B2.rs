
// Bitboard
pub type bitboard = u64;

// Generates all possible bishop moves in a given position
// Returns a bitboard of these moves
pub fn bishop_moves(bishop_bb: bitboard, occupied: bitboard) -> bitboard {
    let mut moves = 0;
    let mut bishops = bishop_bb;

    while bishops != 0 {
        let square = bishops.trailing_zeros() as u8;
        moves |= bishop_attacks(square, occupied);
    }

    moves
}

fn bishop_attacks(square: u8, occupied: bitboard) -> bitboard {
    let mut attacks = 0;
    let directions = [9, 7, -9, -7];

    for &dir in &directions {
        let mut sq = square as i32;

        loop {
            sq += dir;

            // Check boundaries: make sure sq stays on the board and within the same diagonal file/rank limits
            if sq < 0 || sq >= 64 || !on_same_diagonal(square as i32, sq, dir){
                break;
            }

            attacks |= 1u64 << sq;

            // Stop if piece blocking
            if (occupied & (1u64 << sq)) != 0 {
                break;
            }
        }
    }

    attacks
}

// Check if sq2 is on the same diag as s1 for sliding (to prevent wrap around)
fn on_same_diagonal(sq1: i32, sq2: i32, dir: i32) -> bool {
    let file1 = sq1 % 8;
    let file2 = sqr2 % 8;

    match dir {
        9 => file2 > file1,    // NE (file must increase)
        7 => file2 < file1,    // NW (file must decrease)
        -7 => file2 > file1,   // SE (file must increase)
        -9 => file2 < file1,   // SW (file must decrease)
        _ => false,
    }
}