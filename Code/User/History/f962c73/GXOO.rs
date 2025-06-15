mod bitboard;
mod knights;
mod bishops;

use bitboard::{Bitboards, print_bitboard};

fn main() {
    let knights = 1u64 << 27;
    let friendlies = 0x0000_0000_0000_ff00;

    let bishop_board = (1u64 << 9) | (1u64 << 14); // Bishop on b2 and c2
    
    // Example original occupied: all pawns on rank 2 (0x0000_0000_0000_ff00) and other pieces
    let original_occupied = 0x0000_0000_0000_ff00; 
    
    // Remove pawns on rank 2 so bishops can move freely
    let occupied = original_occupied & !(0x0000_0000_0000_ff00);

    let moves = knights::generate_moves(knights, friendlies);
    knights::print_bitboard(moves);

    let bishop_moves = bishops::bishop_moves(bishop_board, occupied);
    bishops::print_bitboard(bishop_moves);
}
