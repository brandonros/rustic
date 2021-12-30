/* =======================================================================
Rustic is a chess playing engine.
Copyright (C) 2019-2022, Marcel Vanthoor
https://rustic-chess.org/

Rustic is written in the Rust programming language. It is an original
work, not derived from any engine that came before it. However, it does
use a lot of concepts which are well-known and are in use by most if not
all classical alpha/beta-based chess engines.

Rustic is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License version 3 as published by
the Free Software Foundation.

Rustic is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
for more details.

You should have received a copy of the GNU General Public License along
with this program.  If not, see <http://www.gnu.org/licenses/>.
======================================================================= */

use super::{defs::ErrFatal, Engine};
use crate::{comm::defs::CommOut, search::defs::SearchReport};

impl Engine {
    pub fn search_handler(&mut self, search_report: &SearchReport) {
        match search_report {
            SearchReport::Finished(m) => {
                self.comm.send(CommOut::BestMove(*m));
                self.set_waiting();

                // If we are using a protocol that expects the engine to
                // maintain game state at all times, we execute the move
                // and send the game result.
                if self.comm.info().stateful() {
                    if self.board.lock().expect(ErrFatal::LOCK).make(*m, &self.mg) {
                        self.send_game_result();
                    } else {
                        // This should never happen. The search came up
                        // with an illegal move.
                        panic!("{}", ErrFatal::GENERATED_ILLEGAL_MOVE);
                    }
                }
            }

            SearchReport::SearchCurrentMove(curr_move) => {
                self.comm.send(CommOut::SearchCurrMove(*curr_move));
            }

            SearchReport::SearchSummary(summary) => {
                self.comm.send(CommOut::SearchSummary(summary.clone()));
            }

            SearchReport::SearchStats(stats) => {
                self.comm.send(CommOut::SearchStats(*stats));
            }

            SearchReport::Ready => (),
        }
    }
}
