//William Haugen
//Sudoku Solver
//See LICENSE for license information.
//
//
use std::fmt::Display;

pub struct Board{
    pub grid: Vec<Vec<u8>>,
    pub calculations: usize,
}

pub fn new(grid_size: u8)->Board{
    let mut grid_new: Vec<Vec<u8>> = Vec::new();
    for _ in 0..grid_size{
        grid_new.push(vec![0; grid_size as usize]);
    }
    Board{grid:grid_new, calculations:0}
}

impl Board{

    pub fn get_column(&self, column_number: u8) -> Vec<u8>{
        let column_number: usize = column_number.into();
        assert!(self.grid[0].len()>column_number);
        let mut column: Vec<u8> = Vec::new();

        for i in 0..self.grid.len(){
            column.push(self.grid[i][column_number]);
        }

        column
    }

    pub fn get_row(&self, row_number: u8) -> Vec<u8> {
        let row_number: usize = row_number.into();
        assert!(self.grid.len()>row_number);
        let mut row: Vec<u8> = Vec::new();
        for i in 0..self.grid[row_number].len(){
            row.push(self.grid[row_number][i]);
        }
        row
    }

    pub fn get_square(&self, box_number: u8) -> Vec<u8> {
        assert!(self.grid.len()>box_number as usize);
        let mut sudoku_box: Vec<u8> = Vec::new();
        let square_root: u8 = (self.grid.len() as f64).sqrt() as u8;
        let start_col: u8 = (box_number % square_root) as u8 * square_root;
        let start_row: u8 = (box_number/square_root) as u8 * square_root;

       for i in start_row..(start_row+square_root){
            for j in start_col..(start_col+square_root){
                sudoku_box.push(self.grid[i as usize][j as usize] as u8);
            }
        }
        sudoku_box
    }

    pub fn check_duplicate(&self) -> bool{
        let mut dup: bool = false;

        for i in 0..self.grid.len(){
            if !dup{
                let mut already = vec![0; self.grid.len()+1];
                let arr = self.get_row(i as u8);
                for j in 0..arr.len(){
                    already[arr[j] as usize] += 1;
                }
                for j in already{
                    if j > 1{
                        dup = true;
                    }
                }
                let arr = self.get_column(i as u8);
                let mut already = vec![0; self.grid.len()+1];
                for j in 0..arr.len(){
                    already[arr[j] as usize] += 1;
                }
                for j in already{
                    if j > 1{
                        dup = true;
                    }
                }
                let arr = self.get_square(i as u8);
                let mut already = vec![0; self.grid.len()+1];
                for j in 0..arr.len(){
                    already[arr[j] as usize] += 1;
                }
                for j in already{
                    if j > 1{
                        dup = true;
                    }
                }
            }
        }

        dup
    }

}

impl Display for Board{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for iter in self.grid.iter(){
            for inner_iter in iter.iter(){
                write!(fmt, "{} ", inner_iter)?;
            }
            write!(fmt, "\n")?;
        }
        Ok(())
    }
}