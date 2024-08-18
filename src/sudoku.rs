use core::panic;
use std::time::SystemTime;

use rand::Rng;
//Board definitiion
#[derive(Clone, Debug)]
pub struct Board {
   num_matrix : [[i32; 9]; 9],
   num_filled : i32,
   rows : [i16; 9],
   columns : [i16; 9],
   boxes : [i16; 9],
   empty_cells : Vec<(i32, i32)>
}

const FILLED : i16 = 0b111111111;

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

impl Board {
    pub fn new_empty() -> Self {
        let mut new_board = Self {
            num_matrix : [[0; 9]; 9],
            num_filled : 0,
            rows : [FILLED; 9],
            columns : [FILLED; 9],
            boxes : [FILLED; 9],
            empty_cells : Vec::new()
        };
        for i in 0..9{
            for j in 0..9 {
                new_board.empty_cells.push((i, j));
            }
        }
        new_board
    }

    //generate a new board specifying the number of empty cells
    pub fn generate(num_empty: i32, solved_ref : Option<&mut Board>) -> Self {
        let now = SystemTime::now();
        let mut returnable_board = Self::new_empty();
        //generate a solution for a empty board
        Self::new_empty().solutions(&mut Some(&mut returnable_board), true);
        match solved_ref {
            Some(solved) => {*solved = returnable_board.clone()}
            None => ()
        }
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
            //removes and checks if there is still one solution, if there is, continue
            let current_number = returnable_board.num_matrix[cell_i][cell_j];
            returnable_board.remove_number(cell_i, cell_j);
            if returnable_board.solutions(&mut None, false) == 1 {
                empty_cells+=1;
            }
            else {
                //cell to reinsert number is the last empty cell since removal pushes to the last position 
                returnable_board.insert_number(current_number, returnable_board.empty_cells.len() - 1);
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
    fn get_box_i(&self, cell_i : usize, cell_j : usize) -> usize{
        return cell_i - cell_i % 3 + cell_j/3;

    }

    fn get_candidates(&self, cell_i : usize, cell_j : usize) -> Vec<i32>{
        let combined = self.rows[cell_i] & self.columns[cell_j] & self.boxes[self.get_box_i(cell_i, cell_j)];
        let mut candidates = Vec::new();
        for i in 0..9{
            if ((1 << i as i16) & combined) > 0{
                candidates.push(i+1);
            }}
        return candidates;

    }

    fn get_number_of_candidates(&self, cell_i : usize, cell_j : usize ) -> i16{
        let combined = self.rows[cell_i] & self.columns[cell_j] & self.boxes[self.get_box_i(cell_i, cell_j)];
        let mut total :i16 = 0;
        for i in 0..9{
            if ((1 << i as i16) & combined) > 0{
                total+=1;
            }

        }
        total
    }

    // insert number in board if the slot isn't occupied, thorugh the empty cell
    pub fn insert_number(&mut self, num : i32, empty_cell_index : usize) -> bool {
        if num < 1 || num > 9 {
            panic!("Number isn't between 1 and 9");
        }
        if empty_cell_index >= self.empty_cells.len(){
            panic!("Trying to insert in empty cell that doesn't exist");
        }
        let cell = self.empty_cells[empty_cell_index];
        let cell_i = cell.0 as usize;
        let cell_j = cell.1 as usize;
        if self.num_matrix[cell_i][cell_j] != 0 {
            panic!("Trying to insert in a not empty cell");
        }
        self.empty_cells.remove(empty_cell_index);
        self.num_matrix[cell_i][cell_j] = num;
        //update candidates
        self.rows[cell_i] ^= 1<<(num - 1);
        self.columns[cell_j] ^= 1<<(num - 1);
        let box_i = self.get_box_i(cell_i, cell_j);
        self.boxes[box_i] ^= 1 << (num - 1);
        self.num_filled += 1;
        return true;
    }
    // remove filled number from cell and update candidates
    pub fn remove_number(&mut self, cell_i : usize, cell_j : usize){
        //set candidates
        let num = self.num_matrix[cell_i][cell_j];
        self.num_matrix[cell_i][cell_j] = 0;
        self.rows[cell_i] |= 1<<(num - 1);
        self.columns[cell_j] |= 1<<(num - 1);
        let box_i = self.get_box_i(cell_i, cell_j);
        self.boxes[box_i] |= 1<<(num - 1);
        self.empty_cells.push((cell_i as i32, cell_j as i32));
        self.num_filled-=1;

    }
    pub fn enter_number(&mut self, i : i32, j : i32, num: i32){
        //must find empty cell
        let mut empty_cell_index = 0;
        for (index, cell) in self.empty_cells.iter().enumerate() {
            if i == cell.0 && j == cell.1 {
                empty_cell_index = index;
                break;
            }

        }
        self.insert_number(num, empty_cell_index);
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
    
    fn solve_backtracking(&mut self , sol_counter : &mut i32, board_ref : &mut Option<&mut Board>, is_generating: bool) {
        //if bord is filled -> increment solution numbers
        if self.num_filled + self.empty_cells.len() as i32 != 81 {
            panic!("Empty + filled cells != 81")
        }
        
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
        //loop through empty cells
        let mut most_suitable : (i32, i32) = (0,0);
        let mut most_suitable_c_number = 10;
        let mut most_suitable_c_index = 0;
        for (i, c) in self.empty_cells.iter().enumerate() {
            let curr = self.get_number_of_candidates(c.0 as usize, c.1 as usize);
            if curr < most_suitable_c_number {
                most_suitable = *c;
                most_suitable_c_number = curr;
                most_suitable_c_index = i;
            }
        }
        if most_suitable_c_number==0  {
            return;
        }
        let last_index = self.empty_cells.len() - 1;
        self.empty_cells.swap(last_index, most_suitable_c_index);
        let mut candidates = self.get_candidates(most_suitable.0 as usize, most_suitable.1 as usize);
        if is_generating{
            shuffle_array(&mut candidates);

        } 
        for num in candidates{
            self.insert_number(num, last_index);
            self.solve_backtracking(sol_counter, board_ref, is_generating);
            self.remove_number(most_suitable.0 as usize, most_suitable.1 as usize);

        }

    }
    //calculates if it has 0, 1 or more solutions and copys one solution to a reference if specified
    pub fn solutions(&self, board_ref : &mut Option<&mut Board>, is_generating: bool) -> i32 {
        let mut total=0;
        let mut to_be_solved = self.clone();
        to_be_solved.solve_backtracking(&mut total, board_ref, is_generating);
        return total;
    }

    pub fn get_num(&self, i : i32, j : i32) -> i32{
        self.num_matrix[i as usize][j as usize]

    }
    pub fn is_solved(&self) -> bool {
        self.empty_cells.len() == 0
    }

}


