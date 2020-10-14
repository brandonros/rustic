// search.rs contains the engine's search routine.

pub mod sorting;

use crate::{
    board::Board,
    defs::MAX_DEPTH,
    engine::defs::{ErrFatal, Information},
    movegen::{
        defs::{Move, MoveList},
        MoveGenerator,
    },
};
use crossbeam_channel::{Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

const INF: i16 = 25000;
const MATE: i16 = 24000;

#[derive(PartialEq)]
pub enum SearchControl {
    Start,
    Stop,
    Quit,
    Nothing,
}

#[derive(PartialEq)]
pub enum SearchTerminate {
    Stop,
    Quit,
    Nothing,
}

pub struct SearchParams {
    depth: u8,
}

impl SearchParams {
    pub fn new(depth: u8) -> Self {
        Self { depth }
    }
}

#[derive(PartialEq)]
pub struct SearchInfo {
    pub termination: SearchTerminate,
    pub nodes: usize,
    pub ply: u8,
}

impl SearchInfo {
    pub fn new() -> Self {
        Self {
            termination: SearchTerminate::Nothing,
            nodes: 0,
            ply: 0,
        }
    }
}

pub struct Search {
    handle: Option<JoinHandle<()>>,
    control_tx: Option<Sender<SearchControl>>,
}

impl Search {
    pub fn new() -> Self {
        Self {
            handle: None,
            control_tx: None,
        }
    }

    pub fn init(
        &mut self,
        report_tx: Sender<Information>,
        board: Arc<Mutex<Board>>,
        mg: Arc<MoveGenerator>,
    ) {
        // Set up a channel for incoming commands
        let (control_tx, control_rx) = crossbeam_channel::unbounded::<SearchControl>();

        // Create thread-local variables.
        let _t_report_tx = report_tx.clone();
        let t_mg = Arc::clone(&mg);
        let mut t_board = Arc::clone(&board);
        let mut t_search_info = SearchInfo::new();

        // Create the search thread.
        let h = thread::spawn(move || {
            let mut quit = false;
            let mut halt = true;

            while !quit {
                let cmd = control_rx.recv().expect(ErrFatal::CHANNEL);

                match cmd {
                    SearchControl::Start => {
                        t_search_info.termination = SearchTerminate::Nothing;
                        halt = false;
                    }
                    SearchControl::Stop => halt = true,
                    SearchControl::Quit => quit = true,
                    SearchControl::Nothing => (),
                }

                if !halt && !quit {
                    let mut search_params = SearchParams::new(6);
                    Search::iterative_deepening(
                        &mut t_board,
                        &t_mg,
                        &mut search_params,
                        &mut t_search_info,
                        &control_rx,
                    );
                }

                match t_search_info.termination {
                    SearchTerminate::Stop => {
                        halt = true;
                    }
                    SearchTerminate::Quit => {
                        halt = true;
                        quit = true;
                    }
                    SearchTerminate::Nothing => (),
                }
            }
        });

        // Store the thread's handle and command sender.
        self.handle = Some(h);
        self.control_tx = Some(control_tx);
    }

    // This function is used to send commands into the search thread.
    pub fn send(&self, cmd: SearchControl) {
        if let Some(tx) = &self.control_tx {
            tx.send(cmd).expect(ErrFatal::CHANNEL);
        }
    }

    pub fn wait_for_shutdown(&mut self) {
        if let Some(h) = self.handle.take() {
            h.join().expect(ErrFatal::THREAD);
        }
    }
}

// Actual search routines.
impl Search {
    fn iterative_deepening(
        board: &Arc<Mutex<Board>>,
        mg: &Arc<MoveGenerator>,
        search_params: &mut SearchParams,
        search_info: &mut SearchInfo,
        control_rx: &Receiver<SearchControl>,
    ) {
        let mut depth = 1;
        let mut terminate = false;

        while depth <= search_params.depth && depth <= MAX_DEPTH && !terminate {
            Search::alpha_beta(
                depth,
                -INF,
                INF,
                board,
                mg,
                search_params,
                search_info,
                control_rx,
            );
            depth += 1;

            // Check if termination is required.
            terminate = search_info.termination != SearchTerminate::Nothing;
        }
    }

    fn alpha_beta(
        depth: u8,
        alpha: i16,
        beta: i16,
        board: &Arc<Mutex<Board>>,
        mg: &Arc<MoveGenerator>,
        search_params: &mut SearchParams,
        search_info: &mut SearchInfo,
        control_rx: &Receiver<SearchControl>,
    ) {
        if depth == 0 {
            println!("done.");
            return;
        }

        // Check for stop or quit commands.
        // ======================================================================
        let cmd = control_rx.try_recv().unwrap_or(SearchControl::Nothing);
        match cmd {
            SearchControl::Stop => {
                search_info.termination = SearchTerminate::Stop;
                return;
            }
            SearchControl::Quit => {
                search_info.termination = SearchTerminate::Quit;
                return;
            }
            _ => (),
        };
        // ======================================================================

        println!("Depth: {}", depth);
        thread::sleep(std::time::Duration::from_secs(1));
        Search::alpha_beta(
            depth - 1,
            INF,
            -INF,
            board,
            mg,
            search_params,
            search_info,
            control_rx,
        );
    }
}
