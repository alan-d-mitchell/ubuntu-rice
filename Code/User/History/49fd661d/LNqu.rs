
pub type Move = u32;

/// Move encoding (in 32 bits)
/// bits  0-5:  from square (0-63)
/// bits  6-11: to square (0-63)
/// bits 12-15: move flag (type of move)
/// bits 16-19: promotion piece type (if applicable)
/// bits 20-31: reserved or custom (e.g., scoring, quiet vs noisy, etc.)

// Move Flag Constants
pub const FLAG_QUIET: u32 = 0;
pub const FLAG_CAPTURE: u32 = 1;
pub const FLAG_DOUBLE_PAWN_PUSH: u32 = 2;
pub const FLAG_EN_PASSANT: u32 = 3;
pub const FLAG_CASTLING: u32 = 4;
pub const FLAG_PROMOTION: u32 = 5;

// Piece Type Encoding (if promotion)
pub const PROMO_NONE: u32 = 0;
pub const PROMO_N: u32 = 1;
pub const PROMO_B: u32 = 2;
pub const PROMO_R: u32 = 3;
pub const PROMO_Q: u32 = 4;

// Encodes a move
pub fn encode_move(from: u8, to: u8, flag: u32, promo: u32) -> Move {
    (from as Move)
        | ((to as Move) << 6)
        | ((flag & 0xF) << 12)
        | ((promo & 0xF) << 16)
}

// Decoding helpers
pub fn from_square(m: Move) -> u8 {
    (m & 0x3F) as u8
}

pub fn to_square(m: Move) -> u8 {
    ((m >> 6) & 0x3F) as u8
}

pub fn move_flag(m: Move) -> u32 {
    (m >> 12) & 0xF
}

pub fn promo_piece(m: Move) -> u32 {
    (m >> 16) & 0xF
}

// Checks
pub fn is_capture(m: Move) -> bool {
    move_flag(m) == FLAG_CAPTURE || move_flag(m) == FLAG_EN_PASSANT
}

pub fn is_promotion(m: Move) -> bool {
    move_flag(m) == FLAG_PROMOTION
}

pub fn is_castling(m: Move) -> bool {
    move_flag(m) == FLAG_CASTLING
}

pub fn is_en_passant(m: Move) -> bool {
    move_flag(m) == FLAG_EN_PASSANT
}

// Convert square index (0..63) to algebraic notation
pub fn square_to_coord(square: u8) -> String {
    let file = (square % 8) as u8;
    let rank = (square / 8) as u8;
    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;
    format!("{}{}", file_char, rank_char)
}

// Pretty-print move (e.g., e2e4, e7e8q, O-O)
pub fn move_to_string(m: Move) -> String {
    let from = square_to_coord(from_square(m));
    let to = square_to_coord(to_square(m));

    if is_castling(m) {
        if to_square(m) % 8 == 6 {
            return "O-O".to_string(); // kingside
        } else {
            return "O-O-O".to_string(); // queenside
        }
    }

    if is_promotion(m) {
        let promo = match promo_piece(m) {
            PROMO_N => "n",
            PROMO_B => "b",
            PROMO_R => "r",
            PROMO_Q => "q",
            _ => "?",
        };
        return format!("{}{}{}", from, to, promo);
    }

    format!("{}{}", from, to)
}