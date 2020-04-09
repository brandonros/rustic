use super::representation::{Board, UnMakeInfo};
use super::unmake_move;
use crate::defs::{
    A1, A8, BLACK, C1, C8, CASTLE_BK, CASTLE_BQ, CASTLE_WK, CASTLE_WQ, D1, D8, E1, E8, F1, F8, G1,
    G8, H1, H8, KING, PAWN, PNONE, ROOK, WHITE,
};
use crate::movegen::{info, movedefs::Move};

// TODO: Update comments

pub fn make_move(board: &mut Board, m: Move) -> bool {
    // Create the unmake info and store it.
    store_unmake_info(board, m);

    // Set "us" and "opponent"
    let us = board.active_color as usize;
    let opponent = (us ^ 1) as usize;

    // Dissect the move so we don't need "m.function()" and type casts everywhere.
    let piece = m.piece() as usize;
    let from = m.from();
    let to = m.to();
    let captured = m.captured() as usize;
    let promoted = m.promoted() as usize;
    let castling = m.castling();
    let double_step = m.double_step();
    let en_passant = m.en_passant();
    let promotion_move = promoted != PNONE;

    // If piece was captured with this move then remove it.
    if captured != PNONE {
        board.remove_piece(opponent, captured, to);
    }

    if captured == ROOK && (board.castling > 0) {
        adjust_castling_perms_on_rook_capture(board, to);
    }

    // Make the move, taking promotion into account.
    board.remove_piece(us, piece, from);
    if !promotion_move {
        board.put_piece(us, piece, to);
    } else {
        board.put_piece(us, promoted, to);
    }

    // The king performed a castling move. Make the correct rook move.
    if castling {
        let king_square = to;
        move_rook_during_castling(board, king_square);
    }

    // King or rook moves from starting square; castling permissions are dropped.
    if !castling && (piece == KING || piece == ROOK) && (board.castling > 0) {
        adjust_castling_perms_if_leaving_starting_square(board, from);
    }

    // Every move unsets the up-square (if not unset already).
    board.clear_ep_square();

    // After an en-passant maneuver, the opponent's pawn has yet to be removed.
    if en_passant {
        let pawn_square = if us == WHITE { to - 8 } else { to + 8 };
        board.remove_piece(opponent, PAWN, pawn_square);
    }

    // A double-step is the only move that sets the ep-square.
    if double_step {
        let ep_square = if us == WHITE { to - 8 } else { to + 8 };
        board.set_ep_square(ep_square);
    }

    // *** Update the remainder of the board state ***

    // Swap the side to move.
    board.swap_side();

    // Update the move counter
    if (piece == PAWN) || (captured != PNONE) {
        board.halfmove_clock = 0;
    } else {
        board.halfmove_clock += 1;
    }

    // Increase full move number if black has moved
    if us == BLACK {
        board.fullmove_number += 1;
    }

    /*** Validating move: see if "us" is in check ***/

    let king_square = board.get_pieces(KING, us).trailing_zeros() as u8;
    let is_legal = !info::square_attacked(board, opponent, king_square);

    // We're in check. Undo everything.
    if !is_legal {
        unmake_move::unmake_move(board);
    }

    is_legal
}

// Stores the current board state, and the move made while in that state
fn store_unmake_info(board: &mut Board, m: Move) {
    let unmake_info = UnMakeInfo::new(
        board.active_color,
        board.castling,
        board.en_passant,
        board.halfmove_clock,
        board.fullmove_number,
        board.zobrist_key,
        m,
    );
    board.unmake_list.push(unmake_info);
}

// This function changes castling permissions according to the rook being captured
fn adjust_castling_perms_on_rook_capture(board: &mut Board, square: u8) {
    board.zobrist_key ^= board.zobrist_randoms.castling(board.castling);
    match square {
        H1 => board.castling &= !CASTLE_WK,
        A1 => board.castling &= !CASTLE_WQ,
        H8 => board.castling &= !CASTLE_BK,
        A8 => board.castling &= !CASTLE_BQ,
        _ => (),
    }
    board.zobrist_key ^= board.zobrist_randoms.castling(board.castling);
}

// This function adjusts castling permissions if king or rook leaves a starting square.
fn adjust_castling_perms_if_leaving_starting_square(board: &mut Board, square: u8) {
    board.zobrist_key ^= board.zobrist_randoms.castling(board.castling);
    match square {
        H1 => board.castling &= !CASTLE_WK,
        A1 => board.castling &= !CASTLE_WQ,
        E1 => board.castling &= !(CASTLE_WK + CASTLE_WQ),
        H8 => board.castling &= !CASTLE_BK,
        A8 => board.castling &= !CASTLE_BQ,
        E8 => board.castling &= !(CASTLE_BK + CASTLE_BQ),
        _ => (),
    };
    board.zobrist_key ^= board.zobrist_randoms.castling(board.castling);
}

// This function moves the correct rook after the king has moved during castling.
fn move_rook_during_castling(board: &mut Board, king_square: u8) {
    let us = board.active_color as usize;
    board.zobrist_key ^= board.zobrist_randoms.castling(board.castling);
    match king_square {
        G1 => {
            board.remove_piece(us, ROOK, H1);
            board.put_piece(us, ROOK, F1);
            board.castling &= !(CASTLE_WK + CASTLE_WQ);
        }
        C1 => {
            board.remove_piece(us, ROOK, A1);
            board.put_piece(us, ROOK, D1);
            board.castling &= !(CASTLE_WK + CASTLE_WQ);
        }
        G8 => {
            board.remove_piece(us, ROOK, H8);
            board.put_piece(us, ROOK, F8);
            board.castling &= !(CASTLE_BK + CASTLE_BQ);
        }
        C8 => {
            board.remove_piece(us, ROOK, A8);
            board.put_piece(us, ROOK, D8);
            board.castling &= !(CASTLE_BK + CASTLE_BQ);
        }
        _ => (),
    }
    board.zobrist_key ^= board.zobrist_randoms.castling(board.castling);
}
