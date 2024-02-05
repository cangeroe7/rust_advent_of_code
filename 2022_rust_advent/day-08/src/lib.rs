pub fn process_part1(input: &str) -> String {
    fn check_up(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool{
        if row == 0 { return true }
        if num <= grid[row-1][col] { false } else {check_up(&grid, num, row-1, col)} 
    }
    
    fn check_down(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize, max_length: usize) -> bool {
        if row == max_length { return true };
        if num <= grid[row+1][col] { false } else {check_down(&grid, num, row+1, col, max_length)}
    }
    
    fn check_left(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool {
        if col == 0 { return true };
        if num <= grid[row][col-1] { false } else {check_left(&grid, num, row, col-1)} 
    }
    
    fn check_right(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize, max_width: usize) -> bool{
        if col == max_width { return true };
        if num <= grid[row][col+1] { false } else {check_right(&grid, num, row, col+1, max_width)} 
    }
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|kar| kar.to_digit(10).unwrap())
                .collect()
        }).collect();

    let length = grid.len()-1;
    let width = grid[0].len()-1;
    let mut visible = length*2 + width*2;

    for row in 1..length {
        for col in 1..width {
            let num = grid[row][col];
            visible += (
                check_up(&grid, num, row, col) ||
                check_down(&grid, num, row, col, length) ||
                check_left(&grid, num, row, col) ||
                check_right(&grid, num, row, col, width)
            ) as usize;  
        }
    }
    visible.to_string()
}

use std::cmp::max;
pub fn process_part2(input: &str) -> String {

    fn check_up(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize, views: usize) -> usize {
        if row == 0 || grid[row][col] >= num { return views };
        check_up(grid, num, row-1, col, views+1)
    }

    fn check_down(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize, length: usize, views: usize) -> usize {
        if row == length || grid[row][col] >= num { return views };
        check_down(grid, num, row+1, col, length, views+1)
    }

    fn check_left(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize, views: usize) -> usize {
        if col == 0 || grid[row][col] >= num { return views };
        check_left(grid, num, row, col-1, views+1)
    }

    fn check_right(grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize, width: usize, views: usize) -> usize {
        if col == width || grid[row][col] >= num { return views };
        check_right(grid, num, row, col+1, width, views+1)
    }
    
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|kar| kar.to_digit(10).unwrap())
                .collect()
        }).collect();

    let length = grid.len()-1;
    let width = grid[0].len()-1;
    let mut biggest = 0;

    for row in 1..length {
        for col in 1..width {
            let num = grid[row][col];
            biggest = max(biggest, 
                check_up(&grid, num, row-1, col, 1) *
                check_down(&grid, num, row+1, col, length, 1) *
                check_left(&grid, num, row, col-1, 1) *
                check_right(&grid, num, row, col+1, width, 1)
            );
        }
    }
    biggest.to_string()
}