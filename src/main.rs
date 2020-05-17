use std::fs::File;
use std::io::prelude::*;

struct ValidChecker {
    map: u32,
    is_valid: bool,
}

impl ValidChecker {
    fn new() -> ValidChecker {
        ValidChecker {
            map: 0,
            is_valid: true,
        }
    }

    fn is_valid(self) -> bool {
        self.is_valid
    }

    fn add(&mut self, num: &u32) {
        if *num != 0 && self.is_valid {
            let new_map = self.map | (1 << *num - 1);
            if self.map == new_map {
                self.is_valid = false;
            } else {
                self.map = new_map;
            }
        }
    }
}

fn is_row_valid(puzzle: &Vec<Vec<u32>>, row_num: usize) -> bool {
    let row = &puzzle[row_num];
    let mut checker = ValidChecker::new();

    for value in row {
        checker.add(value)
    }

    return checker.is_valid();
}

fn is_column_valid(puzzle: &Vec<Vec<u32>>, col_num: usize) -> bool {
    let mut checker = ValidChecker::new();

    for row in puzzle {
        checker.add(&row[col_num]);
    }

    return checker.is_valid();
}

fn is_square_valid(puzzle: &Vec<Vec<u32>>, col_num: usize, row_num: usize) -> bool {
    let x_start = col_num - (col_num % 3);
    let y_start = row_num - (row_num % 3);
    let mut checker = ValidChecker::new();

    for y in y_start..(y_start + 3) {
        for x in x_start..(x_start + 3) {
            checker.add(&puzzle[y][x]);
        }
    }

    return checker.is_valid();
}

fn load_puzzle() -> Vec<Vec<u32>> {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut puzzle = vec![vec![0; 9]; 9];
    let mut pointer: usize = 0;
    let mut index: usize = 0;
    for c in input.chars() {
        if c == '\n' {
            pointer += 1;
            index = 0;
        } else {
            puzzle[pointer][index] = u32::from(c) - 48;
            index += 1;
        }
    }
    return puzzle;
}

fn solve(puzzle: &mut Vec<Vec<u32>>, cell: usize) -> bool {
    let row_num = cell / 9;
    let col_num = cell % 9;
    let currently_valid = is_row_valid(puzzle, row_num)
        && is_column_valid(puzzle, col_num)
        && is_square_valid(puzzle, col_num, row_num);

    if !currently_valid {
        return false;
    }

    for new_cell in cell..(9 * 9) {
        let y = new_cell / 9;
        let x = new_cell % 9;

        if puzzle[y][x] != 0 {
            continue;
        }

        for n in 1..=9 {
            puzzle[y][x] = n;
            if solve(puzzle, y * 9 + x) {
                return true;
            }
            puzzle[y][x] = 0;
        }

        if puzzle[y][x] == 0 {
            return false;
        }
    }

    return currently_valid;
}

fn print_puzzle(puzzle: &Vec<Vec<u32>>) {
    for row in puzzle {
        for n in row {
            print!("{}", n);
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut puzzle = load_puzzle();
    print_puzzle(&puzzle);
    solve(&mut puzzle, 0);
    print_puzzle(&puzzle);
}
