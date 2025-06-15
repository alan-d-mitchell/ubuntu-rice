mod bitboard;

use bitboard::Bitboards;

fn main() {
    let position = Bitboards::starting_position();
    print_bitboard(&position);
}
