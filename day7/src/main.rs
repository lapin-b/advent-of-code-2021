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

    let fuel_req_part1 = calculate_fuel_requirements(&crab_positions, |diff| diff);
    println!("Cheapest fuel requirement for part 1: {}", fuel_req_part1);

    let fuel_req_part2 = calculate_fuel_requirements(&crab_positions, |diff| (1..=diff).sum());
    println!("Cheapest fuel requirement for part 2: {}", fuel_req_part2);
}

fn calculate_fuel_requirements<F>(initial_positions: &[u32], fuel_calculation_fun: F) -> u32
    where F: Fn(u32) -> u32
{
    let largest_initial_position = *initial_positions.iter().max().unwrap();
    let mut minimal_fuel_requirement = u32::MAX;

    for possible_position in 0..=largest_initial_position {
        let fuel_requirement = initial_positions
            .iter()
            .map(|crab_position|{
                let crab_position = *crab_position;

                let difference_in_position = if crab_position > possible_position {
                    crab_position - possible_position
                } else {
                    possible_position - crab_position
                };

                fuel_calculation_fun(difference_in_position)
            })
            .sum::<u32>();

        minimal_fuel_requirement = min(minimal_fuel_requirement, fuel_requirement);
    }

    minimal_fuel_requirement
}
