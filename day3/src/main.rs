use std::cmp::Ordering;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

type CountOnesAndZeroesClosure = dyn FnMut((i32, i32), &Vec<String>) -> (i32, i32);

fn main() {
    let input_filename = args().nth(1).expect("USAGE: day3 <input file>");
    let file = File::open(&input_filename).unwrap_or_else(|_| panic!("Can't open file {}", input_filename));
    let file = BufReader::new(file);

    let lines = file
        .lines()
        .map(Result::unwrap)
        .map(|l: String| l
            .split("")
            .filter(|part| !part.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[Vec<String>]) {
    let columns_count = lines[0].len();
    let mut gamma_number = 0u32;
    let mut epsilon_number = 0u32;

    for column in 0..columns_count {
        let (one_count, zero_count) = lines
            .iter()
            .fold((0, 0), count_ones_and_zeroes(column));

        // Gamma rate: most common bit makes its way into the number
        // Epsilon rate: least common bit makes its way into the number
        if one_count > zero_count {
            gamma_number = (gamma_number << 1) | 1;
            epsilon_number <<= 1;
        } else {
            gamma_number <<= 1;
            epsilon_number = (epsilon_number << 1) | 1;
        }
    }

    println!("Gamma number: {}, Epsilon number: {}, Power consumption rate: {}", gamma_number, epsilon_number, gamma_number * epsilon_number);
}

fn part2(lines: &[Vec<String>]) {
    let o2_generator_rating = bit_criteria_filtering(
        lines,
        |one_count, zero_count| {
            match one_count.cmp(&zero_count) {
                Ordering::Less => "0",
                Ordering::Equal => "1",
                Ordering::Greater => "1"
            }
        }
    );

    let co2_scrubber_rating = bit_criteria_filtering(
        lines,
        |one_count, zero_count| {
            match one_count.cmp(&zero_count) {
                Ordering::Less => "1",
                Ordering::Equal => "0",
                Ordering::Greater => "0"
            }
        }
    );

    println!("O2 rating: {}, CO2 rating: {}, Life support rating: {}", o2_generator_rating, co2_scrubber_rating, o2_generator_rating * co2_scrubber_rating);
}

// Naming is hard
fn bit_criteria_filtering<F>(lines: &[Vec<String>], comparison_function: F) -> i32
    where F: Fn(i32, i32) -> &'static str
{
    let mut lines = lines.iter().collect::<Vec<_>>();

    let mut current_column = 0;
    while lines.len() > 1 {
        let (one_count, zero_count) = lines
            .iter()
            .copied()
            .fold((0, 0), count_ones_and_zeroes(current_column));


        let keep = comparison_function(one_count, zero_count);

        lines = lines
            .iter()
            .filter(|line| line[current_column] == keep)
            .copied()
            .collect::<Vec<_>>();

        current_column += 1;
    }

    i32::from_str_radix(&lines[0].join(""), 2).unwrap()
}

fn count_ones_and_zeroes(column: usize) -> Box<CountOnesAndZeroesClosure> {
    Box::new(move |(one_count, zero_count), bits| {
        let bit = bits[column].as_str();
        match bit {
            "0" => (one_count, zero_count + 1),
            "1" => (one_count + 1, zero_count),
            _ => unreachable!("The numbers should only be ones or zeroes, not anything else in {:?}", bits)
        }
    })
}