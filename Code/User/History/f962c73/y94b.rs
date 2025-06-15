mod bitboard;
mod knights;

use bitboard::{Bitboards, print_bitboard};

fn main() {
    let knights = 1u64 << 27;
    let friendlies = 0x0000_0000_0000_ff00;

    let moves = knights::generate_moves(knights, friendlies);
    knights::print_bitboard(moves);
}
