mod bitboard;
mod knights;

use bitboard::{Bitboards, print_bitboard};

fn main() {
    let knights = 1u64 << 27;
    let friendlies = 0x0000_0000_0000_ff00;

    let bishops = 0x0000_0000_0000_0040; // Bishop on c1 (square 2)
    let occupied = 0x0000_0000_0000_1000; // A blocker on d4 (square 27)

    let moves = knights::generate_moves(knights, friendlies);
    knights::print_bitboard(moves);

    let bishop = bishop_moves(bishops, occupied);
}
