use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let input_filename = args().skip(1).next().expect("USAGE: day1 <input file>");
    let file = File::open(&input_filename).unwrap_or_else(|_| panic!("Can't open file {}", input_filename));
    let file = BufReader::new(file);

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
    let (_, nb_increased_readings) = lines
        .iter()
        .fold((None, 0), |(previous_depth, nb_increased), depth|{
            if let Some(prev_depth) = previous_depth {
                if depth > prev_depth {
                    return (Some(depth), nb_increased + 1);
                }
            }

            (Some(depth), nb_increased)
        });

    println!("Nb of increasing depth: {}", nb_increased_readings);
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