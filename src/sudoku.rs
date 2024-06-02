use rand::Rng;
//Board definitiion
#[derive(Clone, Copy, Debug)]
pub struct Board {
   num_matrix : [[i32; 9]; 9],
   num_filled : i32
}


//shuffle numbers array
//works by generating a random index to be the first element, then move forward
fn shuffle_array(array : &mut [i32; 9]) {
    for i in 0..9 {
        let random_index = rand::thread_rng().gen_range(i..9);
        //swap array elements
        let temp = array[i];
        array[i] = array[random_index];
        array[random_index] = temp;
    }

}




// defining implementation for board methods

impl Default for Board {
    fn default() -> Self {
        Self {
            num_matrix : [[0; 9]; 9],
            num_filled : 0
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
    pub fn insert_number(&mut self, num : i32, cell_i : usize, cell_j : usize) -> bool {
        if num < 1 || num > 9 {
            return false;
        }

        if self.num_matrix[cell_i][cell_j] != 0 {
            return false;
        }
        self.num_matrix[cell_i][cell_j] = num;
        self.num_filled += 1;
        return true;
    }
    // remove filled number from cell
    pub fn remove_number(&mut self, cell_i : usize, cell_j : usize){
        if self.num_matrix[cell_i][cell_j] != 0 {
            self.num_matrix[cell_i][cell_j] = 0;
            self.num_filled -= 1;
        }

    }

    //checks if it's possible to insert number in certain slot
    pub fn check_number (&self, num : i32, cell_i : usize, cell_j : usize) -> bool {
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
    
    
    fn solve_backtracking(&mut self, sol_counter : &mut i32) {
        //if bord is filled -> increment solution numbers
        if self.num_filled == 81 {
            *sol_counter += 1;
            self.print();
            return;
        }
        // it has already found out there is no unique solution
        if *sol_counter > 1 {
            return;
        }
        //loop through all cells and check if it's possible to fill with numbers
        for i in 0..9 {
            for j in 0..9 {
                if self.num_matrix[i][j] != 0 {
                    continue;
                }
                let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
                shuffle_array(&mut numbers); 
                for num in numbers {
                    if self.check_number(num, i, j) {
                        //update board to
                        self.insert_number(num, i, j);
                        self.solve_backtracking(sol_counter);
                        self.remove_number(i, j)

                    }
                }
                return;
            }
        }


    }
    //calculates if it has 0, 1 or more solutions
    pub fn num_solutions (&self) -> i32 {
        let mut total=0;
        let mut temp_board = self.clone();
        temp_board.solve_backtracking(&mut total);
        return total;
    }
}


