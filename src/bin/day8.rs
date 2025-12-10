use std::{
    cmp::Reverse,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::Args;
use clap::Parser;
use nom::{
    IResult, Parser as _, bytes::complete::tag, character::complete::i64, combinator::map,
    sequence::separated_pair,
};

struct Junction {
    loc: (i64, i64, i64),
    id: usize,
    circuit: usize,
}

fn distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> f64 {
    f64::sqrt(((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)) as f64)
}

fn parse_junction(input: &str, id: usize) -> IResult<&str, Junction> {
    map(
        separated_pair(i64, tag(","), separated_pair(i64, tag(","), i64)),
        |(a, (b, c))| Junction {
            loc: (a, b, c),
            id: id,
            circuit: id,
        },
    )
    .parse(input)
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut junctions: Vec<Junction> = reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            parse_junction(&line.expect("Could not read line"), i)
                .expect("Could not parse line")
                .1
        })
        .collect();

    let mut possible_edges = Vec::new();
    for i in 0..junctions.len() {
        for j in 0..junctions.len() {
            if i != j && i < j {
                possible_edges.push((i, j));
            }
        }
    }
    possible_edges.sort_by(|(a, b), (c, d)| {
        distance(junctions[*a].loc, junctions[*b].loc)
            .total_cmp(&distance(junctions[*c].loc, junctions[*d].loc))
    });

    let mut circuits: Vec<HashSet<usize>> = junctions
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(i);
            new_circuit
        })
        .collect();

    let mut merge_idx = 0;
    // for part 1, stop after 1000 connections
    loop {
        let (a, b) = possible_edges[merge_idx];
        let a_circuit = junctions[a].circuit;
        let b_circuit = junctions[b].circuit;
        if a_circuit != b_circuit {
            circuits[b_circuit]
                .iter()
                .for_each(|ji| junctions[*ji].circuit = a_circuit);
            circuits[a_circuit] = circuits[a_circuit]
                .union(&circuits[b_circuit])
                .map(|&id| id)
                .collect();
            circuits[b_circuit].clear();

            if circuits[a_circuit].len() == junctions.len() {
                println!(
                    "Product of x of last two junctions: {}",
                    junctions[a].loc.0 * junctions[b].loc.0
                );
                break;
            }
        }
        merge_idx += 1;
    }

    // part 1
    // circuits.sort_by_key(|c| Reverse(c.len()));
    // let largest_3_prod: usize = circuits.iter().take(3).map(|c| c.len()).product();
    // println!("largest 3 circuit product: {}", largest_3_prod);
}
