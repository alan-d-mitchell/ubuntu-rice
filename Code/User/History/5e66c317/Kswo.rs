// Bitboard type
pub type Bitboard = u64;

// Constants for file masks
const A_FILE: Bitboard = 0x0101010101010101;
const H_FILE: Bitboard = 0x8080808080808080;
const RANK_4: Bitboard = 0x00000000FF000000;
const RANK_5: Bitboard = 0x000000FF00000000;
const RANK_2: Bitboard = 0x000000000000FF00;
const RANK_7: Bitboard = 0x00FF000000000000;

// Precomputed pawn attacks for captures (not pushes)
pub static WHITE_ATTACKING: [Bitboard; 64] = pawn_attacks(true);
pub static BLACK_ATTACKING: [Bitboard; 64] = pawn_attacks(false);

const fn pawn_attacks(is_white: bool) -> [Bitboard; 64] {
    let mut table = [0u64; 64];
    let mut sq = 0;

    while sq < 64 {
        table[sq] = calculate_attacks(sq as u8, is_white);
        sq += 1;
    }
    table
}

const fn calculate_attacks(square: u8, is_white: bool) -> Bitboard {
    let pawn = 1u64 << square;
    let mut attacks = 0;

    if is_white {
        if pawn & !A_FILE != 0 {
            attacks |= pawn << 7;
        }
        if pawn & !H_FILE != 0 {
            attacks |= pawn << 9;
        }
    } else {
        if pawn & !A_FILE != 0 {
            attacks |= pawn >> 9;
        }
        if pawn & !H_FILE != 0 {
            attacks |= pawn >> 7;
        }
    }

    attacks
}

// Runtime pawn move generation (non-captures and captures)
pub fn generate_pawn_moves(pawns: Bitboard, friendlies: Bitboard, enemies: Bitboard, is_white: bool) -> Bitboard {
    let mut moves = 0;

    if is_white {
        // 1 square forward
        let single_push = (pawns << 8) & !(friendlies | enemies);
        // 2 square forward (only from rank 2)
        let double_push = ((single_push & RANK_3()) << 8) & !(friendlies | enemies);
        // Captures
        let left_attacks = (pawns & !A_FILE) << 7 & enemies;
        let right_attacks = (pawns & !H_FILE) << 9 & enemies;

        moves = single_push | double_push | left_attacks | right_attacks;
    } else {
        // 1 square forward
        let single_push = (pawns >> 8) & !(friendlies | enemies);
        // 2 square forward (only from rank 7)
        let double_push = ((single_push & RANK_6()) >> 8) & !(friendlies | enemies);
        // Captures
        let left_attacks = (pawns & !A_FILE) >> 9 & enemies;
        let right_attacks = (pawns & !H_FILE) >> 7 & enemies;

        moves = single_push | double_push | left_attacks | right_attacks;
    }

    moves
}

// Needed for const math in double push logic
const fn RANK_3() -> Bitboard { 0x0000000000FF0000 }
const fn RANK_6() -> Bitboard { 0x0000FF0000000000 }

// Debug bitboard print
pub fn print_bitboard(bb: Bitboard) {
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
