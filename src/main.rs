
#![allow(clippy::single_range_in_vec_init)]
#![allow(clippy::single_element_loop)]
#![allow(clippy::println_empty_string)]
#![allow(clippy::format_in_format_args)]
#![allow(dead_code)]


mod display;
mod soduku;

use display::{present_grid};
use soduku::{Soduku};




fn main() {

    let mut _clear = false;
    let mut soduku: Soduku = Soduku::new();

    soduku.board[30] = 5;

    present_grid(&soduku.board, false);

    soduku.board[31] = 8;
    
    present_grid(&soduku.board, true);

}
