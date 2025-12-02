use advent_of_code_2025::Args;
use clap::Parser;
use nom::{
    IResult, Parser as nomParser, branch::alt, bytes::complete::tag, character::complete::u32,
    combinator::map, sequence::preceded,
};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum LockAction {
    TurnRight(u32),
    TurnLeft(u32),
}

fn parse_lock_action(input: &str) -> IResult<&str, LockAction> {
    alt((
        map(preceded(tag("L"), u32), LockAction::TurnLeft),
        map(preceded(tag("R"), u32), LockAction::TurnRight),
    ))
    .parse(input)
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut pos = 50;
    let mut zero_count = 0;
    let mut zero_pass_count = 0;
    for line in reader.lines() {
        let line = line.expect("Could not get line");
        let start_pos = pos;
        match parse_lock_action(&line) {
            Ok((_, LockAction::TurnLeft(l))) => {
                zero_pass_count += l / 100;
                let l = l % 100;
                if l >= pos {
                    pos = 100 - (l - pos);
                    if start_pos != 0 {
                        zero_pass_count += 1;
                    }
                    pos = pos % 100;
                } else {
                    pos -= l;
                }
            }
            Ok((_, LockAction::TurnRight(r))) => {
                zero_pass_count += r / 100;
                let r = r % 100;
                pos = pos + r;
                zero_pass_count += pos / 100;
                pos = pos % 100;
            }
            Err(e) => {
                eprintln!("Could not parse line {} because {}", line, e)
            }
        }
        if pos == 0 {
            zero_count += 1;
        }
    }

    println!("Zero count: {}", zero_count);
    println!("Zero pass count: {}", zero_pass_count);
}
