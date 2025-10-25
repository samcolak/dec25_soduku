

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
    pub v: u8
}


impl Move {

    pub fn new(column: u8, row: u8, val: u8) -> Self {
        Self {
            column: u8::clamp(column, 0, 8),
            row : u8::clamp(row, 0, 8),
            v: u8::clamp(val, 0, 9)
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
    pub board: [u8; 81],
    moves: Vec<Move>,
}


impl Soduku {


    pub fn idx_to_cr(idx: u8) -> (u8, u8) {
        let _idx = u8::clamp(idx, 0, 81);
        let _column = _idx % 9;
        let _row = _idx / 9;
        (_column, _row)
    }


    pub fn cr_to_idx(column: u8, row: u8) -> u8 {
        let _column = u8::clamp(column, 0, 8);
        let _row = u8::clamp(row, 0, 8);
        (_column + (_row * 9))
    }


    pub fn remaining_moves(&self) -> Vec<Move> {

        let mut _out: Vec<Move> = Vec::new();

        for (_i, _c) in self.board.iter().enumerate() {
            let (_column, _row) = Self::idx_to_cr(_i as u8);
            if self.board[_i] == 0 {
                _out.push(Move::new(_column, _row, 0))
            }
        }

        _out

    }


    pub fn new() -> Self {
        Self {
            board: [0; 81],
            moves: Vec::new(),
        }
    }


    pub fn add_move(&mut self, m: Move) -> MoveStatus {

        let mut _newboard = self.board;

        let idx = m.idx();
        let quad = m.quad();

        if _newboard[idx as usize] == 0 {

            _newboard[idx as usize] = m.v;

            let _vcheck = Self::check_column(m.column, &_newboard);
            let _hcheck = Self::check_row(m.row, &_newboard);
            let _qcheck = Self::check_quad(quad, &_newboard);

            if (_vcheck as i8 > 0) && (_hcheck as i8 > 0) && (_qcheck as i8 > 0) {
                
                self.board = _newboard;
                self.moves.push(m);

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


    fn contains_duplicates(valsin: &[u8]) -> bool {
        let mut _copy: Vec<u8> = valsin.to_vec();
        _copy.sort(); // we need to sort first...
        _copy.dedup();
        _copy.len() != valsin.len()
    }


    fn sum(valsin: &[u8]) -> u8 {
        let mut _total: u8 = 0;
        for _c in valsin {
            _total += _c;
        }
        _total
    }


    pub fn check_quad(quad: u8, v: &[u8]) -> MoveStatus {

        let _quad = u8::clamp(quad, 0, 8);

        // 0 1 2
        // 3 4 5
        // 6 7 8

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

        if Self::contains_duplicates(&_found) {
            MoveStatus::Duplicate
        } else {
            match Self::sum(&_found) {
                45 => MoveStatus::ValidComplete,
                _ => MoveStatus::ValidPartialComplete
            }
        }

    }


    pub fn check_column(column: u8, v: &[u8]) -> MoveStatus {

        let _column = u8::clamp(column, 0, 8);
        let mut _found: Vec<u8> = Vec::new();

        for _row in 0..9 {
            let _index: usize = (_column + (_row * 9)) as usize;
            let _v = v[_index];
            if _v > 0 {
                _found.push(_v);
            }
        }

        if Self::contains_duplicates(&_found) {
            MoveStatus::Duplicate
        } else {
            match Self::sum(&_found) {
                45 => MoveStatus::ValidComplete,
                _ => MoveStatus::ValidPartialComplete
            }
        }

    }


    pub fn check_row(row: u8, v: &[u8]) -> MoveStatus {

        let _row = u8::clamp(row, 0, 8);
        let mut _found: Vec<u8> = Vec::new();

        for _column in 0..9 {
            let _index: usize = (_column + (_row * 9)) as usize;
            let _v = v[_index];
            if _v > 0 {
                _found.push(_v);
            }
        }

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