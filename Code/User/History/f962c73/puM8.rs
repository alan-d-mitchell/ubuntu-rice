mod bitboard;

use bitboard::{Bitboards, print_bitboard};

fn main() {
    let position = Bitboards::starting_position();
    print_bitboard(&position);
}
