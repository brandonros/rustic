use crate::board::{playmove, representation::Board};
use crate::evaluation;
use crate::evaluation::EvalScore;
use crate::extra::print;
use crate::movegen::{movedefs::Move, movelist::MoveList};

pub fn search(board: &mut Board, depth: u64) -> Move {
    let mut move_list = MoveList::new();
    let mut best_move = Move::new();
    let mut best_eval: EvalScore = i64::MIN;

    board.gen_all_moves(&mut move_list);
    for i in 0..move_list.len() {
        let m = move_list.get_move(i);

        let legal = playmove::make(board, m);
        if legal {
            let eval = evaluation::evaluate(board);

            print::move_data(m);
            println!("Evaluation: {}", eval);

            if eval > best_eval {
                best_eval = eval;
                best_move = m;
            }
            playmove::unmake(board);
        }
    }

    best_move
}
