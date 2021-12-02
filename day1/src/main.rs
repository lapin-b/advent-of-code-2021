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
    let mut previous_sum = None;
    let mut increase_count = 0;

    for (i, line) in lines.iter().enumerate() {
        let line = *line;
        let line_follow_1 = lines.get(i + 1).copied();
        let line_follow_2 = lines.get(i + 2).copied();

        if line_follow_1.is_none() || line_follow_2.is_none(){
            break;
        }

        let line_follow_1 = line_follow_1.unwrap();
        let line_follow_2 = line_follow_2.unwrap();

        let sum = line + line_follow_1 + line_follow_2;

        if let Some(prev_sum) = previous_sum {
            if sum > prev_sum {
                increase_count += 1;
            }
        }

        previous_sum = Some(sum);
    }

    println!("Increase counts with window of 3: {}", increase_count);
}