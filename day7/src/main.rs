use std::cmp::min;
use std::env::args;
use std::str::FromStr;

fn main() {
    let filename = args().nth(1).expect("USAGE: day7 <input file>");
    let content = std::fs::read_to_string(filename).unwrap();

    let crab_positions = content
        .split(',')
        .map(|pos| u32::from_str(pos).unwrap())
        .collect::<Vec<_>>();

    part1(&crab_positions);
}

fn part1(positions: &[u32]) {
    let max_position = *positions.iter().max().unwrap();
    let mut cheapest_fuel_qty = u32::MAX;

    for possible_position in 0..=max_position {
        let fuel = positions
            .iter()
            .map(|position| if *position > possible_position { position - possible_position } else { possible_position - position })
            .sum::<u32>();

        cheapest_fuel_qty = min(cheapest_fuel_qty, fuel);
    }

    println!("Cheapest fuel qty for everybody to align: {}", cheapest_fuel_qty);
}
