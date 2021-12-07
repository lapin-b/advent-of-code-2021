use std::env::args;
use std::str::FromStr;

const ITERATION_COUNT: u32 = 256;
const GROWTH_STATE_COUNT: usize = 9; // 0 to 8 incl.
const AFTER_NEW_FISH_PLACE: usize = 6; // Stage number

fn main() {
    let filename = args().nth(1).expect("USAGE: day6 <input file>");
    let content = std::fs::read_to_string(filename).unwrap();

    let lanternfish_array = content.split(',')
        .map(|fish| usize::from_str(fish))
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    let mut lanternfish_groups: Vec<u64> = vec![0; GROWTH_STATE_COUNT];

    for lanternfish_timer in &lanternfish_array {
        lanternfish_groups[*lanternfish_timer] += 1;
    }

    // Now the simulation of the fishes
    for iteration in 1..=ITERATION_COUNT {
        // Add the fishes from stage zero into the latest stage possible
        let stage_zero = lanternfish_groups.remove(0);
        lanternfish_groups.push(stage_zero);

        // The fishes we have taken from stage zero by poping the vector still exist !
        lanternfish_groups[AFTER_NEW_FISH_PLACE] += stage_zero;

        let fishes_count = lanternfish_groups.iter().sum::<u64>();
        println!("Iteration {}: groups: {:?}; count: {}", iteration, lanternfish_groups, fishes_count);
    }
}
