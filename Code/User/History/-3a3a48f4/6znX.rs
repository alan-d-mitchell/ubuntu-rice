pub type bitboard = u64;

const BOARD_SIZE: usize = 64;

// Precomputed magic numbers for bishops (from chess programming wiki)
const BISHOP_MAGICS: [bitboard; BOARD_SIZE] = [
    0x0040201000402000, 0x0000402010004000, 0x0000204081000200, 0x0000104400801000,
    0x0000082080400800, 0x0000041040200400, 0x0000020810200200, 0x0000010100800100,
    0x0000201000402000, 0x0000100800201000, 0x0000080400810000, 0x0000040200408000,
    0x0000020100204000, 0x0000010080102000, 0x0000008040081000, 0x0000004020040800,
    0x0000100800201000, 0x0000080400100800, 0x0000040200080400, 0x0000020100040200,
    0x0000010080020100, 0x0000008040010080, 0x0000004020008040, 0x0000002010004020,
    0x0000080400100800, 0x0000040200080400, 0x0000020100040200, 0x0000010080020100,
    0x0000008040010080, 0x0000004020008040, 0x0000002010004020, 0x0000001008002010,
    0x0000040200080400, 0x0000020100040200, 0x0000010080020100, 0x0000008040010080,
    0x0000004020008040, 0x0000002010004020, 0x0000001008002010, 0x0000000804001008,
    0x0000020100040200, 0x0000010080020100, 0x0000008040010080, 0x0000004020008040,
    0x0000002010004020, 0x0000001008002010, 0x0000000804001008, 0x0000000402000804,
    0x0000010080020100, 0x0000008040010080, 0x0000004020008040, 0x0000002010004020,
    0x0000001008002010, 0x0000000804001008, 0x0000000402000804, 0x0000000201000402,
    0x0000008040010080, 0x0000004020008040, 0x0000002010004020, 0x0000001008002010,
    0x0000000804001008, 0x0000000402000804, 0x0000000201000402, 0x0000000100800201,
];

// Directions bishop moves in, relative to square index
const BISHOP_DIRECTIONS: [(i32, i32); 4] = [
    (1, 1),  // NE
    (1, -1), // NW
    (-1, 1), // SE
    (-1, -1) // SW
];

// Helpers for board coordinates
fn rank_of(square: usize) -> usize { square / 8 }
fn file_of(square: usize) -> usize { square % 8 }

// Generate mask of relevant bishop blockers for a square (exclude edges)
fn bishop_mask(square: usize) -> bitboard {
    let mut mask = 0;

    let r = rank_of(square) as i32;
    let f = file_of(square) as i32;

    for &(dr, df) in &BISHOP_DIRECTIONS {
        let mut rr = r + dr;
        let mut ff = f + df;

        // Stop one short of the edges (exclude edge squares)
        while rr > 0 && rr < 7 && ff > 0 && ff < 7 {
            mask |= 1u64 << (rr * 8 + ff);
            rr += dr;
            ff += df;
        }
    }

    mask
}

// Generate bishop attacks for a given blockers set (used to precompute)
fn bishop_attack_on_the_fly(square: usize, blockers: bitboard) -> bitboard {
    let mut attacks = 0;

    let r = rank_of(square) as i32;
    let f = file_of(square) as i32;

    for &(dr, df) in &BISHOP_DIRECTIONS {
        let mut rr = r + dr;
        let mut ff = f + df;

        while rr >= 0 && rr < 8 && ff >= 0 && ff < 8 {
            let sq = (rr * 8 + ff) as usize;
            attacks |= 1u64 << sq;

            if blockers & (1u64 << sq) != 0 {
                break; // Blocked by piece
            }

            rr += dr;
            ff += df;
        }
    }

    attacks
}

// Given an index for bits in mask, generate a blocker bitboard for the occupancy variation
fn set_occupancy(index: usize, bits_in_mask: usize, mask: bitboard) -> bitboard {
    let mut blockers = 0;
    let mut bit_index = 0;

    for sq in 0..64 {
        let bit = 1u64 << sq;

        if mask & bit != 0 {
            if (index & (1 << bit_index)) != 0 {
                blockers |= bit;
            }
            bit_index += 1;

            if bit_index == bits_in_mask {
                break;
            }
        }
    }

    blockers
}

// Struct storing bishop attack data and tables
pub struct BishopMagic {
    masks: [bitboard; BOARD_SIZE],
    magics: [bitboard; BOARD_SIZE],
    attack_table_offsets: [usize; BOARD_SIZE], // Starting index in big attack table
    attack_table: Vec<bitboard>,               // Flat attack table for all squares
}

impl BishopMagic {
    // Initialize and precompute all tables
    pub fn new() -> Self {
        let mut masks = [0; BOARD_SIZE];
        let mut attack_table_offsets = [0; BOARD_SIZE];
        let mut magics = BISHOP_MAGICS;

        // Count total table size (sum of 2^(bits in mask) for all squares)
        let mut total_size = 0;
        for sq in 0..BOARD_SIZE {
            masks[sq] = bishop_mask(sq);
            total_size += 1 << masks[sq].count_ones();
        }

        let mut attack_table = Vec::with_capacity(total_size);

        let mut offset = 0;
        for sq in 0..BOARD_SIZE {
            attack_table_offsets[sq] = offset;

            let mask = masks[sq];
            let bits = mask.count_ones() as usize;
            let table_size = 1 << bits;

            for index in 0..table_size {
                let blockers = set_occupancy(index, bits, mask);
                let attack = bishop_attack_on_the_fly(sq, blockers);
                attack_table.push(attack);
            }

            offset += table_size;
        }

        Self {
            masks,
            magics,
            attack_table_offsets,
            attack_table,
        }
    }

    // Compute bishop attacks from precomputed tables using magic indexing
    pub fn bishop_attacks(&self, square: usize, occupied: bitboard) -> bitboard {
        let mask = self.masks[square];
        let magic = self.magics[square];
        let relevant_occupancy = occupied & mask;
        let bits_in_mask = mask.count_ones();

        // Magic indexing: multiply & shift
        let index = ((relevant_occupancy.wrapping_mul(magic)) >> (64 - bits_in_mask)) as usize;

        let offset = self.attack_table_offsets[square];
        self.attack_table[offset + index]
    }

    // Compute bishop moves for all bishops on board
    pub fn bishop_moves(&self, bishops: bitboard, occupied: bitboard) -> bitboard {
        let mut moves = 0;
        let mut bb = bishops;

        while bb != 0 {
            let sq = bb.trailing_zeros() as usize;
            moves |= self.bishop_attacks(sq, occupied);
            bb &= bb - 1;
        }

        moves
    }
}