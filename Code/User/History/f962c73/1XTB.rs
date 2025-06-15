mod bitboard;
mod knights;
mod bishops;

use bitboard::{Bitboards, print_bitboard};

fn main() {
    let knights = 1u64 << 27;
    let friendlies = 0x0000_0000_0000_ff00;

    let bishop_board = (1u64 << 9) | (1u64 << 14); // Bishop on b2 and c2
    let occupied = 0x0000_0000_0000_1000; // A blocker on d4 (square 27)

    let moves = knights::generate_moves(knights, friendlies);
    knights::print_bitboard(moves);

    let bishop_moves = bishops::bishop_moves(bishop_board, occupied);
    bishops::print_bitboard(bishop_moves);
}
