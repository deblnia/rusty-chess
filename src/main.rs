use std::fmt;

// fn format_grid(board:&Vec<Vec<u32>>) { 
//     for i in 0..8{ 
//         println!("{:?}", board[i])
//     }
// }

struct Board {
    array: Vec<Vec<u32>>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..8 { 
             f.write_fmt(format_args!("{:?} \n", self.array[i])) ?;
        }
        Ok(())
    }
}

fn main() {
    let width = 8;
    let height = 8;

    let array = vec![vec![0; width]; height];
    let board = Board {
        array
    }; 
    // println!("{:?}", array);
    // format_grid(&array); 
    println!("{}", board); 

    
}