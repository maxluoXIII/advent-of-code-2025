use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::Args;
use clap::Parser;
use nom::{IResult, Parser as _, branch::alt, bytes::complete::tag, combinator::map, multi::many1};

#[derive(Clone, PartialEq, Debug)]
enum SpaceState {
    Empty,
    Start,
    Splitter,
    Beam,
}

#[derive(Default)]
struct Grid {
    grid: Vec<Vec<SpaceState>>,
}

#[derive(Default)]
struct SimulationResult {
    splits: u64,
    end_locs: Vec<usize>,
}

#[derive(Default)]
struct QuantumSimResult {
    ends: u64,
}

impl Grid {
    fn add_row(&mut self, new_row: Vec<SpaceState>) {
        self.grid.push(new_row);
    }

    fn simulate(&self) -> SimulationResult {
        let mut res = SimulationResult::default();
        let mut working_grid = self.grid.clone();

        for y in 0..working_grid.len() {
            for x in 0..working_grid[y].len() {
                // If this is the last row, don't simulate the next one
                if y != working_grid.len() - 1 {
                    match working_grid[y][x] {
                        SpaceState::Start => {
                            // Bug: should handle an immediate split
                            working_grid[y + 1][x] = SpaceState::Beam;
                        }
                        SpaceState::Beam => {
                            if working_grid[y + 1][x] == SpaceState::Splitter {
                                if x != 0 {
                                    working_grid[y + 1][x - 1] = SpaceState::Beam;
                                }
                                if x != working_grid[y].len() - 1 {
                                    working_grid[y + 1][x + 1] = SpaceState::Beam;
                                }
                                res.splits += 1;
                            } else {
                                working_grid[y + 1][x] = SpaceState::Beam;
                            }
                        }
                        SpaceState::Splitter => {}
                        SpaceState::Empty => {}
                    }
                }
            }
        }

        res.end_locs = working_grid[working_grid.len() - 1]
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == SpaceState::Beam)
            .map(|(i, _)| i)
            .collect();

        res
    }

    fn simulate_quantum(&self) -> u64 {
        let mut memo = HashMap::new();
        let start_pos = self.grid[0]
            .iter()
            .enumerate()
            .find_map(|(i, s)| match s {
                SpaceState::Start => Some(i),
                _ => None,
            })
            .expect("Start was not on the first row");
        self.help_sim_quantum((0, start_pos), &mut memo)
    }

    fn help_sim_quantum(
        &self,
        particle_pos: (usize, usize),
        memo: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        let (y, x) = particle_pos;
        if memo.contains_key(&particle_pos) {
            memo[&particle_pos]
        } else if y == self.grid.len() - 1 {
            memo.insert(particle_pos, 1);
            1
        } else {
            let mut res = 0;
            match self.grid[y + 1][x] {
                SpaceState::Empty => {
                    res += self.help_sim_quantum((y + 1, x), memo);
                }
                SpaceState::Splitter => {
                    if x != 0 {
                        res += self.help_sim_quantum((y + 1, x - 1), memo);
                    }
                    if x != self.grid[y].len() - 1 {
                        res += self.help_sim_quantum((y + 1, x + 1), memo);
                    }
                }
                SpaceState::Start => {
                    // this should never happen
                }
                SpaceState::Beam => {
                    // this should never happen
                }
            }

            memo.insert(particle_pos, res);
            res
        }
    }
}

fn parse_space(input: &str) -> IResult<&str, SpaceState> {
    alt((
        map(tag("S"), |_| SpaceState::Start),
        map(tag("."), |_| SpaceState::Empty),
        map(tag("^"), |_| SpaceState::Splitter),
    ))
    .parse(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<SpaceState>> {
    many1(parse_space).parse(input)
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut grid = Grid::default();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let (_, row) = parse_row(&line).expect("Could not parse line");
        grid.add_row(row);
    }

    let sim_res = grid.simulate();
    let quantum_sim = grid.simulate_quantum();
    println!("Splits: {}", sim_res.splits);
    println!("Ends: {}", quantum_sim);
}
