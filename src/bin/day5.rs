use advent_of_code_2025::Args;
use clap::Parser;

use nom::{
    IResult, Parser as _, bytes::complete::tag, character::complete::u64, combinator::map,
    sequence::separated_pair,
};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn parse_fresh_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(separated_pair(u64, tag("-"), u64), |(a, b)| a..=b).parse(input)
}

fn ranges_overlap(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

/// Ranges should be guaranteed to be overlapping before calling this function
fn union_ranges(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    let start = if r1.start() < r2.start() {
        r1.start().clone()
    } else {
        r2.start().clone()
    };
    let end = if r1.end() > r2.end() {
        r1.end().clone()
    } else {
        r2.end().clone()
    };

    start..=end
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut reading_fresh_ranges = true;
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");

        if reading_fresh_ranges {
            if line == "" {
                reading_fresh_ranges = false;
                continue;
            }
            let (_, mut new_range) = parse_fresh_range(&line).expect("Could not parse fresh range");
            let mut i = 0;
            while i < ranges.len() {
                if ranges_overlap(&ranges[i], &new_range) {
                    new_range = union_ranges(&ranges[i], &new_range);
                    ranges.swap_remove(i);
                } else {
                    i += 1;
                }
            }
            ranges.push(new_range.clone());
        } else {
            let ingredient = line.parse::<u64>().expect("Could not parse ingredient");
            ingredients.push(ingredient);
        }
    }

    let mut fresh_ingredient_count = 0;
    for ingredient in ingredients {
        for range in &ranges {
            if range.contains(&ingredient) {
                fresh_ingredient_count += 1;
                break;
            }
        }
    }

    let mut fresh_count = 0;
    for range in ranges {
        fresh_count += range.count();
    }

    println!("Fresh ingredient count: {}", fresh_ingredient_count);
    println!("Fresh count: {}", fresh_count);
}
