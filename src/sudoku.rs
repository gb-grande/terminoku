use std::time::SystemTime;

use bitvec::prelude::*;
use rand::Rng;
//Board definitiion
#[derive(Clone, Copy, Debug)]
pub struct Board {
   num_matrix : [[i32; 9]; 9],
   num_filled : i32,
   candidates : [[BitArr!(for 9, in u8, Msb0); 9]; 9] 
}


//shuffle numbers array
//works by generating a random index to be the first element, then move forward
//
fn shuffle_array<T: Copy>(array : &mut [T]) {
    let size = array.len();
    for i in 0..size {
        let random_index = rand::thread_rng().gen_range(i..size);
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
            num_filled : 0,
            candidates : [[(bitarr!(u8, Msb0; 1; 9)); 9]; 9]
        }
    }
}

impl Board {
    pub fn new_empty() -> Self {
        Self {
            num_matrix : [[0; 9]; 9],
            num_filled : 0,
            candidates : [[(bitarr!(u8, Msb0; 1; 9)); 9]; 9]
        }
    }

    //generate a new board specifying the number of empty cells
    pub fn generate(num_empty: i32) -> Self {
        let now = SystemTime::now();
        let mut returnable_board = Self::new_empty();
        //generate a solution for a empty board
        Self::new_empty().solutions(&mut Some(&mut returnable_board), true);
        let mut random_cells_order : Vec<usize> = (0..81).collect();
        shuffle_array(&mut random_cells_order);
        let mut empty_cells = 0;
        for n in random_cells_order {
            if empty_cells == num_empty {
                break;
            }
            let cell_i : usize = n/9;
            let cell_j: usize = n%9;
            //
            if !returnable_board.is_removable(cell_i, cell_j) {
                continue;
            } 
            let current_number = returnable_board.num_matrix[cell_i][cell_j];
            returnable_board.remove_number(cell_i, cell_j);
            if returnable_board.solutions(&mut None, false) == 1 {
                empty_cells+=1;
            }
            else {
                returnable_board.insert_number(current_number, cell_i, cell_j);
            }
        }
        let elapsed = now.elapsed();
        match elapsed {
            Ok(time) => {
           // it prints '2'
           println!("Time taken to gen: {}", time.as_secs_f32());
       }
       Err(e) => {
           // an error occurred!
           println!("Error: {e:?}");
       }
        }
        returnable_board

    }



    pub fn print(&self) {
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
    //propagates a new number in a cell's changes to candidates across the board    
    pub fn update_candidates(&mut self, cell_i : usize, cell_j : usize){
        if self.num_matrix[cell_i][cell_j] == 0 {
            return;
        }
        let num = self.num_matrix[cell_i][cell_j];

        for j in 0..9 {
            self.candidates[cell_i][j].set((num - 1) as usize, false);
        }
        //check column
        for i in 0..9 {
            self.candidates[i][cell_j].set((num - 1) as usize, false);
        }
        //check block 
        let block_i_start = (cell_i / 3) * 3;
        let block_j_start = (cell_j / 3) * 3;
        for i in block_i_start..(block_i_start+3){
            for j in block_j_start..(block_j_start+3){
                self.candidates[i][j].set((num - 1) as usize, false);
            }
        }
        return;
    }
    pub fn rebuild_all_candidates(&mut self){
        self.candidates = [[(bitarr!(u8, Msb0; 1; 9)); 9]; 9];
        for i in 0..9{
            for j in 0..9 {
                self.update_candidates(i, j);
            }
        }


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
        //update candidates
        self.update_candidates(cell_i, cell_j);
        return true;
    }
    // remove filled number from cell and update candidates
    pub fn remove_number(&mut self, cell_i : usize, cell_j : usize){
        //updating num_matrix
        if self.num_matrix[cell_i][cell_j] != 0 {
            self.num_matrix[cell_i][cell_j] = 0;
            self.num_filled -= 1;
        }
        //must rebuild all
        self.rebuild_all_candidates();

    }

    //checks if it's possible to insert number in certain slot
    pub fn check_number (&self, num : i32, cell_i : usize, cell_j : usize) -> bool {
        if num < 1 || num > 9 {
            return false;
        }
        if self.num_matrix[cell_i][cell_j] != 0 {
            return false;
        }
        let is_possible = self.candidates[cell_i][cell_j].get((num - 1) as usize);
        match is_possible {
            Some(x) => return *x,
            None => return false

        }
    }
    
    //basic checks in order to not remove cells that would leave empty rows, columns or blocks
    fn is_removable(&self, cell_i : usize, cell_j : usize) -> bool{
        //checks column
        let mut num_in_column = 0;
        for i in 0..9{
            if self.num_matrix[i][cell_j] != 0 {
                num_in_column += 1;
            }
            if num_in_column > 1 {
                break;
            }

        }
        if num_in_column == 1 {
            return false
        }
        //checks row
        let mut num_in_row = 0;
        for j in 0..9{
            if self.num_matrix[cell_i][j] != 0 {
                num_in_row += 1;
            }
            if num_in_row > 1 {
                break;
            }

        }
        if num_in_row == 1 {
            return false
        }
        let mut num_in_square = 0;
        let block_i_start = (cell_i / 3) * 3;
        let block_j_start = (cell_j / 3) * 3;
        'outer: for i in block_i_start..(block_i_start+3){
            for j in block_j_start..(block_j_start+3){
                if self.num_matrix[i][j] != 0 {
                    num_in_square += 1 ;
                }
                if num_in_square > 1 {
                    break 'outer;
                }
            }
        }
        if num_in_square == 0 {
            return false;
        }

        true
    }
    
    fn solve_backtracking(&self, sol_counter : &mut i32, board_ref : &mut Option<&mut Board>, is_generating: bool) {
        //if bord is filled -> increment solution numbers
        if self.num_filled == 81 {
            //assign first solution to board passed as reference
            if *sol_counter == 0 {
                let opt = board_ref;
                match opt {
                    Some(reference) => **reference = self.clone(),
                    None => (),
                }
            }
            *sol_counter += 1;
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
                //only shuffles if trying to generate puzzle
                if is_generating {
                    shuffle_array(&mut numbers);
                }
                for num in numbers {
                    if self.check_number(num, i, j) {
                        //clone board and insert number
                        let mut updated_board = self.clone();
                        updated_board.insert_number(num, i, j);
                        updated_board.solve_backtracking(sol_counter, board_ref, is_generating);

                    }
                }
                return;
            }
        }


    }
    //calculates if it has 0, 1 or more solutions and copys one solution to a reference if specified
    pub fn solutions(&self, board_ref : &mut Option<&mut Board>, is_generating: bool) -> i32 {
        let mut total=0;
        self.solve_backtracking(&mut total, board_ref, is_generating);
        return total;
    }
}


