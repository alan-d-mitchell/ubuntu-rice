mod bitboard;
mod knights;
mod bishops;

use bitboard::{Bitboards, print_bitboard};

fn main() {
    let bishop_board = (1u64 << 9) | (1u64 << 14); // Bishop on b2 and c2

     // Original occupied (for example, pawns on rank 2)
    let original_occupied = 0x0000_0000_0000_ff00;
    
    // Remove pawns on rank 2
    let mut occupied = original_occupied & !(0x0000_0000_0000_ff00);

    // Add rooks on a1 and g1 (square 0 and 6)
    occupied |= (1u64 << 0) | (1u64 << 6);

    let bishop_moves = bishops::bishop_moves(bishop_board, occupied);
    println!("Occupied:");
    bishops::print_bitboard(occupied);
    
    bishops::print_bitboard(bishop_moves);
}
