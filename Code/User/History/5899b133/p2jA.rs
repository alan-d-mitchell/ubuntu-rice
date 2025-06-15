use lazy_static::lazy_static;
use std::collections::HashMap;

pub type Bitboard = u64;

const BOARD_SIZE: usize = 64;

// Directions for rook sliding: N, E, S, W
const ROOK_DIRECTIONS: [i32; 4] = [8, 1, -8, -1];

// Directions for bishop sliding: NE, NW, SE, SW
const BISHOP_DIRECTIONS: [i32; 4] = [9, 7, -9, -7];

pub const ROOK_MAGICS: [u64; 64] = [
    0x8a80104000800020, 0x140002000100040, 0x2801880a0017001, 0x100081001000420,
    0x200020010080420, 0x3001c0002010008, 0x8480008002000100, 0x2080088004402900,
    0x800098204000, 0x2024401000200040, 0x100802000801000, 0x120800800801000,
    0x208808088000400, 0x2802200800400, 0x2200800100020080, 0x801000060821100,
    0x80044006422000, 0x100808020004000, 0x12108a0010204200, 0x140848010000802,
    0x481828014002800, 0x8094004002004100, 0x4010040010010802, 0x20008806104,
    0x100400080208000, 0x2040002120081000, 0x21200680100081, 0x20100080080080,
    0x2000a00200410, 0x20080800400, 0x80088400100102, 0x80004600042881,
    0x4040008040800020, 0x440003000200801, 0x4200011004500, 0x188020010100100,
    0x14800401802800, 0x2080040080800200, 0x124080204001001, 0x200046502000484,
    0x480400080088020, 0x1000422010034000, 0x30200100110040, 0x100021010009,
    0x2002080100110004, 0x202008004008002, 0x20020004010100, 0x2048440040820001,
    0x101002200408200, 0x40802000401080, 0x4008142004410100, 0x2060820c0120200,
    0x1001004080100, 0x20c020080040080, 0x2935610830022400, 0x44440041009200,
    0x280001040802101, 0x2100190040002085, 0x80c0084100102001, 0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402,
];

pub const BISHOP_MAGICS: [u64; 64] = [
    0x40040844404084, 0x2004208a004208, 0x10190041080202, 0x108060845042010,
    0x581104180800210, 0x2112080446200010, 0x1080820820060210, 0x3c0808410220200,
    0x4050404440404, 0x21001420088, 0x24d0080801082102, 0x1020a0a020400,
    0x40308200402, 0x4011002100800, 0x401484104104005, 0x801010402020200,
    0x400210c3880100, 0x404022024108200, 0x810018200204102, 0x4002801a02003,
    0x85040820080400, 0x810102c808880400, 0xe900410884800, 0x8002020480840102,
    0x220200865090201, 0x2010100a02021202, 0x152048408022401, 0x20080002081110,
    0x4001001021004000, 0x800040400a011002, 0xe4004081011002, 0x1c004001012080,
    0x8004200962a00220, 0x8422100208500202, 0x2000402200300c08, 0x8646020080080080,
    0x80020a0200100808, 0x2010004880111000, 0x623000a080011400, 0x42008c0340209202,
    0x209188240001000, 0x400408a884001800, 0x110400a6080400, 0x1840060a44020800,
    0x90080104000041, 0x201011000808101, 0x1a2208080504f080, 0x8012020600211212,
    0x500861011240000, 0x180806108200800, 0x4000020e01040044, 0x300000261044000a,
    0x802241102020002, 0x20906061210001, 0x5a84841004010310, 0x4010801011c04,
    0xa010109502200, 0x4a02012000, 0x500201010098b028, 0x8040002811040900,
    0x28000010020204, 0x6000020202d0240, 0x8918844842082200, 0x4010011029020020,
];

// Relevant bits for rook occupancy mask per square
pub const ROOK_RELEVANT_BITS: [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

// Relevant bits for bishop occupancy mask per square
pub const BISHOP_RELEVANT_BITS: [u32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
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

pub struct Magic {
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