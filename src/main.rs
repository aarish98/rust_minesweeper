use std::io;

//initialize our Minesweeper Grid
fn init_grid(){
    println!("Hello! Give a number for a nxn grid.");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("Creating a grid of size {} x {}", i , i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

fn main() {

    init_grid();
    
    println!("Hello, world!");
}
