//William Haugen
//Sudoku Solver
//See LICENSE for license information.
//
//
mod board;

fn main() {
    let grid: Vec<Vec<u8>> = vec!{
        vec!{0,0,0,1,1,1,2,2,2}, 
        vec!{0,0,0,1,1,1,2,2,2}, 
        vec!{0,0,0,1,1,1,2,2,2}, 
        vec!{3,3,3,4,4,4,5,5,5},
        vec!{3,3,3,4,4,4,5,5,5},
        vec!{3,3,3,4,4,4,5,5,5},
        vec!{6,6,6,7,7,7,8,8,8}, 
        vec!{6,6,6,7,7,7,8,8,8}, 
        vec!{6,6,6,7,7,7,8,8,8}
    };
    let _test_board = board::Board{grid:grid, calculations:0};
    let test_board = board::new(9);
    println!("{:?}", test_board.get_column(1));
    println!("{:?}", test_board.get_row(1));
    println!("{:?}", test_board.get_square(0));
    println!("{}",test_board.check_duplicate());
    println!("{}", test_board);
}
