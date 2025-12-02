use advent_of_code_2025::Args;
use clap::Parser;
use nom::{
    IResult, Parser as _, bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::separated_pair,
};

use std::{
    fs::File,
    io::{BufReader, Read},
};

fn parse_range(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(u64, tag("-"), u64).parse(input)
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(tag(","), parse_range).parse(input)
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let mut reader = BufReader::new(input_file);

    let mut range_str = String::new();
    reader
        .read_to_string(&mut range_str)
        .expect("Could not read file");
    let (_, ranges) = parse_ranges(&range_str).expect("Could not parse ranges");

    let mut repeat_sum = 0;
    let mut multi_sum = 0;
    for (start, end) in ranges {
        for i in start..=end {
            let num_str = i.to_string();
            if num_str[..num_str.len() / 2] == num_str[num_str.len() / 2..] {
                repeat_sum += i;
            }

            let mut multi = false;
            for jump in 1..=num_str.len() / 2 {
                if num_str.len() % jump == 0 {
                    let mut jump_multi = true;
                    for j in 1..num_str.len() / jump {
                        if num_str[0..jump] != num_str[j * jump..j * jump + jump] {
                            jump_multi = false;
                            break;
                        }
                    }
                    if jump_multi {
                        multi = true;
                        break;
                    }
                }
            }
            if multi {
                multi_sum += i;
            }
        }
    }

    println!("Repeat sum: {}", repeat_sum);
    println!("Multi sum: {}", multi_sum);
}
