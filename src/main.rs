use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

type Board = [[u32; 9]; 9];

struct ValidChecker {
    map: [bool; 9],
}

impl ValidChecker {
    fn new() -> Self {
        Self { map: [false; 9] }
    }

    fn add(&mut self, num: u32) -> Result<(), ()> {
        if num != 0 {
            let value = self
                .map
                .get_mut(num as usize - 1)
                .expect("checker out of bounds");

            if *value {
                return Err(());
            }

            *value = true;
        }

        Ok(())
    }
}

fn is_row_valid(puzzle: &Board, row_num: usize) -> Result<(), ()> {
    let row = &puzzle[row_num];
    let mut checker = ValidChecker::new();

    for value in row {
        checker.add(*value)?
    }

    Ok(())
}

fn is_column_valid(puzzle: &Board, col_num: usize) -> Result<(), ()> {
    let mut checker = ValidChecker::new();

    for row in puzzle {
        checker.add(row[col_num])?
    }

    Ok(())
}

fn is_square_valid(puzzle: &Board, col_num: usize, row_num: usize) -> Result<(), ()> {
    let x_start = col_num - (col_num % 3);
    let y_start = row_num - (row_num % 3);
    let mut checker = ValidChecker::new();

    for y in y_start..(y_start + 3) {
        for x in x_start..(x_start + 3) {
            checker.add(puzzle[y][x])?
        }
    }

    Ok(())
}

fn load_puzzle() -> Result<Board, std::io::Error> {
    let mut input = String::new();
    File::open("input.txt")?.read_to_string(&mut input)?;
    let mut puzzle = [[0; 9]; 9];
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            puzzle[row][col] = c.to_digit(10).expect("failed to parse digit");
        }
    }

    Ok(puzzle)
}

fn solve(puzzle: &mut Board, row_num: usize, col_num: usize) -> Result<(), ()> {
    is_row_valid(puzzle, row_num)?;
    is_column_valid(puzzle, col_num)?;
    is_square_valid(puzzle, col_num, row_num)?;

    for y in 0..9 {
        for x in 0..9 {
            if puzzle[y][x] != 0 {
                continue;
            }

            for n in 1..=9 {
                puzzle[y][x] = n;
                if solve(puzzle, y, x).is_ok() {
                    return Ok(());
                }
                puzzle[y][x] = 0;
            }

            if puzzle[y][x] == 0 {
                return Err(());
            }
        }
    }

    Ok(())
}

fn print_puzzle(puzzle: &Board) {
    for row in puzzle {
        for n in row {
            print!("{}", n);
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut puzzle = load_puzzle().expect("failed to parse puzzle");
    print_puzzle(&puzzle);
    solve(&mut puzzle, 0, 0).expect("failed to solve puzzle");
    print_puzzle(&puzzle);
}
