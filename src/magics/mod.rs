/** Magics.rs implements the generation of magic bitboards for sliders, and the attack tables
 * for the non-slider squares. After all the bitboards are generated, using the three functions
 * "get_non_slider_attacks()", "get_slider_attacks()" and "get_pawn_attacks()" will "magically"
 * return all the possible attacks from the given square on the current board.
 * All possible moves for all pieces on all board squares (taking into account all possible
 * combination of blockers for the sliders) are calculated at the start of the engine, which
 * saves tremendous amounts of time in the move generator.
 */
mod masks;

use crate::defines::{
    Bitboard, Piece, Side, ALL_SQUARES, BLACK, FILE_A, FILE_B, FILE_G, FILE_H, KING, KNIGHT,
    NR_OF_SQUARES, PAWN_SQUARES, RANK_1, RANK_2, RANK_7, RANK_8, WHITE,
};
use crate::print;
use crate::utils::{create_bb_files, create_bb_ranks};
use masks::{create_bishop_mask, create_rook_mask};

const WHITE_BLACK: usize = 2;
const EMPTY: Bitboard = 0;
const NSQ: usize = NR_OF_SQUARES as usize;

// This is a total sum of all rook or bishop blocker permutations per square.
// const ROOK_TABLE_SIZE: u32 = 102400;
// const BISHOP_TABLE_SIZE: u32 = 5248;

/**
 * The struct "Magics" will hold all of the attack tables for each piece on each square.
*/
pub struct Magics {
    _king: [Bitboard; NSQ],
    _knight: [Bitboard; NSQ],
    _pawns: [[Bitboard; NSQ]; WHITE_BLACK],
}

impl Default for Magics {
    fn default() -> Magics {
        Magics {
            _king: [EMPTY; NSQ],
            _knight: [EMPTY; NSQ],
            _pawns: [[EMPTY; NSQ]; WHITE_BLACK],
        }
    }
}

impl Magics {
    /** Initialize all of the attack tables for each piece on each square. */
    pub fn initialize(&mut self) {
        let files = create_bb_files();
        let ranks = create_bb_ranks();

        self.init_king(&files, &ranks);
        self.init_knight(&files, &ranks);
        self.init_pawns(&files);
        self.init_magics();
    }

    /** Return non-slider (King, Knight) attacks for the given square. */
    pub fn get_non_slider_attacks(&self, piece: Piece, square: u8) -> Bitboard {
        match piece {
            KING => self._king[square as usize],
            KNIGHT => self._knight[square as usize],
            _ => 0,
        }
    }

    /** Return pawn attacks for the given square. */
    pub fn get_pawn_attacks(&self, side: Side, square: u8) -> Bitboard {
        self._pawns[side][square as usize]
    }

    /**
     * Generate all the possible king moves for each square.
     * Exampe:
     * Generate a bitboard for the square the king is on.
     * Generate a move to Up-Left, if the king is not on the A file, and not on the last rank.
     * Generate a move to Up, if the king is not on the last rank.
     * ... and so on. All the moves are combined in the bb_move bitboard.
     * Do this for each square.
     */
    fn init_king(&mut self, files: &[Bitboard; 8], ranks: &[Bitboard; 8]) {
        for sq in ALL_SQUARES {
            let bb_square = 1u64 << sq;
            let bb_moves = (bb_square & !files[FILE_A] & !ranks[RANK_8]) << 7
                | (bb_square & !ranks[RANK_8]) << 8
                | (bb_square & !files[FILE_H] & !ranks[RANK_8]) << 9
                | (bb_square & !files[FILE_H]) << 1
                | (bb_square & !files[FILE_H] & !ranks[RANK_1]) >> 7
                | (bb_square & !ranks[RANK_1]) >> 8
                | (bb_square & !files[FILE_A] & !ranks[RANK_1]) >> 9
                | (bb_square & !files[FILE_A]) >> 1;
            self._king[sq as usize] = bb_moves;
        }
    }

    /**
     * Generate all the possible knight moves for each square.
     * Works exactly the same as the king move generation.
     */
    fn init_knight(&mut self, files: &[Bitboard; 8], ranks: &[Bitboard; 8]) {
        for sq in ALL_SQUARES {
            let bb_square = 1u64 << sq;
            let bb_moves = (bb_square & !ranks[RANK_8] & !ranks[RANK_7] & !files[FILE_A]) << 15
                | (bb_square & !ranks[RANK_8] & !ranks[RANK_7] & !files[FILE_H]) << 17
                | (bb_square & !files[FILE_A] & !files[FILE_B] & !ranks[RANK_8]) << 6
                | (bb_square & !files[FILE_G] & !files[FILE_H] & !ranks[RANK_8]) << 10
                | (bb_square & !ranks[RANK_1] & !ranks[RANK_2] & !files[FILE_A]) >> 17
                | (bb_square & !ranks[RANK_1] & !ranks[RANK_2] & !files[FILE_H]) >> 15
                | (bb_square & !files[FILE_A] & !files[FILE_B] & !ranks[RANK_1]) >> 10
                | (bb_square & !files[FILE_G] & !files[FILE_H] & !ranks[RANK_1]) >> 6;
            self._knight[sq as usize] = bb_moves;
        }
    }

    /**
     * Generate all the possible pawn capture targets for each square.
     * Same again... generate a move to up-left/up-right, or down-left down-right
     * if the location of the pawn makes that move possible.
     */
    fn init_pawns(&mut self, files: &[Bitboard; 8]) {
        for sq in PAWN_SQUARES {
            let bb_square = 1u64 << sq;
            let w = (bb_square & !files[FILE_A]) << 7 | (bb_square & !files[FILE_H]) << 9;
            let b = (bb_square & !files[FILE_A]) >> 9 | (bb_square & !files[FILE_H]) >> 7;
            self._pawns[WHITE][sq as usize] = w;
            self._pawns[BLACK][sq as usize] = b;
        }
    }

    /** This is the main part of the module: it generates all the "magic" numbers and
     * bitboards for every slider, on every square, for every blocker setup.
     * A blocker is a piece that is "in the way", causing the slider to not be able to
     * 'see' beyond that piece..
     */
    fn init_magics(&mut self) {
        // TODO: Implement magics
        // for i in ALL_SQUARES {
        //     println!("square: {}", i);
        //     let x = create_rook_mask(i);
        //     print::bitboard(x, Some(i));
        // }
    }
}
