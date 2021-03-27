
//William Haugen
//Sudoku Solver
//See LICENSE for license information.
//
//
#[path="../src/board.rs"]
mod board;

#[test]
fn new_valid_test(){
    let grid: Vec<Vec<u8>> = vec!{
        vec!{0,0,0,0},
        vec!{0,0,0,0},
        vec!{0,0,0,0},
        vec!{0,0,0,0}
    };
    let b = board::new(4);
    assert_eq!(b.grid, grid);

    let grid: Vec<Vec<u8>> = vec!{
        vec!{0,0,0,0,0,0,0,0,0},
        vec!{0,0,0,0,0,0,0,0,0},
        vec!{0,0,0,0,0,0,0,0,0},
        vec!{0,0,0,0,0,0,0,0,0},
        vec!{0,0,0,0,0,0,0,0,0},
        vec!{0,0,0,0,0,0,0,0,0},
        vec!{0,0,0,0,0,0,0,0,0},
        vec!{0,0,0,0,0,0,0,0,0},
        vec!{0,0,0,0,0,0,0,0,0}
    };
    let b = board::new(9);
    assert_eq!(b.grid, grid);
}

#[test]
fn get_column_valid_test(){
    let grid: Vec<Vec<u8>> = vec!{
        vec!{1,2,3,4,5,6,7,8,9},
        vec!{2,3,4,5,6,7,8,9,10}}
    ;
    let b = board::Board{grid: grid, calculations: 0};

    assert_eq!(b.get_column(8), vec!{9,10});
    assert_eq!(b.get_column(0), vec!{1,2});
    assert_eq!(b.get_column(5), vec!{6,7});
}

#[test]
#[should_panic]
fn get_column_invalid_test(){
    let grid: Vec<Vec<u8>> = vec!{vec!{1,2,3,4,5,6,7,8,9}, vec!{2,3,4,5,6,7,8,9,10}};
    let b = board::Board{grid: grid, calculations: 0};

    b.get_column(10);
}

#[test]
fn get_row_valid_test(){
    let grid: Vec<Vec<u8>> = vec!{vec!{1,2,3,4,5,6,7,8,9}, vec!{2,3,4,5,6,7,8,9,10}};
    let b = board::Board{grid: grid, calculations: 0};
    let grid: Vec<Vec<u8>> = vec!{vec!{1,2,3,4,5,6,7,8,9}, vec!{2,3,4,5,6,7,8,9,10}};
    assert_eq!{b.get_row(0), grid[0]};
    assert_eq!(b.get_row(1), grid[1]);
}

#[test]
#[should_panic]
fn get_row_invalid_test(){
    let grid: Vec<Vec<u8>> = vec!{vec!{1,2,3,4,5,6,7,8,9}, vec!{2,3,4,5,6,7,8,9,10}};
    let b = board::Board{grid: grid, calculations: 0};

    b.get_row(3);
}

#[test]
fn get_square_valid_test(){
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
    let b = board::Board{grid: grid, calculations: 0};

    assert_eq!(b.get_square(0), vec!{0,0,0,0,0,0,0,0,0});
    assert_eq!(b.get_square(8), vec!{8,8,8,8,8,8,8,8,8});
    assert_eq!(b.get_square(4), vec!{4,4,4,4,4,4,4,4,4});
    assert_eq!(b.get_square(6), vec!{6,6,6,6,6,6,6,6,6});
}

#[test]
#[should_panic]
fn get_square_invalid_test(){
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
    let b = board::Board{grid: grid, calculations: 0};

    assert_eq!(b.get_square(9), vec!{0,0,0,0,0,0,0,0,0});
}

#[test]
fn check_duplicate_test(){
    let grid: Vec<Vec<u8>> = vec!{
        vec!{0,0},
        vec!{0,0}
    };
    let b = board::Board{grid: grid, calculations: 0};

    assert_eq!(b.check_duplicate(), true);

    let grid: Vec<Vec<u8>> = vec!{
        vec!{1,2,3,4},
        vec!{3,4,1,2},
        vec!{2,1,4,3},
        vec!{4,3,2,1}
    };
    let b = board::Board{grid: grid, calculations: 0};

    assert_eq!(b.check_duplicate(), false);
}