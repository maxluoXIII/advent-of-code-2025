use advent_of_code_2025::Args;
use clap::Parser;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nom::{
    IResult, Parser as _,
    branch::alt,
    bytes::tag,
    character::complete::{space1, u64},
    combinator::map,
    multi::separated_list1,
};

enum MathOp {
    Multiply,
    Add,
}

fn parse_num_row(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64).parse(input)
}

fn parse_math_op(input: &str) -> IResult<&str, MathOp> {
    alt((
        map(tag("+"), |_| MathOp::Add),
        map(tag("*"), |_| MathOp::Multiply),
    ))
    .parse(input)
}

fn parse_op_row(input: &str) -> IResult<&str, Vec<MathOp>> {
    separated_list1(space1, parse_math_op).parse(input)
}

fn parse_col_num(char_grid: &Vec<Vec<char>>, col: usize) -> u64 {
    let mut col_num_str = String::new();
    for i in 0..char_grid.len() - 1 {
        col_num_str.push(char_grid[i][col]);
    }
    col_num_str
        .trim()
        .parse()
        .expect("Unable to parse into u64")
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut num_grid = Vec::new();
    let mut char_grid: Vec<Vec<char>> = Vec::new();
    let mut ops = Vec::new();
    let mut empty_cols = Vec::new();
    let mut first_row = true;
    for line in reader.lines() {
        let raw_line = line.expect("Could not read line");
        let line = raw_line.trim();
        if let Ok((_, num_row)) = parse_num_row(&line) {
            num_grid.push(num_row);
        } else {
            let (_, op_row) = parse_op_row(&line).expect("Could not parse op row");
            ops.extend(op_row);
        }

        if first_row {
            empty_cols = raw_line
                .char_indices()
                .filter(|(_, c)| *c == ' ')
                .map(|(i, _)| i)
                .collect();
        } else {
            for (i, c) in raw_line.char_indices() {
                if c != ' ' && empty_cols.contains(&i) {
                    empty_cols.remove(empty_cols.binary_search(&i).unwrap());
                }
            }
        }
        char_grid.push(raw_line.chars().collect());

        if first_row {
            first_row = false;
        }
    }

    let mut calc_sum = 0;
    for col in 0..num_grid[0].len() {
        let cur_op = &ops[col];
        let mut res = match cur_op {
            MathOp::Multiply => 1,
            MathOp::Add => 0,
        };
        for row in 0..num_grid.len() {
            match cur_op {
                MathOp::Multiply => {
                    res *= num_grid[row][col];
                }
                MathOp::Add => {
                    res += num_grid[row][col];
                }
            }
        }
        calc_sum += res;
    }

    let mut ceph_sum: u64 = 0;
    let mut operands = Vec::new();
    let mut op_num = 0;
    for col in 0..char_grid[0].len() {
        if empty_cols.contains(&col) {
            ceph_sum += match ops[op_num] {
                MathOp::Multiply => operands.iter().product::<u64>(),
                MathOp::Add => operands.iter().sum(),
            };
            op_num += 1;
            operands.clear();
        } else {
            operands.push(parse_col_num(&char_grid, col));
        }
    }

    // Finish the last operation
    ceph_sum += match ops[op_num] {
        MathOp::Multiply => operands.iter().product::<u64>(),
        MathOp::Add => operands.iter().sum(),
    };

    println!("Calc sum: {}", calc_sum);
    println!("Ceph sum: {}", ceph_sum);
}
