

use std::time::Instant;

use crate::soduku::{Soduku, SolveResult, Solver};



#[derive(Clone, Debug)]
pub struct SodukuSolver {}


impl Solver for SodukuSolver {

    fn solve(board: &Soduku) -> (SolveResult, u128) {

        let _timer = Instant::now();

        // write your code below this line...




        // write your code above this line...

        let _elapsed = _timer.elapsed();

        (SolveResult::Undefined, _elapsed.as_millis())

    }

}