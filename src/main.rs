use std::{io, collections::btree_map::Range};
use rand::{thread_rng, Rng};

type Position = (i32, i32);

struct MineSweeper {
    width: i32,
    height: i32,
    open_cells: Vec<Position>,
    mines: Vec<Position>,
    flagged_cells: Vec<Position>,
}

impl MineSweeper {
    pub fn new(width: i32, height: i32, mine_count: usize) -> MineSweeper {
        MineSweeper {
            width,
            height,
            open_cells: {
                let mut open_cells: Vec<Position>  =  Vec::new();
                for i in 0..width {
                    for j in 0..height{
                        open_cells.push((i,j));
                    }
                }
                open_cells
            },
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
pub fn random_number(max: i32) -> i32 {
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

        let mut mineSweeper = MineSweeper::new(grid_size, grid_size, (0.25_f64 * (grid_size* grid_size) as f64).floor() as usize );
        


        println!("Hello, world!: {}", grid_size);
    }
    else {
        println!("Invalid parameters provided")
    }
    
}
