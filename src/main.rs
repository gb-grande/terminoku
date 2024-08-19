use std::io;

mod sudoku;

fn num_to_diff(num : i8) -> String {
    match num {
        0 => String::from("Easy"),
        1 => String::from("Medium"),
        2 => String::from("Hard"), 
        3 => String::from("Very Hard"),
        _ => String::from("Unknown")
    }
}

fn num_diff_to_empty_cells(num : i32) -> i32 {
    (num + 1)*15 
}

fn main() {
    println!("Welcome to Terminoku!");
    println!("Please choose a difficulty");
    let mut difficulty : i8 = -1;
    let mut diff_input = String::new();
    while difficulty < 0 || difficulty > 3  {
        println!("0 - Easy\n1 - Medium\n2 - Hard\n3 - Very Hard");
        io::stdin()
            .read_line(&mut diff_input)
            .expect("Failed to read input.");
        difficulty = diff_input.trim().parse().expect("Input not an integer");
    }
    let cd = num_to_diff(difficulty);
    println!("Chosen difficulty is {cd}");
    let mut solved = sudoku::Board::new_empty();
    let mut game_board : sudoku::Board = sudoku::Board::generate(num_diff_to_empty_cells(difficulty as i32), Some(&mut solved));
    
    'outer: while !game_board.is_solved(){
        game_board.print();

        let mut input_line : String = "".to_string();
        println!("Choose a position (1-indexed) and number (i j num)");
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read input");
        println!();
        let input_splitted: Vec<&str> = input_line.split_whitespace().collect();
        if input_splitted.len() != 3 {
            println!("Please insert 3 terms.");
            continue 'outer;
        }
        let mut parsed : Vec<i32> = vec![];
        for s in input_splitted {
            let term_to_num : i32;
            let parse_res = s.parse::<i32>();
            match parse_res {
                Ok(x) => {term_to_num = x},
                Err (_)=> {
                    println!("All terms must be an integer.");
                    continue 'outer;
                } 
            }
            if term_to_num < 1 || term_to_num > 9 {
                println!("All terms must be between 1 and 9.");
                continue 'outer;
            }
            parsed.push(term_to_num);
        }
        let true_i = parsed[0] - 1;
        let true_j = parsed[1] - 1;
        let num = parsed[2];

        //checks if num already filled
        let current = game_board.get_num(true_i, true_j);
        if current != 0  {
            println!("Cell already filled.");
            continue;
        }
        let right_answer = solved.get_num(parsed[0] - 1, parsed[1] - 1);
        if num != right_answer {
            println!("Wrong number chosen for cell.");
            continue;
        }
        game_board.enter_number(true_i, true_j, num);
        println!("Correct!")
    }
    println!("Well done!")
}
