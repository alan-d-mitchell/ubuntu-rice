use lazy_static::lazy_static;
use std::collections::HashMap;

pub type Bitboard = u64;

const BOARD_SIZE: usize = 64;

// Directions for rook sliding: N, E, S, W
const ROOK_DIRECTIONS: [i32; 4] = [8, 1, -8, -1];

// Directions for bishop sliding: NE, NW, SE, SW
const BISHOP_DIRECTIONS: [i32; 4] = [9, 7, -9, -7];

// Predefined magic numbers for rook and bishop per square (these are "good" magic numbers)
// In real engines, these are carefully generated.
// Here, just placeholders — you'd replace with known good magic numbers.
const ROOK_MAGICS: [u64; 64] = [
    0xA8002C000108020, 0x6C00049B0002001, /* ... fill all 64 ... */ 0x8040008000000
];
const BISHOP_MAGICS: [u64; 64] = [
    0x40040844404084, 0x2004208A004208, /* ... fill all 64 ... */ 0x202040100020
];

// Relevant occupancy bits for rook and bishop per square
const ROOK_RELEVANT_BITS: [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, /* ... fill all 64 ... */
];
const BISHOP_RELEVANT_BITS: [u32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, /* ... fill all 64 ... */
];

// Utility: check if square is on board and not wrapped around files
fn on_board(sq: i32) -> bool {
    sq >= 0 && sq < 64
}
fn file_of(sq: i32) -> i32 {
    sq % 8
}
fn rank_of(sq: i32) -> i32 {
    sq / 8
}

// Generate occupancy mask for sliding piece on square (rook or bishop)
fn mask_rook_attacks(square: usize) -> Bitboard {
    let mut attacks = 0u64;
    let rank = rank_of(square as i32);
    let file = file_of(square as i32);

    // Rook moves along ranks and files excluding edges
    // (Exclude outer edge squares to allow blockers)
    // Up
    for r in rank+1..7 {
        attacks |= 1u64 << (file + r*8);
    }
    // Down
    for r in (1..rank).rev() {
        attacks |= 1u64 << (file + r*8);
    }
    // Right
    for f in file+1..7 {
        attacks |= 1u64 << (f + rank*8);
    }
    // Left
    for f in (1..file).rev() {
        attacks |= 1u64 << (f + rank*8);
    }

    attacks
}

fn mask_bishop_attacks(square: usize) -> Bitboard {
    let mut attacks = 0u64;
    let rank = rank_of(square as i32);
    let file = file_of(square as i32);

    // Bishop moves diagonally excluding edges
    // NE
    let mut r = rank + 1;
    let mut f = file + 1;
    while r < 7 && f < 7 {
        attacks |= 1u64 << (f + r*8);
        r += 1;
        f += 1;
    }
    // NW
    r = rank + 1;
    f = file - 1;
    while r < 7 && f > 0 {
        attacks |= 1u64 << (f + r*8);
        r += 1;
        f -= 1;
    }
    // SE
    r = rank - 1;
    f = file + 1;
    while r > 0 && f < 7 {
        attacks |= 1u64 << (f + r*8);
        r -= 1;
        f += 1;
    }
    // SW
    r = rank - 1;
    f = file - 1;
    while r > 0 && f > 0 {
        attacks |= 1u64 << (f + r*8);
        r -= 1;
        f -= 1;
    }

    attacks
}

// Generate all blocker boards for mask bits (used for indexing attack tables)
fn generate_blocker_boards(mask: Bitboard) -> Vec<Bitboard> {
    let bits = mask.count_ones();
    let blockers_count = 1 << bits;
    let mut blockers = Vec::with_capacity(blockers_count as usize);

    for index in 0..blockers_count {
        let mut blocker = 0u64;
        let mut bits_set = 0;
        for i in 0..64 {
            if (mask & (1u64 << i)) != 0 {
                if (index & (1 << bits_set)) != 0 {
                    blocker |= 1u64 << i;
                }
                bits_set += 1;
            }
        }
        blockers.push(blocker);
    }
    blockers
}

// Calculate rook attacks for square with blockers present
fn rook_attacks_on_the_fly(square: usize, blockers: Bitboard) -> Bitboard {
    let mut attacks = 0u64;
    let rank = rank_of(square as i32);
    let file = file_of(square as i32);

    // Up
    for r in rank+1..8 {
        let sq = file + r*8;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
    }
    // Down
    for r in (0..rank).rev() {
        let sq = file + r*8;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
    }
    // Right
    for f in file+1..8 {
        let sq = f + rank*8;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
    }
    // Left
    for f in (0..file).rev() {
        let sq = f + rank*8;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
    }
    attacks
}

// Calculate bishop attacks for square with blockers present
fn bishop_attacks_on_the_fly(square: usize, blockers: Bitboard) -> Bitboard {
    let mut attacks = 0u64;
    let rank = rank_of(square as i32);
    let file = file_of(square as i32);

    // NE
    let mut r = rank + 1;
    let mut f = file + 1;
    while r < 8 && f < 8 {
        let sq = f + r*8;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
        r += 1;
        f += 1;
    }
    // NW
    r = rank + 1;
    f = file.checked_sub(1).unwrap_or(8); // use 8 if underflow
    while r < 8 && f < 8 {
        let sq = f + r*8;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
        r += 1;
        f = f.checked_sub(1).unwrap_or(8);
    }
    // SE
    r = rank.checked_sub(1).unwrap_or(8);
    f = file + 1;
    while r < 8 && f < 8 {
        let sq = f + r*8;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
        r = r.checked_sub(1).unwrap_or(8);
        f += 1;
    }
    // SW
    r = rank.checked_sub(1).unwrap_or(8);
    f = file.checked_sub(1).unwrap_or(8);
    while r < 8 && f < 8 {
        let sq = f + r*8;
        attacks |= 1u64 << sq;
        if blockers & (1u64 << sq) != 0 {
            break;
        }
        r = r.checked_sub(1).unwrap_or(8);
        f = f.checked_sub(1).unwrap_or(8);
    }

    attacks
}

struct Magic {
    mask: Bitboard,
    magic: u64,
    shift: u32,
    attack_table: Vec<Bitboard>,
}

impl Magic {
    fn new(square: usize, is_rook: bool) -> Magic {
        let mask = if is_rook {
            mask_rook_attacks(square)
        } else {
            mask_bishop_attacks(square)
        };

        let relevant_bits = if is_rook {
            ROOK_RELEVANT_BITS[square]
        } else {
            BISHOP_RELEVANT_BITS[square]
        };

        let magic = if is_rook {
            ROOK_MAGICS[square]
        } else {
            BISHOP_MAGICS[square]
        };

        let blocker_boards = generate_blocker_boards(mask);
        let mut attack_table = Vec::with_capacity(1 << relevant_bits);

        for blockers in blocker_boards {
            let attack = if is_rook {
                rook_attacks_on_the_fly(square, blockers)
            } else {
                bishop_attacks_on_the_fly(square, blockers)
            };
            attack_table.push(attack);
        }

        Magic {
            mask,
            magic,
            shift: 64 - relevant_bits,
            attack_table,
        }
    }

    fn get_attacks(&self, blockers: Bitboard) -> Bitboard {
        let blockers_masked = blockers & self.mask;
        let index = ((blockers_masked.wrapping_mul(self.magic)) >> self.shift) as usize;
        self.attack_table[index]
    }
}

lazy_static! {
    // Create magic tables for rook and bishop per square
    pub static ref ROOK_MAGICS_TABLE: Vec<Magic> = (0..64).map(|sq| Magic::new(sq, true)).collect();
    pub static ref BISHOP_MAGICS_TABLE: Vec<Magic> = (0..64).map(|sq| Magic::new(sq, false)).collect();
}

// Main public function to get queen moves for a bitboard of queens with blockers on board
pub fn queen_moves(queen_bb: Bitboard, blockers: Bitboard) -> Bitboard {
    let mut moves = 0u64;
    let mut queens = queen_bb;

    while queens != 0 {
        let square = queens.trailing_zeros() as usize;

        let rook_attacks = ROOK_MAGICS_TABLE[square].get_attacks(blockers);
        let bishop_attacks = BISHOP_MAGICS_TABLE[square].get_attacks(blockers);

        moves |= rook_attacks | bishop_attacks;

        queens &= queens - 1;
    }

    moves
}