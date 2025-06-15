
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