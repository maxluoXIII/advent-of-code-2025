use std::{
    cmp,
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

struct Rect {
    lo_x: usize,
    hi_x: usize,
    lo_y: usize,
    hi_y: usize,
}

impl Rect {
    fn from(a: (usize, usize), b: (usize, usize)) -> Rect {
        Rect {
            lo_x: cmp::min(a.0, b.0),
            hi_x: cmp::max(a.0, b.0),
            lo_y: cmp::min(a.1, b.1),
            hi_y: cmp::max(a.1, b.1),
        }
    }
}

fn is_inner(
    a: (usize, usize),
    b: (usize, usize),
    hori_edges: &Vec<((usize, usize), (usize, usize))>,
    vert_edges: &Vec<((usize, usize), (usize, usize))>,
) -> bool {
    let rect = Rect::from(a, b);

    for ((x1, y), (x2, _)) in hori_edges {
        if rect.lo_y < *y && *y < rect.hi_y {
            if *x1 < rect.lo_x && *x2 > rect.hi_x {
                return false;
            }

            if *x2 > rect.lo_x && *x1 < rect.hi_x {
                return false;
            }
        }
    }

    for ((x, y1), (_, y2)) in vert_edges {
        if rect.lo_x < *x && *x < rect.hi_x {
            if *y1 < rect.lo_y && *y2 > rect.hi_y {
                return false;
            }

            if *y2 > rect.lo_y && *y1 < rect.hi_y {
                return false;
            }
        }
    }

    true
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

    // Get borders
    let mut hori_edges: Vec<((usize, usize), (usize, usize))> = Vec::new();
    let mut vert_edges: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for (i, cur_tile) in red_tiles.iter().enumerate() {
        let next_tile = if i < red_tiles.len() - 1 {
            red_tiles[i + 1]
        } else {
            red_tiles[0]
        };

        if cur_tile.0 == next_tile.0 {
            vert_edges.push((
                (cur_tile.0, cmp::min(cur_tile.1, next_tile.1)),
                (cur_tile.0, cmp::max(cur_tile.1, next_tile.1)),
            ));
        } else {
            hori_edges.push((
                (cmp::min(cur_tile.0, next_tile.0), cur_tile.1),
                (cmp::max(cur_tile.0, next_tile.0), cur_tile.1),
            ));
        }
    }

    let mut biggest_area = 0;
    let mut biggest_inner_area = 0;
    for i in 0..red_tiles.len() {
        for j in 0..red_tiles.len() {
            if i < j {
                let cur_area = area(red_tiles[i], red_tiles[j]);
                if cur_area > biggest_area {
                    biggest_area = cur_area;
                }

                if cur_area > biggest_inner_area
                    && is_inner(red_tiles[i], red_tiles[j], &hori_edges, &vert_edges)
                {
                    biggest_inner_area = cur_area;
                }
            }
        }
    }

    println!("Biggest rectangle area: {}", biggest_area);
    println!("Biggest inner area: {}", biggest_inner_area);
}
