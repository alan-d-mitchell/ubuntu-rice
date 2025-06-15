mod bitboard;
mod knights;

use bitboard::{Bitboards, print_bitboard};

fn main() {
    let knights = 0x0000_0000_0000_0042;
    let friendlies = 0x0000_0000_0000_ff00;

    let moves = knights::generate_moves(knights, friendlies);
}
