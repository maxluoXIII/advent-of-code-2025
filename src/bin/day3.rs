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

fn find_two_digit_joltage(bank: &Vec<u32>) -> u32 {
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

fn find_n_digit_joltage(bank: &Vec<u32>, n: usize) -> u64 {
    let mut digits = vec![0; n];
    let mut last_taken_idx = 0;
    for (i, e) in digits.iter_mut().enumerate() {
        let mut biggest = 0;
        let start = if i == 0 { i } else { last_taken_idx + 1 };
        for j in start..(bank.len() - (n - i - 1)) {
            if bank[j] > biggest {
                biggest = bank[j];
                last_taken_idx = j;
            }
        }
        *e = biggest;
    }

    let mut joltage = 0;
    for (i, digit) in digits.iter().enumerate() {
        joltage += (10u64.pow((n - i - 1) as u32)) * (*digit as u64);
    }
    joltage
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut two_digit_joltage = 0;
    let mut twelve_digit_joltage = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let (_, bank) = parse_battery_bank(&line).expect("Could not parse line");
        two_digit_joltage += find_two_digit_joltage(&bank);
        twelve_digit_joltage += find_n_digit_joltage(&bank, 12);
    }

    println!("two digit joltage: {}", two_digit_joltage);
    println!("twelve digit joltage: {}", twelve_digit_joltage);
}
