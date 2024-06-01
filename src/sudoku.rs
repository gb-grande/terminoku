//Board definitiion
#[derive(Debug)]
pub struct Board {
   num_matrix: [[i8; 9]; 9]
}

// defining implementation for board methods

impl Default for Board {
    fn default() -> Self {
        Self {
            num_matrix : [[0; 9]; 9]
        }
    }
}

impl Board {
    pub fn print (&self) {
        println!(" -----------------------");
        for i in 0..9 {
            print!("|");
            for j in 0..3 {
                let n1 = self.num_matrix[i][0 + j*3];
                let n2 = self.num_matrix[i][1 + j*3];
                let n3 = self.num_matrix[i][2 + j*3];
                print!(" {n1} {n2} {n3} |");
            }
            println!("");
            if i == 2 || i == 5 {
                println!("|-------+-------+-------|");
            }
        }
        println!(" -----------------------");
        }
    // insert number in board if the slot isn't occupied, i and j are 0-indexed 
    pub fn insert_number(&mut self, num : i8, cell_i : usize, cell_j : usize) -> bool {
        if num < 1 || num > 9 {
            return false;
        }

        if self.num_matrix[cell_i][cell_j] != 0 {
            return false;
        }
        self.num_matrix[cell_i][cell_j] = num;
        return true;
    }
    //checks if it's possible to insert number in certain slot
    pub fn check_number (&self, num : i8, cell_i : usize, cell_j : usize) -> bool {
        if num < 1 || num > 9 {
            return false;
        }
        if self.num_matrix[cell_i][cell_j] != 0 {
            return false;
        }
        //check row
        for j in 0..9 {
            if self.num_matrix[cell_i][j] == num {
                return false;
            }
        }
        //check column
        for i in 0..9 {
            if self.num_matrix[i][cell_j] == num {
                return false;
            }
        }
        //check block 
        let block_i_start = (cell_i / 3) * 3;
        let block_j_start = (cell_j / 3) * 3;
        for i in block_i_start..(block_i_start+3){
            for j in block_j_start..(block_j_start+3){
                if self.num_matrix[i][j] == num {
                    return false;
                }
            }
        }
        return true;
    }
    


    //calculates if it has 0, 1 or more solutions
    pub fn num_solution (&self) -> i8 {
        let total=0;

    }
    
    fn solve_backtracking(b : &mut board, &sol_counter) {



    }
}


