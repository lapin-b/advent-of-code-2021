use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    part1();
}

fn part1(){
    let file = BufReader::new(File::open("day1/src/files/input1.txt").unwrap());

    let nb_of_increased_reading = file
        .lines()
        .map(Result::unwrap)
        .map(|line| i32::from_str(&line).unwrap())
        .fold((None, 0), |prev, depth|{
            let (previous_depth, mut nb_increased) = prev;

            if let Some(prev_depth) = previous_depth {
                if depth > prev_depth {
                    nb_increased += 1;
                }
            }

            (Some(depth), nb_increased)
        }).1;

    println!("Nb of increasing depth: {:?}", nb_of_increased_reading);
}
