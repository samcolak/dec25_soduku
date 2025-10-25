
#![allow(clippy::single_range_in_vec_init)]
#![allow(clippy::single_element_loop)]
#![allow(clippy::println_empty_string)]
#![allow(clippy::format_in_format_args)]
#![allow(unused_parens)]
#![allow(unused_attributes)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]



mod display;
mod soduku;
mod solver;

use std::env;

use display::{present_grid};
use soduku::{Soduku, Move, MoveStatus, SolveResult, Solver};
use solver::{SodukuSolver};



fn main() {

    let _args: Vec<String> = env::args().collect();

    let mut soduku: Soduku = Soduku::new();

    match _args.len() {
        2 => {
            let _filename = _args[1].to_string();
            soduku.load(&_filename);
        },
        _ => {
            // do nothing at the moment
        }
    };

    // example code for the how the board works...

    // clear indicates to overwrite a current displayed board - false does not erase the previous board..
    present_grid(&soduku.board(), false);

    // let _m1 = Move::new(0,0,1, 0);
    // let _m2 = Move::new(0,1,2, 0);
    // let _m3 = Move::new(0,2,3, 0);

    // soduku.add_move(_m1);
    // soduku.add_move(_m2);
    // soduku.add_move(_m3);
        
    // present_grid(&soduku.board(), true);
    
    let (_result, _time_in_ms) = SodukuSolver::solve(&soduku);

}
