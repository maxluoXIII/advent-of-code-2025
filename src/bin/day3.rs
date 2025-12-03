use advent_of_code_2025::Args;
use clap::Parser;
use nom::{IResult, Parser as _, bytes::complete::take, combinator::map, multi::many1};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_battery_bank(input: &str) -> IResult<&str, Vec<u32>> {
    many1(map(take(1usize), |d: &str| d.parse::<u32>().unwrap())).parse(input)
}

fn find_max_joltage(bank: Vec<u32>) -> u32 {
    let mut first = 0;
    let mut second = 0;
    for (i, battery) in bank.iter().enumerate() {
        if *battery > first && i != bank.len() - 1 {
            first = *battery;
            second = 0;
        } else if *battery > second {
            second = *battery;
        }
    }

    first * 10 + second
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut total_joltage = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let (_, bank) = parse_battery_bank(&line).expect("Could not parse line");
        total_joltage += find_max_joltage(bank);
    }

    println!("Total joltage: {}", total_joltage);
}
