use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::Args;
use clap::Parser;

fn count_accessible(grid: &Vec<Vec<bool>>) -> u32 {
    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, is_pt) in row.iter().enumerate() {
            if *is_pt {
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
                    count += 1;
                }
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

    println!("Accessible count: {}", accessible_count);
}
