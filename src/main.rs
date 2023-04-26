use std::io;

type Position = (usize, usize);

struct MineSweeper {
    width: usize,
    height: usize,
    open_tiles: Vec<Position>,
    mines: Vec<Position>,
    flagged_tiles: Vec<Position>,
}

//get user cli input for file
fn get_user_input () -> i32{
    println!("Hello! Give a number for a nxn grid.");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => return 0,
    };
}

fn main() {

    let grid_size = get_user_input();
    
    if grid_size > 0 {
        let mut grid  =  [[0u8; 4]; 6];

        println!("Hello, world!: {}", grid_size);
    }
    else {
        println!("Invalid parameters provided")
    }
    
}
