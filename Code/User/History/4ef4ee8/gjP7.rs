pub type bitboard = u64;

const BOARD_SIZE: usize = 64;

// Predefined rook magic numbers (known from chess programming resources)
const ROOK_MAGICS: [u64; BOARD_SIZE] = [
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
    0x12001008414402, 0x2006104900a0804, 0x1004081002402,
];

// Relevant rook blocker mask bits count for each square
const ROOK_RELEVANT_BITS: [u8; BOARD_SIZE] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

// Mask for rook moves - excludes edges, because those are not blockers affecting sliding attacks
fn rook_mask(square: usize) -> bitboard {
    let mut mask = 0u64;
    let rank = square / 8;
    let file = square % 8;

    // Vertical (up)
    for r in (rank + 1)..7 {
        mask |= 1u64 << (file + r * 8);
    }
    // Vertical (down)
    for r in 1..rank {
        mask |= 1u64 << (file + (rank - r) * 8);
    }
    // Horizontal (right)
    for f in (file + 1)..7 {
        mask |= 1u64 << (f + rank * 8);
    }
    // Horizontal (left)
    for f in 1..file {
        mask |= 1u64 << (file - f + rank * 8);
    }

    mask
}

// Generate all blocker variations for a given mask (for indexing attacks)
fn generate_blocker_variations(mask: bitboard) -> Vec<bitboard> {
    let bits_count = mask.count_ones();
    let variations_count = 1 << bits_count;
    let mut variations = Vec::with_capacity(variations_count as usize);

    for i in 0..variations_count {
        let mut blocker = 0;
        let mut bit_index = 0;
        for bit in 0..64 {
            if (mask & (1u64 << bit)) != 0 {
                if (i & (1 << bit_index)) != 0 {
                    blocker |= 1u64 << bit;
                }
                bit_index += 1;
            }
        }
        variations.push(blocker);
    }

    variations
}

// Calculate rook attacks for a square with blockers on board
fn rook_attack_on_the_fly(square: usize, blockers: bitboard) -> bitboard {
    let mut attacks = 0u64;
    let rank = square / 8;
    let file = square % 8;

    // Up
    for r in (rank + 1)..8 {
        let sq = file + r * 8;
        attacks |= 1u64 << sq;
        if (blockers & (1u64 << sq)) != 0 {
            break;
        }
    }
    // Down
    for r in (0..rank).rev() {
        let sq = file + r * 8;
        attacks |= 1u64 << sq;
        if (blockers & (1u64 << sq)) != 0 {
            break;
        }
    }
    // Right
    for f in (file + 1)..8 {
        let sq = f + rank * 8;
        attacks |= 1u64 << sq;
        if (blockers & (1u64 << sq)) != 0 {
            break;
        }
    }
    // Left
    for f in (0..file).rev() {
        let sq = f + rank * 8;
        attacks |= 1u64 << sq;
        if (blockers & (1u64 << sq)) != 0 {
            break;
        }
    }

    attacks
}

// Struct to hold rook magic data for each square
pub struct RookMagic {
    pub mask: bitboard,
    pub magic: u64,
    pub shift: u8,
    pub attacks: Vec<bitboard>,
}

impl RookMagic {
    pub fn new(square: usize) -> Self {
        let mask = rook_mask(square);
        let magic = ROOK_MAGICS[square];
        let relevant_bits = ROOK_RELEVANT_BITS[square];
        let shift = 64 - relevant_bits;

        // Generate all blocker variations
        let blockers = generate_blocker_variations(mask);

        // Generate attack table for each blocker variation
        let mut attacks = Vec::with_capacity(blockers.len());
        for blocker in &blockers {
            attacks.push(rook_attack_on_the_fly(square, *blocker));
        }

        Self {
            mask,
            magic,
            shift,
            attacks,
        }
    }

    // Given the current blockers on board, calculate the rook attacks for this square
    pub fn attacks(&self, blockers: bitboard) -> bitboard {
        // Extract the blockers relevant to this rook square's mask
        let relevant_blockers = blockers & self.mask;

        // Calculate magic index
        let index = ((relevant_blockers.wrapping_mul(self.magic)) >> self.shift) as usize;

        self.attacks[index]
    }
}

// Precompute all rook magic tables for all squares
lazy_static::lazy_static! {
    pub static ref ROOK_MAGICS_TABLE: Vec<RookMagic> = {
        let mut table = Vec::with_capacity(64);
        for sq in 0..64 {
            table.push(RookMagic::new(sq));
        }
        table
    };
}

// Generate rook moves for all rooks on the board
pub fn rook_moves(rook_bb: bitboard, occupied: bitboard) -> bitboard {
    let mut moves = 0u64;
    let mut rooks = rook_bb;

    while rooks != 0 {
        let sq = rooks.trailing_zeros() as usize;
        moves |= ROOK_MAGICS_TABLE[sq].attacks(occupied);
        rooks &= rooks - 1;
    }

    moves
}