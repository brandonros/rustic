use crate::{
    board::Board,
    engine::defs::Information,
    movegen::{defs::Move, MoveGenerator},
};
use crossbeam_channel::{Receiver, Sender};
use std::{sync::Arc, time::Instant};

pub const INF: i16 = 25_000;
pub const CHECKMATE: i16 = 24_000;
pub const STALEMATE: i16 = 0;
pub const CHECKPOINT: usize = 10_000; // nodes
pub const UPDATE_STATS: usize = 5_000_000; // nodes

pub type SearchResult = (Move, SearchTerminate);

#[derive(PartialEq)]
// These commands can be used by the engine thread to control the search.
pub enum SearchControl {
    Start,
    Stop,
    Quit,
    Nothing,
}

// Ways to terminate a search.
#[derive(PartialEq, Copy, Clone)]
pub enum SearchTerminate {
    Stop,    // Search is halted.
    Quit,    // Search module is quit completely.
    Nothing, // No command received yet.
}

// This struct holds all the search parameters as set by the engine thread.
// (These parameters are either default, or provided by the user interface
// before the game starts.)
pub struct SearchParams {
    pub depth: u8,
    pub time_for_move: u128,
}

impl SearchParams {
    pub fn new(depth: u8, time_for_move: u128) -> Self {
        Self {
            depth,
            time_for_move,
        }
    }
}

// The search function will put all findings collected during the running
// search into this struct.
#[derive(PartialEq)]
pub struct SearchInfo {
    pub depth: u8,
    pub start_time: Instant,
    pub last_checkpoint: usize,
    pub last_stats: usize,
    pub bm_at_depth: Move,
    pub nodes: usize,
    pub ply: u8,
    pub terminate: SearchTerminate,
}

impl SearchInfo {
    pub fn new() -> Self {
        Self {
            depth: 0,
            start_time: Instant::now(),
            last_checkpoint: 0,
            last_stats: 0,
            bm_at_depth: Move::new(0),
            nodes: 0,
            ply: 0,
            terminate: SearchTerminate::Nothing,
        }
    }
}

// After each completed depth, iterative deepening summarizes the running
// search results within this struct before sending it to the engine
// thread. The engine thread will send it to Comm, which will transform the
// information into UCI/XBoard/Console output and print it to STDOUT.

#[derive(PartialEq, Copy, Clone)]
pub struct SearchSummary {
    pub depth: u8,         // depth reached during search
    pub time: u128,        // milliseconds
    pub cp: i16,           // centipawns score
    pub mate: u8,          // mate in X moves
    pub nodes: usize,      // nodes searched
    pub nps: usize,        // nodes per second
    pub bm_at_depth: Move, // best move after this depth
}

#[derive(PartialEq, Copy, Clone)]
// This struct holds the currently searched move, and its move number in
// the list of legal moves. This struct is sent through the engine thread
// to Comm, to be transmitted to the (G)UI.
pub struct SearchCurrentMove {
    pub curr_move: Move,
    pub curr_move_number: u8,
}

impl SearchCurrentMove {
    pub fn new(curr_move: Move, curr_move_number: u8) -> Self {
        Self {
            curr_move,
            curr_move_number,
        }
    }
}

// This struct holds search statistics. These will be sent through the
// engine thread to Comm, to be transmitted to the (G)UI.
#[derive(PartialEq, Copy, Clone)]
pub struct SearchStats {
    pub nodes: usize, // Number of nodes searched
    pub nps: usize,   // Speed in nodes per second
}

impl SearchStats {
    pub fn new(nodes: usize, nps: usize) -> Self {
        Self { nodes, nps }
    }
}

// The search process needs references to a lot of data, such as a copy of
// the current board to make moves on, the move generator, search paramters
// (depth, time available, etc...), SearchInfo to put the results, and a
// control receiver so the search can receive commands from the engine.
// These references are grouped in SearchRefs, so they don't have to be
// passed one by one as function arguments.
pub struct SearchRefs<'a> {
    pub board: &'a mut Board,
    pub mg: &'a Arc<MoveGenerator>,
    pub search_params: &'a mut SearchParams,
    pub search_info: &'a mut SearchInfo,
    pub control_rx: &'a Receiver<SearchControl>,
    pub report_tx: &'a Sender<Information>,
}

#[derive(PartialEq)]
pub enum SearchReport {
    Finished(Move),
    SearchSummary(SearchSummary),
    SearchCurrentMove(SearchCurrentMove),
    SearchStats(SearchStats),
}
