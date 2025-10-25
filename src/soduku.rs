

#[derive(Clone, Debug)]
pub struct Soduku {
    pub board: [u8; 81]
}



impl Soduku {

    pub fn new() -> Self {
        Self {
            board: [0; 81],
        }
    }


    pub fn contains_duplicates(valsin: &[u8]) -> bool {
        let mut _copy: Vec<u8> = valsin.to_vec();
        _copy.sort(); // we need to sort first...
        _copy.dedup();
        _copy.len() != valsin.len()
    }


    pub fn sum(valsin: &[u8]) -> u8 {
        let mut _total: u8 = 0;
        for _c in valsin {
            _total += _c;
        }
        _total
    }


    pub fn check_quad(&self, quad: u8) -> bool {

        // 0 1 2
        // 3 4 5
        // 6 7 8

        let _x = (quad % 3) * 3;
        let _y = (quad / 3) * 3;

        let mut _found: Vec<u8> = Vec::new();

        for _row in _y..(_y + 3) {
            for _column in _x..(_x + 3) {
                let _index: usize = (_column + (_row * 9)) as usize;
                let _v = self.board[_index];
                if _v > 0 {
                    _found.push(_v);
                }
            }
        }

        if Self::contains_duplicates(&_found) {
            false
        } else {
            Self::sum(&_found) == 45
        }

    }


    pub fn check_column(&self, column: u8) -> bool {

        let mut _found: Vec<u8> = Vec::new();

        for _row in 0..9 {
            let _index: usize = (column + (_row * 9)) as usize;
            let _v = self.board[_index];
            if _v > 0 {
                _found.push(_v);
            }
        }

        if Self::contains_duplicates(&_found) {
            false
        } else {
            Self::sum(&_found) == 45
        }

    }


    pub fn check_row(&self, row: u8) -> bool {

        let mut _found: Vec<u8> = Vec::new();

        for _column in 0..9 {
            let _index: usize = (_column + (row * 9)) as usize;
            let _v = self.board[_index];
            if _v > 0 {
                _found.push(_v);
            }
        }

        if Self::contains_duplicates(&_found) {
            false
        } else {
            Self::sum(&_found) == 45
        }

    }

}