use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

#[derive(Debug)]
enum DirectionError {
    NotEnoughItems,
    IntError(ParseIntError),
    UnknownDirection(String)
}

impl From<ParseIntError> for DirectionError {
    fn from(e: ParseIntError) -> Self {
        Self::IntError(e)
    }
}

impl FromStr for Direction {
    type Err = DirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces = s.split(' ').collect::<Vec<_>>();

        if pieces.len() < 2 {
            return Err(DirectionError::NotEnoughItems);
        }

        let direction = pieces[0];
        let quantity = u32::from_str(pieces[1])?;

        Ok(match direction {
            "forward" => Direction::Forward(quantity),
            "up" => Direction::Up(quantity),
            "down" => Direction::Down(quantity),
            _ => return Err(DirectionError::UnknownDirection(direction.to_string()))
        })
    }
}

fn main(){
    let file = BufReader::new(File::open("files/day2/input.txt").unwrap());
    let lines = file.lines()
        .map(Result::unwrap)
        .map(|line| Direction::from_str(&line))
        .map(Result::unwrap)
        .collect::<Vec<Direction>>();

    part1(&lines);
}

// Part 1: calculate horizontal and vertical movement
fn part1(lines: &[Direction]){
    let (horizontal, depth) = lines
        .iter()
        .fold((0, 0), |acc, direction|{
            let direction = *direction;

            match direction {
                Direction::Forward(q) => (acc.0 + q, acc.1),
                Direction::Up(q) => (acc.0, acc.1 - q),
                Direction::Down(q) => (acc.0, acc.1 + q)
            }
        });

    println!("Part 1: Horizontal: {}, Depth: {}, Result: {}", horizontal, depth, horizontal * depth);
}
