use std::io;
use rand::{thread_rng, Rng};

type Position = (usize, usize);

struct MineSweeper {
    width: usize,
    height: usize,
    open_cells: Vec<Position>,
    mines: Vec<Position>,
    flagged_cells: Vec<Position>,
}

impl MineSweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> MineSweeper {
        MineSweeper {
            width,
            height,
            open_cells: Vec::new(),
            mines: {
                let mut mines:Vec<Position> = Vec::new();

                while mines.len() < mine_count {
                    mines.push((random_number(width), random_number(height)))
                }

                mines
            },
            flagged_cells: Vec::new(),
        }
    }
}

//creates a random number between 0 and max
pub fn random_number(max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..max)
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
