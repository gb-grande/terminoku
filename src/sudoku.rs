#[derive(Debug)]
pub struct Board {
    numMatrix: [[i8; 9]; 9]
}

// defining implementation for board methods

impl Default for Board {
    fn default() -> Self {
        Self {
            numMatrix : [[0; 9]; 9]
        }
    }
}

