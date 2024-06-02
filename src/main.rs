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
    let mut b : sudoku::Board = Default::default();
    b.insert_number(1, 0, 0);
    b.print();
    let mut c;
    unsafe {c = b.num_solutions();}
    println!("{}", c);
    
}
