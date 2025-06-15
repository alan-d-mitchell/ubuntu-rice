use crate::board::{Board, Color, Square};
use crate::r#move::Move;
use crate::state::GameState;

pub fn undo_move(board: &mut Board, mov: Move, state: &GameState) {
    let from = mov.from;
    let to = mov.to;
    let color = board.side_to_move.opposite();

    // Revert side
    board.side_to_move = color;

    // Undo castling
    if mov.is_castling {
        match to.0 {
            62 => { // White kingside
                board.set_piece(Square(63), Some((Piece::Rook, Color::White)));
                board.set_piece(Square(61), None);
            }
            58 => { // White queenside
                board.set_piece(Square(56), Some((Piece::Rook, Color::White)));
                board.set_piece(Square(59), None);
            }
            6 => { // Black kingside
                board.set_piece(Square(7), Some((Piece::Rook, Color::Black)));
                board.set_piece(Square(5), None);
            }
            2 => { // Black queenside
                board.set_piece(Square(0), Some((Piece::Rook, Color::Black)));
                board.set_piece(Square(3), None);
            }
            _ => {}
        }
    }

    // Undo en passant
    if mov.is_en_passant {
        let ep_capture_sq = if color == Color::White {
            Square(to.0 - 8)
        } else {
            Square(to.0 + 8)
        };
        board.set_piece(ep_capture_sq, state.captured_piece);
        board.set_piece(to, None);
    } else {
        // Restore captured piece if there was one
        board.set_piece(to, state.captured_piece);
    }

    // Restore moved piece
    board.set_piece(from, Some((mov.piece, color)));

    // Restore castling rights, en passant square, etc.
    board.restore_state(state);
}
