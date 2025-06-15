type bitboard = u64;

#[derive(Debug, Clone, Copy)]
enum Piece {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
}

#[derive(Debug, Clone, Copy)]
enum Color {
        White,
        Black,
}

pub struct Bitboards {
        white_pawns: bitboard,
        black_pawns: bitboard,
        white_knights: bitboard,
        black_knights: bitboard,
        white_bishops: bitboard,
        black_bishops: bitboard,
        white_rooks: bitboard,
        black_rooks: bitboard,
        white_queen: bitboard,
        black_queen: bitboard,
        white_king: bitboard,
        black_king: bitboard,

        white_pieces: bitboard,
        black_pieces: bitboard,
        all_pieces: bitboard,
}

impl Bitboards {
        pub fn starting_position() -> Self {
                let white_pawns = 0x0000_0000_0000_ff00; // a2-h2
                let white_knights = 0x0000_0000_0000_0042; // b1 + g1
                let white_bishops = 0x0000_0000_0000_0024; // c1 + f1
                let white_rooks = 0x0000_0000_0000_0081; // a1 + h1
                let white_queen = 0x0000_0000_0000_0010; // d1
                let white_king = 0x0000_0000_0000_0008; // e1

                let black_pawns = 0x00ff_0000_0000_0000; // a7-h7
                let black_knights = 0x4200_0000_0000_0000; // b8 + g8
                let black_bishops = 0x2400_0000_0000_0000; // c8 + f8
                let black_rooks = 0x8100_0000_0000_0000; // a8 + h8
                let black_queen = 0x1000_0000_0000_0000; // d8
                let black_king = 0x0800_0000_0000_0000; // e8

                let white_pieces = white_pawns | white_knights | white_bishops | white_rooks | white_queen | white_king;
                let black_pieces = black_pawns | black_knights | black_bishops | black_rooks | black_queen | black_king;
                let all_pieces = white_pieces | black_pieces;

                Self {
                        white_pawns,
                        white_knights,
                        white_bishops,
                        white_rooks,
                        white_queen,
                        white_king,

                        black_pawns,
                        black_knights,
                        black_bishops,
                        black_rooks,
                        black_queen,
                        black_king,

                        white_pieces,
                        black_pieces,
                        all_pieces,
                }
        }
}

fn print_bitboard(position: &Bitboards) {
        for rank in (0..8).rev() {
                for file in 0..8 {
                        let square = rank * 8 + file;
                        let mask = 1u64 << square;
                        let piece = if position.white_pawns & mask != 0 {
                                'P'
                        } else if position.white_knights & mask != 0 {
                                'N'
                        } else if position.white_bishops & mask != 0 {
                                'B'
                        } else if position.white_rooks & mask != 0 {
                                'R'
                        } else if position.white_queen & mask != 0 {
                                'Q'
                        } else if position.white_king & mask != 0 {
                                'K'
                        } else if position.black_pawns & mask != 0 {
                                'p'
                        } else if position.black_knights & mask != 0 {
                                'n'
                        } else if position.black_bishops & mask != 0 {
                                'b'
                        } else if position.black_rooks & mask != 0 {
                                'r'
                        } else if position.black_queen & mask != 0 {
                                'q'
                        } else if position.black_king & mask != 0 {
                                'k'
                        } else {
                                '.'
                        };
                        
                        print!("{} ", piece);
                }
                println!();
        }
        println!();
}
