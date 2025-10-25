
#![allow(clippy::single_range_in_vec_init)]
#![allow(clippy::single_element_loop)]
#![allow(clippy::println_empty_string)]
#![allow(clippy::format_in_format_args)]
#![allow(unused_parens)]
#![allow(unused_attributes)]
#![allow(dead_code)]
#![allow(unused_imports)]



mod display;
mod soduku;

use display::{present_grid};
use soduku::{Soduku, Move, MoveStatus};




fn main() {

    let mut _clear = false;
    let mut soduku: Soduku = Soduku::new();

    present_grid(&soduku.board, false);

    let _m = Move::new(0,0,1);
    let _m2 = Move::new(0,1,2);
    let _m3 = Move::new(0,2,3);

    soduku.add_move(_m);
    soduku.add_move(_m2);
    soduku.add_move(_m3);
    
    present_grid(&soduku.board, true);

}
