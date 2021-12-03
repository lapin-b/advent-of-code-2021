use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_filename = args().nth(1).expect("USAGE: day3 <input file>");
    let file = File::open(&input_filename).unwrap_or_else(|_| panic!("Can't open file {}", input_filename));
    let file = BufReader::new(file);

    let lines = file
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    part1(&lines);
}

fn part1(lines: &[String]){
    let prepared_lines = lines
        .iter()
        .map(|l| l.split("").filter(|c| !c.is_empty()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let columns_count = prepared_lines[0].len();
    let mut gamma_number = String::with_capacity(columns_count);
    let mut epsilon_number = String::with_capacity(columns_count);

    for column in 0..columns_count {
        let mut one_count = 0;
        let mut zero_count = 0;

        for diagnostic_line in &prepared_lines {
            let bit = diagnostic_line[column];
            match bit {
                "0" => zero_count += 1,
                "1" => one_count += 1,
                _ => unreachable!("The numbers should only be ones or zeroes, not anything else in {:?}", diagnostic_line)
            }
        }

        // Gamma rate: most common bit makes its way into the number
        // Epsilon rate: least common bit makes its way into the number
        if one_count > zero_count {
            gamma_number += "1";
            epsilon_number += "0";
        } else {
            gamma_number += "0";
            epsilon_number += "1";
        }
    }

    let gamma_number = u32::from_str_radix(&gamma_number, 2).unwrap();
    let epsilon_number = u32::from_str_radix(&epsilon_number, 2).unwrap();

    println!("Gamma number: {}, Epsilon number: {}, Power consumption rate: {}", gamma_number, epsilon_number, gamma_number * epsilon_number);
}