use std::env::args;
use std::str::FromStr;

const PART1_ITERATION_COUNT: u32 = 80;

fn main() {
    let filename = args().nth(1).expect("USAGE: day6 <input file>");
    let content = std::fs::read_to_string(filename).unwrap();

    let lanternfish_school = content.split(',')
        .map(|fish| u8::from_str(fish))
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    lantern_fish_simulation(&lanternfish_school, PART1_ITERATION_COUNT);
}

fn lantern_fish_simulation(lanternfishes: &[u8], iterations_count: u32){
    let mut lanternfishes = lanternfishes.to_vec();

    println!("Initial state: {:?}", lanternfishes);

    for _iteration in 1..=iterations_count {
        let mut fishes_to_add = 0;

        lanternfishes = lanternfishes
            .iter()
            .map(|fish_timer| if *fish_timer == 0 { fishes_to_add += 1; 6 } else { fish_timer - 1 } )
            .collect::<Vec<u8>>();

        for _ in 0..fishes_to_add {
            lanternfishes.push(8)
        }

        //println!("End of iteration {}: {:?}", iteration, lanternfishes);
    }

    println!("Fish count: {} after {} iterations", lanternfishes.len(), iterations_count);
}
