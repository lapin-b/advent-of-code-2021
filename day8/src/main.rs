use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = args().nth(1).expect("USAGE: day8 <input file>");
    let file = File::open(&filename).expect("File does not exist");
    let file = BufReader::new(file);

    let lines = file
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.split('|')
            .filter(|p| !p.is_empty())
            .map(str::trim)
            .map(str::to_string)
            .collect::<Vec<String>>()
        )
        .collect::<Vec<_>>();

    part1(&lines);
}

fn part1(notes: &[Vec<String>]) {
    let outputs = notes
        .iter()
        .map(|line| line[1].clone())
        .collect::<Vec<_>>();

    let numbers_count = outputs
        .iter()
        .map(|out_line| out_line.split(" "))
        .flatten()
        .filter(|output_segments| match output_segments.len() {
            2 | 3 | 4 | 7 => true,
            _ => false
        })
        .count();

    println!("Count of numbers with 2, 3, 4 and 7 digits: {:?}", numbers_count);

}