use super::representation::{Board, UnMakeInfo};
use crate::movegen::movedefs::Move;

pub fn make_move(board: &mut Board, m: Move) {
    let unmake_info = UnMakeInfo::new(
        board.active_color,
        board.castling,
        board.en_passant,
        board.halfmove_clock,
        board.fullmove_number,
        board.get_zobrist_key(),
        m,
    );
    board.unmake_list.push(unmake_info);
}
