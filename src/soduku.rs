


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
#[repr(i8)]
pub enum MoveStatus {
    Duplicate = -3,
    Illegal = -2,
    Occupied = -1,
    Undefined = 0,
    Valid = 1,
    ValidPartialComplete = 5,
    ValidComplete = 8
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
            column,
            row,
            v: val
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
                MoveStatus::Valid
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

        let mut _found: Vec<u8> = Vec::new();

        for _row in 0..9 {
            let _index: usize = (column + (_row * 9)) as usize;
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

        let mut _found: Vec<u8> = Vec::new();

        for _column in 0..9 {
            let _index: usize = (_column + (row * 9)) as usize;
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