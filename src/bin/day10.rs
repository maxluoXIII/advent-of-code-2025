use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::Args;
use clap::Parser;
use nom::{
    IResult, Parser as _,
    branch::alt,
    bytes::complete::tag,
    character::complete::{u64, usize},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
};

struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

impl Machine {
    fn min_button_initialize(&self) -> u64 {
        let mut state = vec![false; self.target.len()];
        self.help_min_button_initialize(&mut state, 0)
            .expect("Could not find a solution")
    }

    fn help_min_button_initialize(&self, state: &mut Vec<bool>, button_idx: usize) -> Option<u64> {
        if button_idx >= self.buttons.len() {
            match *state == self.target {
                true => Some(0),
                false => None,
            }
        } else {
            let no_push = self.help_min_button_initialize(state, button_idx + 1);
            self.apply_button_to_state(button_idx, state);
            let push = self.help_min_button_initialize(state, button_idx + 1);
            self.apply_button_to_state(button_idx, state);
            match (push, no_push) {
                (Some(p), Some(n)) => Some(cmp::min(1 + p, n)),
                (Some(p), None) => Some(1 + p),
                (None, Some(n)) => Some(n),
                (None, None) => None,
            }
        }
    }

    fn apply_button_to_state(&self, button_idx: usize, state: &mut Vec<bool>) {
        for light in &self.buttons[button_idx] {
            state[*light] = !state[*light];
        }
    }
}

fn parse_target(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(
        tag("["),
        many1(alt((map(tag("#"), |_| true), map(tag("."), |_| false)))),
        tag("]"),
    )
    .parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(tag("("), separated_list1(tag(","), usize), tag(")")).parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(tag(" "), parse_button).parse(input)
}

fn parse_joltages(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("{"), separated_list1(tag(","), u64), tag("}")).parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, target) = parse_target(input)?;
    let (input, buttons) = preceded(tag(" "), parse_buttons).parse(input)?;
    let (input, joltages) = preceded(tag(" "), parse_joltages).parse(input)?;
    Ok((
        input,
        Machine {
            target: target,
            buttons: buttons,
            joltages: joltages,
        },
    ))
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.input).expect("Could not open file");
    let reader = BufReader::new(file);

    let machines: Vec<Machine> = reader
        .lines()
        .map(|line| {
            parse_machine(&line.expect("Could not read line"))
                .expect("Could not parse line")
                .1
        })
        .collect();

    let button_sum: u64 = machines
        .iter()
        .map(|machine| machine.min_button_initialize())
        .sum();
    println!("Min button presses to initialize: {}", button_sum);
}
