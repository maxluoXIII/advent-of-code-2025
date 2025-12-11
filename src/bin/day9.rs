use std::{
    cmp,
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::Args;
use clap::Parser;
use nom::{
    IResult, Parser as _, bytes::complete::tag, character::complete::usize,
    sequence::separated_pair,
};

#[derive(Clone, PartialEq)]
enum Tile {
    Festive,
    Gray,
}

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

fn is_in_borders(grid: &Vec<Vec<Tile>>, check_point: (usize, usize)) -> bool {
    if grid[check_point.0][check_point.1] == Tile::Festive {
        return false;
    }

    let mut found_border = 0;
    for x in (0..=check_point.0).rev() {
        if grid[x][check_point.1] == Tile::Festive {
            found_border += 1;
            break;
        }
    }

    for x in check_point.0..grid.len() {
        if grid[x][check_point.1] == Tile::Festive {
            found_border += 1;
            break;
        }
    }

    for y in (0..=check_point.1).rev() {
        if grid[check_point.0][y] == Tile::Festive {
            found_border += 1;
            break;
        }
    }

    for y in check_point.1..grid[0].len() {
        if grid[check_point.0][y] == Tile::Festive {
            found_border += 1;
            break;
        }
    }

    found_border == 4
}

fn find_inner_point(grid: &Vec<Vec<Tile>>, red_tiles: &Vec<(usize, usize)>) -> (usize, usize) {
    // Find a point inside the borders
    // There are a bunch of edges cases with shapes that are only two units wide
    // or where the first two points are on an edge, but this
    // should work for this puzzle
    let check_point = (red_tiles[1].0 - 1, red_tiles[1].1 - 1);

    let in_borders = is_in_borders(grid, check_point);

    if in_borders {
        check_point
    } else {
        (red_tiles[1].0 + 1, red_tiles[1].1 + 1)
    }
}

fn fill_from(grid: &mut Vec<Vec<Tile>>, starting_point: (usize, usize)) {
    let mut queue = VecDeque::from([starting_point]);
    let mut queued = HashSet::from([starting_point]);

    // we don't need to check edges because borders should surround our area
    while !queue.is_empty() {
        let check = queue.pop_front().unwrap();

        // left
        if grid[check.0 - 1][check.1] != Tile::Festive && !queued.contains(&(check.0 - 1, check.1))
        {
            queue.push_back((check.0 - 1, check.1));
            queued.insert((check.0 - 1, check.1));
        }

        // up
        if grid[check.0][check.1 - 1] != Tile::Festive && !queued.contains(&(check.0, check.1 - 1))
        {
            queue.push_back((check.0, check.1 - 1));
            queued.insert((check.0, check.1 - 1));
        }

        // right
        if grid[check.0 + 1][check.1] != Tile::Festive && !queued.contains(&(check.0 + 1, check.1))
        {
            queue.push_back((check.0 + 1, check.1));
            queued.insert((check.0 + 1, check.1));
        }

        // down
        if grid[check.0][check.1 + 1] != Tile::Festive && !queued.contains(&(check.0, check.1 + 1))
        {
            queue.push_back((check.0, check.1 + 1));
            queued.insert((check.0, check.1 + 1));
        }

        grid[check.0][check.1] = Tile::Festive;
    }
}

fn get_rect_points(a: (usize, usize), b: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut points = HashSet::new();
    let start_x = cmp::min(a.0, b.0);
    let end_x = cmp::max(a.0, b.0);
    for x in start_x..=end_x {
        points.insert((x, a.1));
        points.insert((x, b.1));
    }

    let start_y = cmp::min(a.1, b.1);
    let end_y = cmp::min(a.1, b.1);
    for y in start_y..=end_y {
        points.insert((a.0, y));
        points.insert((b.0, y));
    }

    points
}

fn all_points_inner(grid: &Vec<Vec<Tile>>, points: &HashSet<(usize, usize)>) -> bool {
    for point in points.iter() {
        if grid[point.0][point.1] != Tile::Festive {
            return false;
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

    let max_x = red_tiles
        .iter()
        .max_by_key(|tile| tile.0)
        .expect("No red tiles")
        .0
        + 1;
    let max_y = red_tiles
        .iter()
        .max_by_key(|tile| tile.1)
        .expect("No red tiles")
        .1
        + 1;

    // Draw borders
    let mut grid = vec![vec![Tile::Gray; max_y]; max_x];
    for (i, red_tile) in red_tiles.iter().enumerate() {
        grid[red_tile.0][red_tile.1] = Tile::Festive;
        let prev_tile = if i > 0 {
            red_tiles[i - 1]
        } else {
            red_tiles[red_tiles.len() - 1]
        };
        let start_x = cmp::min(prev_tile.0, red_tile.0);
        let end_x = cmp::max(prev_tile.0, red_tile.0);
        let start_y = cmp::min(prev_tile.1, red_tile.1);
        let end_y = cmp::max(prev_tile.1, red_tile.1);
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                grid[x][y] = Tile::Festive;
            }
        }
    }

    let starting_point = find_inner_point(&grid, &red_tiles);
    println!("Using {:?} as start for fill", starting_point);
    if grid[starting_point.0][starting_point.1] == Tile::Festive {
        return;
    }
    fill_from(&mut grid, starting_point);

    // for y in 0..grid[0].len() {
    //     for x in 0..grid.len() {
    //         let print_char = if (x, y) == starting_point {
    //             "X"
    //         } else {
    //             match grid[x][y] {
    //                 Tile::Festive => "#",
    //                 Tile::Gray => ".",
    //             }
    //         };
    //         print!("{}", print_char);
    //     }
    //     println!("");
    // }

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
                    && all_points_inner(&grid, &get_rect_points(red_tiles[i], red_tiles[j]))
                {
                    biggest_inner_area = cur_area;
                }
            }
        }
    }

    println!("Biggest rectangle area: {}", biggest_area);
    println!("Biggest inner area: {}", biggest_inner_area);
}
