use crate::board::{Board, Color, Piece, Square};
use crate::r#move::Move;
use crate::state::GameState;

pub fn make_move(board: &mut Board, mov: Move, state: &mut GameState) -> bool {
    let from = mov.from;
    let to = mov.to;
    let moving_piece = board.piece_at(from);

    if moving_piece.is_none() {
        return false;
    }

    let (piece, color) = moving_piece.unwrap();

    // Store current board state for undo
    state.save(board, mov);

    // Clear the source square
    board.set_piece(from, None);

    // Handle captures
    if let Some(captured) = board.piece_at(to) {
        state.captured_piece = Some(captured);
    }

    // Handle promotions
    let promotion = mov.promotion;
    if let Some(promoted_piece) = promotion {
        board.set_piece(to, Some((promoted_piece, color)));
    } else {
        board.set_piece(to, Some((piece, color)));
    }

    // Handle en passant
    if mov.is_en_passant {
        let ep_capture_sq = if color == Color::White {
            Square(to.0 - 8)
        } else {
            Square(to.0 + 8)
        };
        board.set_piece(ep_capture_sq, None);
    }

    // Handle castling
    if mov.is_castling {
        match to.0 {
            62 => { // White kingside
                board.set_piece(Square(63), None);
                board.set_piece(Square(61), Some((Piece::Rook, Color::White)));
            }
            58 => { // White queenside
                board.set_piece(Square(56), None);
                board.set_piece(Square(59), Some((Piece::Rook, Color::White)));
            }
            6 => { // Black kingside
                board.set_piece(Square(7), None);
                board.set_piece(Square(5), Some((Piece::Rook, Color::Black)));
            }
            2 => { // Black queenside
                board.set_piece(Square(0), None);
                board.set_piece(Square(3), Some((Piece::Rook, Color::Black)));
            }
            _ => {}
        }
    }

    // Update castling rights, en passant, etc.
    board.update_state_after_move(mov);

    // Switch sides
    board.side_to_move = match board.side_to_move {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    true
}