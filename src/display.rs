
use ansi_escapes::*;

const ESC: char = 27u8 as char;


pub fn present_grid(v: &[u8], clear: bool) {

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


