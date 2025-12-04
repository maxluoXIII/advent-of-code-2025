use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::Args;
use clap::Parser;

fn is_accessible(grid: &Vec<Vec<bool>>, y: usize, x: usize) -> bool {
    let row = &grid[y];
    if grid[y][x] {
        let mut empty = 0;
        if y == 0 || x == 0 || !grid[y - 1][x - 1] {
            empty += 1;
        }
        if y == 0 || !grid[y - 1][x] {
            empty += 1;
        }
        if y == 0 || x == row.len() - 1 || !grid[y - 1][x + 1] {
            empty += 1;
        }
        if x == 0 || !grid[y][x - 1] {
            empty += 1;
        }
        if x == row.len() - 1 || !grid[y][x + 1] {
            empty += 1;
        }
        if y == grid.len() - 1 || x == 0 || !grid[y + 1][x - 1] {
            empty += 1;
        }
        if y == grid.len() - 1 || !grid[y + 1][x] {
            empty += 1;
        }
        if y == grid.len() - 1 || x == row.len() - 1 || !grid[y + 1][x + 1] {
            empty += 1;
        }

        if empty >= 5 {
            return true;
        }
    }

    false
}

fn count_accessible(grid: &Vec<Vec<bool>>) -> u32 {
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_accessible(grid, y, x) {
                count += 1;
            }
        }
    }

    count
}

fn remove_accessible(grid: &mut Vec<Vec<bool>>) -> u32 {
    let mut count = 0;
    for y in 0..grid.len() {
        let row = &grid[y];
        for x in 0..row.len() {
            if is_accessible(grid, y, x) {
                count += 1;
                grid[y][x] = false;
            }
        }
    }

    count
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        grid.push(Vec::new());
        for c in line.chars() {
            grid.last_mut().unwrap().push(c == '@');
        }
    }

    let accessible_count = count_accessible(&grid);
    let mut total_removed = 0;
    loop {
        let removed = remove_accessible(&mut grid);
        if removed > 0 {
            total_removed += removed;
        } else {
            break;
        }
    }

    println!("Accessible count: {}", accessible_count);
    println!("Total removed: {}", total_removed);
}
