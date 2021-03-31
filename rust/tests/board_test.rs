
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
    let b = board::Board{grid: grid, calculations: 0, size: 9};

    assert_eq!(b.get_column(8), vec!{2,2,2,5,5,5,8,8,8});
    assert_eq!(b.get_column(0), vec!{0,0,0,3,3,3,6,6,6});
    assert_eq!(b.get_column(5), vec!{1,1,1,4,4,4,7,7,7});
}

#[test]
#[should_panic]
fn get_column_invalid_test(){
    let grid: Vec<Vec<u8>> = vec!{vec!{1,2,3,4,5,6,7,8,9}, vec!{2,3,4,5,6,7,8,9,10}};
    let b = board::Board{grid: grid, calculations: 0, size: 0};

    b.get_column(100);
}

#[test]
fn get_row_valid_test(){
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
    let b = board::Board{grid: grid, calculations: 0, size: 9};
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
    assert_eq!{b.get_row(0), grid[0]};
    assert_eq!(b.get_row(1), grid[1]);
}

#[test]
#[should_panic]
fn get_row_invalid_test(){
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
    let b = board::Board{grid: grid, calculations: 0, size: 9};

    b.get_row(9);
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
    let b = board::Board{grid: grid, calculations: 0, size: 9};

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
    let b = board::Board{grid: grid, calculations: 0, size: 9};

    assert_eq!(b.get_square(9), vec!{0,0,0,0,0,0,0,0,0});
}

#[test]
fn check_duplicate_valid_test(){
    let grid: Vec<Vec<u8>> = vec!{
        vec!{0,0},
        vec!{0,0}
    };
    let b = board::Board{grid: grid, calculations: 0, size: 2};

    assert_eq!(b.check_duplicate(), true);

    let grid: Vec<Vec<u8>> = vec!{
        vec!{1,2,3,4},
        vec!{3,4,1,2},
        vec!{2,1,4,3},
        vec!{4,3,2,1}
    };
    let b = board::Board{grid: grid, calculations: 0, size: 4};

    assert_eq!(b.check_duplicate(), false);
}

#[test]
fn valid_valid_test(){
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
    let b = board::Board{grid: grid, calculations: 0, size: 9};

    assert_eq!(b.valid(true), true);
    assert_eq!(b.valid(false), false);
}

#[test]
fn possible_valid_test(){
    let grid: Vec<Vec<u8>> = vec!{
        vec!{1,2,3,4},
        vec!{3,0,1,2},
        vec!{4,0,2,0},
        vec!{0,2,0,0}
    };
    let b = board::new(4);
    assert_eq!(b.possible(5), vec!{1,2,3,4});
    let b = board::Board{grid: grid, calculations: 0, size: 4};
    assert_eq!(b.possible(5), vec!{4});
    assert_eq!(b.possible(9), vec!{1, 3});
    assert_eq!(b.possible(12), vec!{});
}

#[test]
fn solve_valid_test_4x4(){
    let grid: Vec<Vec<u8>> = vec!{
        vec!{1,2,3,4},
        vec!{3,0,1,2},
        vec!{4,0,2,0},
        vec!{0,0,0,0}
    };
    let mut b = board::Board{grid: grid, calculations: 0, size: 4};
    assert_eq!(true, b.solve());
    assert_eq!(true, b.solve());
}

#[test]
fn solve_valid_test_9x9(){
    let grid: Vec<Vec<u8>> = vec!{
        vec!{0,0,2,4,0,0,0,9,0},
        vec!{3,0,0,6,9,0,0,0,4},
        vec!{0,0,0,0,0,0,0,5,0},
        vec!{0,0,0,0,0,0,5,7,3},
        vec!{0,0,0,9,0,2,0,0,0},
        vec!{4,1,8,0,0,0,0,0,0},
        vec!{0,3,0,0,0,0,0,0,0},
        vec!{2,0,0,0,8,1,0,0,6},
        vec!{0,7,0,0,0,9,8,0,0}
    };
    let mut b = board::Board{grid: grid, calculations: 0, size: 9};
    println!("{:?}", b.solve());
    b = board::new(9);
    println!("{:?}", b.solve());
}

#[test]
fn solve_valid_test_big(){
    let mut b = board::new(25);
    b.solve();
    println!("{}", b);
}