//William Haugen
//Sudoku Solver
//See LICENSE for license information.
//
//
use std::fmt::Display;

pub struct Board{
    pub grid: Vec<Vec<u8>>,
    pub size: usize,
    pub calculations: usize,
}

pub fn new(grid_size: u8)->Board{
    let mut grid_new: Vec<Vec<u8>> = Vec::new();
    for _ in 0..grid_size{
        grid_new.push(vec![0; grid_size as usize]);
    }
    Board{grid:grid_new, calculations:0, size: grid_size as usize}
}

impl Board{

    pub fn get_column(&self, column_number: u8) -> Vec<u8>{
        let column_number: usize = column_number.into();
        assert!(self.size>column_number);
        let mut column: Vec<u8> = Vec::new();

        for i in 0..self.size{
            column.push(self.grid[i][column_number]);
        }

        column
    }

    pub fn get_row(&self, row_number: u8) -> Vec<u8> {
        let row_number: usize = row_number.into();
        assert!(self.size>row_number);
        let mut row: Vec<u8> = Vec::new();
        for i in 0..self.grid[row_number].len(){
            row.push(self.grid[row_number][i]);
        }
        row
    }

    pub fn get_square(&self, box_number: u8) -> Vec<u8> {
        assert!(self.size>box_number as usize);
        let mut sudoku_box: Vec<u8> = Vec::new();
        let square_root: u8 = (self.size as f64).sqrt() as u8;
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

        for i in 0..self.size{
            if !dup{
                let mut already = vec![0; self.size+1];
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
                let mut already = vec![0; self.size+1];
                for j in 0..arr.len(){
                    already[arr[j] as usize] += 1;
                }
                for j in already{
                    if j > 1{
                        dup = true;
                    }
                }
                let arr = self.get_square(i as u8);
                let mut already = vec![0; self.size+1];
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

    pub fn valid(&self, initialize: bool) -> bool{
        let mut valid_board = true;
        for i in self.grid.iter(){
            if !valid_board{
                break;
            }
            for j in i{
                if !valid_board{
                    break;
                }
                if valid_board && j > &(self.size as u8){
                    valid_board = false;
                }
                else {
                    if !initialize && j < &(1 as u8){
                        valid_board = false;
                    }
                }
            }
        }

        valid_board
    }

    pub fn possible(&self, index: u8) -> Vec<u8> {
        let mut p: Vec<u8> = Vec::new();
        let sqroot: u8 = (self.size as f64).sqrt() as u8;
        let mut box_number = 0;
        box_number += index/(self.size as u8)/sqroot*sqroot;
        box_number += index%(self.size as u8)/sqroot;

        let square = self.get_square(box_number);
        let row = self.get_row(index/self.size as u8);
        let column = self.get_column(index%self.size as u8);

        {
            let mut temp: Vec<u8> = vec!{0; self.size};
            for i in 0..(self.size as usize){
                if row[i]> 0{
                    temp[row[i] as usize -1 ] += 1;
                }
                if column[i]> 0{
                    temp[column[i] as usize - 1] += 1;
                }
                if square[i] > 0{
                    temp[square[i] as usize - 1] += 1;
                }
            }
            for i in 0..temp.len(){
                if temp[i] == 0{
                    p.push(i as u8+1);
                }
            }
        }

        p
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