use std::fs;

use serde_json::Value;




#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum SolveResult {
    Undefined = 0,
    Impossible = 1,
    Completed = 9
}


pub trait Solver
{    
    fn solve(board: &Soduku) -> (SolveResult, u128);
}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
#[repr(i8)]
pub enum MoveStatus {
    Duplicate = -3,
    Illegal = -2,
    Occupied = -1,
    Undefined = 0,
    Valid = 1,
    ValidPartialComplete = 5,
    ValidComplete = 8,
    GameFinished = 9,
}


impl From<MoveStatus> for i8 {
    fn from(value: MoveStatus) -> Self {
        value as i8
    }
}


#[derive(Clone, Debug, PartialEq, Default)]
pub struct Move {
    pub column: u8,
    pub row: u8,
    pub v: u8,
    pub owner: i8,
}


impl Move {

    pub fn new(column: u8, row: u8, val: u8, owner: i8) -> Self {
        Self {
            column: u8::clamp(column, 0, 8),
            row : u8::clamp(row, 0, 8),
            v: u8::clamp(val, 0, 9),
            owner
        }
    }

    pub fn idx(&self) -> u8 {
        (self.column + (self.row * 9))
    }

    pub fn quad(&self) -> u8 {

        // 0 1 2
        // 3 4 5
        // 6 7 8

        let _x = self.column / 3;
        let _y = self.row / 3;

        ((_y * 3) + _x)
        
    }

}


#[derive(Clone, Debug)]
pub struct Soduku {
    board: [u8; 81],
    moves: Vec<Move>,
}


impl Soduku {


    pub fn load(&mut self, filename: &str) -> bool {

        let _contents = match fs::read_to_string(filename) {
            Ok(_s) => _s,
            Err(_e) => "".to_string()
        };        

        if !_contents.is_empty() {

            let _json_version: Value = match serde_json::from_str(&_contents) {
                Ok(_c) => _c,
                Err(_e) => Value::Null
            };
    
            if _json_version != Value::Null {

                let _array_of_moves = _json_version.as_array().unwrap();

                for _listitem in _array_of_moves {

                    if let Some(_v) = _listitem.as_str() {                        
                        
                        let _parts = _v.split(".").collect::<Vec<&str>>();
                        
                        match _parts.len() {
                            3 => {
                                
                                let _column = _parts[0].parse::<u8>().unwrap();
                                let _row = _parts[1].parse::<u8>().unwrap();
                                let _value = _parts[2].parse::<u8>().unwrap();

                                let _m = Move::new(_column, _row, _value, 0);
                                self.add_move(_m);

                            }
                            _ => {
                                // ignore...
                            }
                        }

                    }

                }
            }

        }

        false
        
    }


    pub fn board(&self) -> [u8; 81] {
        self.board
    }


    // idx to column, row conversion..
    pub fn idx_to_cr(idx: u8) -> (u8, u8) {
        let _idx = u8::clamp(idx, 0, 80);
        let _column = _idx % 9;
        let _row = _idx / 9;
        (_column, _row)
    }


    // column, row to idx conversion..
    pub fn cr_to_idx(column: u8, row: u8) -> u8 {
        let _column = u8::clamp(column, 0, 8);
        let _row = u8::clamp(row, 0, 8);
        (_column + (_row * 9))
    }


    // find remaining moves available..
    pub fn remaining_moves(&self) -> Vec<Move> {

        let mut _out: Vec<Move> = Vec::new();

        for (_i, _c) in self.board.iter().enumerate() {
            let (_column, _row) = Self::idx_to_cr(_i as u8);
            if self.board[_i] == 0 {
                _out.push(Move::new(_column, _row, 0, -1))
            }
        }

        _out

    }


    // create a new board...
    pub fn new() -> Self {
        Self {
            board: [0; 81],
            moves: Vec::new(),
        }
    }


    // remove specific move..
    pub fn clear_move(&mut self, m: Move) -> bool {

        if let Some(index) = self.moves.iter().position(|x| *x == m) {

            self.moves.remove(index);
            let _idx = m.idx();
            self.board[_idx as usize] = 0; // clear the value
            true
            
        } else {
            false
        }

    }


    // remove the last move... (pop from stack)
    pub fn pop_move(&mut self) -> Option<Move> {

        if let Some(_lastmove) = self.moves.pop() {

            let _idx = _lastmove.idx();            
            self.board[_idx as usize] = 0; // clear the value
            Some(_lastmove)

        } else {

            None

        }

    }


    // add a move... if its legal :)
    pub fn add_move(&mut self, m: Move) -> MoveStatus {

        let mut _newboard = self.board;

        let idx = m.idx();
        let quad = m.quad();

        if _newboard[idx as usize] == 0 {

            _newboard[idx as usize] = m.v;

            // check x, y and quadrant perspectives...
            let _vcheck = Self::check_column(m.column, &_newboard);
            let _hcheck = Self::check_row(m.row, &_newboard);
            let _qcheck = Self::check_quad(quad, &_newboard);

            if (_vcheck as i8 > 0) && (_hcheck as i8 > 0) && (_qcheck as i8 > 0) {
                
                self.board = _newboard;
                self.moves.push(m);

                // was this the last move ... ?
                let remaining = self.remaining_moves();
                
                if remaining.is_empty() {
                    MoveStatus::GameFinished
                } else {
                    MoveStatus::Valid
                }

            } else {

                MoveStatus::Illegal

            }

        } else {
            // move is invalid - occupying position
            MoveStatus::Occupied
        }

    }


    // check if a set contains duplicates (given the input set length == dedup set length)
    fn contains_duplicates(valsin: &[u8]) -> bool {
        let _copy: Vec<u8> = Self::cleaned_nos(valsin);
        _copy.len() != valsin.len()
    }


    // calculate the sum of all the numbers in the sequence...
    fn sum(valsin: &[u8]) -> u8 {
        let mut _total: u8 = 0;
        for _c in valsin {
            _total += _c;
        }
        _total
    }


    pub fn fetch_quad(quad: u8, v: &[u8]) -> Vec<u8> {

        // 0 1 2
        // 3 4 5
        // 6 7 8

        let _quad = u8::clamp(quad, 0, 8);

        let _x = (_quad % 3) * 3;
        let _y = (_quad / 3) * 3;

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

        _found
        
    }


    // check if a quadrant is now completed
    pub fn check_quad(quad: u8, v: &[u8]) -> MoveStatus {

        let mut _found: Vec<u8> = Self::fetch_quad(quad, v);

        if Self::contains_duplicates(&_found) {
            MoveStatus::Duplicate
        } else {
            match Self::sum(&_found) {
                45 => MoveStatus::ValidComplete,
                _ => MoveStatus::ValidPartialComplete
            }
        }

    }


    pub fn cleaned_nos(valsin: &[u8]) -> Vec<u8> {
        let mut _valsout = valsin.to_vec();
        _valsout.sort();
        _valsout.dedup();
        _valsout
    }


    pub fn remaining_nos(valsin: &[u8]) -> Vec<u8> {
        let mut _valsout = Vec::new();
        let uniques = Self::cleaned_nos(valsin);

        if !uniques.is_empty() {

            if uniques.len() == 9 {
                return Vec::new(); // return empty set - all done...
            }

            for i in 1..=9 {
                if !uniques.contains(&i) {
                    _valsout.push(i); // return values if they dont appear in set..
                }
            }

        }

        _valsout
    }


    pub fn fetch_column(column: u8, v: &[u8]) -> Vec<u8> {

        let _column = u8::clamp(column, 0, 8);
        let mut _found: Vec<u8> = Vec::new();

        for _row in 0..9 {
            let _index: usize = (_column + (_row * 9)) as usize;
            let _v = v[_index];
            if _v > 0 {
                _found.push(_v);
            }
        }

        _found
        
    }


    // check that a column validity status...
    pub fn check_column(column: u8, v: &[u8]) -> MoveStatus {

        let mut _found: Vec<u8> = Self::fetch_column(column, v);

        if Self::contains_duplicates(&_found) {
            MoveStatus::Duplicate
        } else {
            match Self::sum(&_found) {
                45 => MoveStatus::ValidComplete,
                _ => MoveStatus::ValidPartialComplete
            }
        }

    }


    pub fn fetch_row(row: u8, v: &[u8]) -> Vec<u8> {

        let _row = u8::clamp(row, 0, 8);
        let mut _found: Vec<u8> = Vec::new();

        for _column in 0..9 {
            let _index: usize = (_column + (_row * 9)) as usize;
            let _v = v[_index];
            if _v > 0 {
                _found.push(_v);
            }
        }

        _found

    }


    // check the row validity status...
    pub fn check_row(row: u8, v: &[u8]) -> MoveStatus {

        let mut _found: Vec<u8> = Self::fetch_row(row, v);

        if Self::contains_duplicates(&_found) {
            MoveStatus::Duplicate
        } else {
            match Self::sum(&_found) {
                45 => MoveStatus::ValidComplete,
                _ => MoveStatus::ValidPartialComplete
            }
        }

    }

}