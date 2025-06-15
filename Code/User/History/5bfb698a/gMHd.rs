use crate::movegen::bitboard::Bitboard;
use crate::move::Move;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone)]
pub struct Square(pub u8); // 0..63

#[derive(Copy, Clone)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

pub struct Board {
    pub pieces: [Option<(Piece, Color)>; 64], // Square-wise representation
    pub bitboards: [[Bitboard; 6]; 2],        // [color][piece_type]
    pub occupancies: [Bitboard; 3],           // [white, black, all]

    pub side_to_move: Color,
    pub castling: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Board {
    pub fn new() -> Self {
        // Create an empty board (you can override with FEN later)
        Self {
            pieces: [None; 64],
            bitboards: [[0; 6]; 2],
            occupancies: [0; 3],
            side_to_move: Color::White,
            castling: CastlingRights {
                white_kingside: false,
                white_queenside: false,
                black_kingside: false,
                black_queenside: false,
            },
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Load a FEN string into the board
    pub fn load_fen(&mut self, fen: &str) {
        // Implement full FEN parsing (piece placement, turn, castling, etc.)
        unimplemented!()
    }

    /// Print board for debugging
    pub fn print(&self) {
        println!("  a b c d e f g h");
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let idx = rank * 8 + file;
                let symbol = match self.pieces[idx as usize] {
                    Some((piece, color)) => piece_to_char(piece, color),
                    None => '.',
                };
                print!("{} ", symbol);
            }
            println!();
        }
        println!();
    }
}

fn piece_to_char(p: Piece, c: Color) -> char {
    match (p, c) {
        (Piece::Pawn, Color::White) => 'P',
        (Piece::Knight, Color::White) => 'N',
        (Piece::Bishop, Color::White) => 'B',
        (Piece::Rook, Color::White) => 'R',
        (Piece::Queen, Color::White) => 'Q',
        (Piece::King, Color::White) => 'K',
        (Piece::Pawn, Color::Black) => 'p',
        (Piece::Knight, Color::Black) => 'n',
        (Piece::Bishop, Color::Black) => 'b',
        (Piece::Rook, Color::Black) => 'r',
        (Piece::Queen, Color::Black) => 'q',
        (Piece::King, Color::Black) => 'k',
    }
}
