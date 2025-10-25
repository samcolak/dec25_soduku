
#![allow(clippy::single_range_in_vec_init)]
#![allow(clippy::single_element_loop)]
#![allow(clippy::println_empty_string)]
#![allow(clippy::format_in_format_args)]
#![allow(dead_code)]

use ansi_escapes::*;


const ESC: char = 27u8 as char;


fn contains_duplicates(valsin: &[u8]) -> bool {
    let mut copy: Vec<u8> = valsin.to_vec();
    copy.sort(); // we need to sort first...
    copy.dedup();
    copy.len() != valsin.len()
}


fn sum(valsin: &[u8]) -> u8 {
    let mut _total: u8 = 0;
    for _c in valsin {
        _total += _c;
    }
    _total
}


fn check_quad(quad: u8, v: &[u8]) -> bool {

    // 0 1 2
    // 3 4 5
    // 6 7 8

    let _x = (quad % 3) * 3;
    let _y = (quad / 3) * 3;

    let mut _found: Vec<u8> = Vec::new();

    for _row in _y..(_y + 3) {
        for _column in _x..(_x + 3) {
            let _index: usize = (_column + (_row * 9)) as usize;
            let _v = v[_index];
            if _v > 0 {
                _found.push(_v);
            }
        }
    }

    if contains_duplicates(&_found) {
        false
    } else {
        sum(&_found) == 45
    }

}


fn check_column(column: u8, v: &[u8]) -> bool {

    let mut _found: Vec<u8> = Vec::new();

    for _row in 0..9 {
        let _index: usize = (column + (_row * 9)) as usize;
        let _v = v[_index];
        if _v > 0 {
            _found.push(_v);
        }
    }

    if contains_duplicates(&_found) {
        false
    } else {
        sum(&_found) == 45
    }

}


fn check_row(row: u8, v: &[u8]) -> bool {

    let mut _found: Vec<u8> = Vec::new();

    for _column in 0..9 {
        let _index: usize = (_column + (row * 9)) as usize;
        let _v = v[_index];
        if _v > 0 {
            _found.push(_v);
        }
    }

    if contains_duplicates(&_found) {
        false
    } else {
        sum(&_found) == 45
    }

}


fn make_grid(v: &[u8], clear: bool) {

    // removes the last 13 lines as needed

    if clear {
        let _lines = 13;
        for _line in 0.._lines {
            print!("{}{}", format!("{}[A", ESC), EraseLine);
        }
    }

    // each block is a 3x3 square..

    let _x = 3;
    let _y = 3;

    for _row in 0..9 {
        
        let mut _rowout = "".to_string();

        for _column in 0..9 {

            let _index: usize = (_column + (_row * 9)) as usize;
            let _v: u8 = v[_index];

            if (_column % 3) == 0 {
                _rowout += " ";
            }

            match _v {
                0 => { _rowout += "⎕ "; },
                n => { _rowout += &format!("{n} "); }
            };

        }

        if (_row % 3) == 0 {
            println!("");
        }

        println!("{}", _rowout);

    }

    println!("");

}



fn main() {

    let mut _clear = false;
    let mut _points: Vec<u8> = [0; 81].to_vec();

    _points[30] = 5;

    make_grid(&_points, false);

    _points[31] = 8;
    
    make_grid(&_points, true);

}
