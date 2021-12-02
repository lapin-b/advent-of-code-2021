use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let file = BufReader::new(File::open("day1/src/files/input1.txt").unwrap());
    let lines = file
        .lines()
        .map(Result::unwrap)
        .map(|l| i32::from_str(&l))
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[i32]){
    let nb_of_increased_reading = lines
        .iter()
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

fn part2(lines: &[i32]){
    let (_, nb_increased_readings) = lines.iter()
        .zip(lines.iter().skip(1))
        .zip(lines.iter().skip(2))
        .map(|((a, b), c)| (*a, *b, *c))
        .fold((None, 0), |(prev_sum, nb_increased), (line, follow1, follow2)| {
            let current_sum = line + follow1 + follow2;

            if let Some(prev_sum) = prev_sum {
                if current_sum > prev_sum {
                    return (Some(current_sum), nb_increased + 1);
                }
            }

            (Some(current_sum), nb_increased)
        });

    println!("Increase counts with window of 3: {}", nb_increased_readings);
}