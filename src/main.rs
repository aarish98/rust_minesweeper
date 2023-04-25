use std::io;

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

    let arr_size = get_user_input();
    
    if arr_size > 0 {
        println!("Hello, world!: {}", arr_size);
    }
    else {
        println!("Invalid parameters provided")
    }
    
}
