mod bitboard;
mod knights;

use bitboard::{Bitboards, print_bitboard};

fn main() {
    let position = Bitboards::starting_position();
    // print_bitboard(&position);

    let knight_moves = knight_attacks(1); // b1
    knight_bitboard_test(knight_moves);
}
