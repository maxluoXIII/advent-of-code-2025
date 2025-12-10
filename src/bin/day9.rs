use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::Args;
use clap::Parser;
use nom::{
    IResult, Parser as _, bytes::complete::tag, character::complete::usize,
    sequence::separated_pair,
};

fn parse_tile(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(usize, tag(","), usize).parse(input)
}

fn area(a: (usize, usize), b: (usize, usize)) -> usize {
    let x = if b.0 > a.0 {
        b.0 - a.0 + 1
    } else {
        a.0 - b.0 + 1
    };
    let y = if b.1 > a.1 {
        b.1 - a.1 + 1
    } else {
        a.1 - b.1 + 1
    };
    x * y
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.input).expect("Could not open file");
    let reader = BufReader::new(file);

    let red_tiles: Vec<(usize, usize)> = reader
        .lines()
        .map(|line| {
            parse_tile(&line.expect("Could not read line"))
                .expect("Could not parse tile")
                .1
        })
        .collect();

    let mut biggest_area = 0;
    for i in 0..red_tiles.len() {
        for j in 0..red_tiles.len() {
            if i < j {
                let cur_area = area(red_tiles[i], red_tiles[j]);
                if cur_area > biggest_area {
                    biggest_area = cur_area;
                }
            }
        }
    }

    println!("Biggest rectangle area: {}", biggest_area);
}
