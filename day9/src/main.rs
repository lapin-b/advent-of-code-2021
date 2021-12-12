use std::env::args;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct HeightMap {
    pub map: Vec<u16>,
    lines_count: usize,
    columns_count: usize,
}

impl HeightMap {
    pub fn new(map: &str) -> Self {
        let lines = map.lines().map(str::to_string).collect::<Vec<String>>();
        let lines_count = lines.len();
        let columns_count = lines.get(0).map(|col| col.len()).unwrap_or(0);

        if lines_count == 0 || columns_count == 0 {
            // Too lazy to do proper error handling
            panic!("Columns and lines count should be more than one");
        }

        let individual_digits = lines
            .iter()
            .map(|line| line.split(""))
            .flatten()
            .filter(|digit| !digit.is_empty())
            .map(u16::from_str)
            .map(Result::unwrap)
            .collect::<Vec<u16>>();

        Self {
            map: individual_digits,
            lines_count,
            columns_count
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u16> {
        if x > self.columns_count - 1 || y > self.lines_count - 1 {
            return None;
        }

        // Y is the line, X the column
        self.map.get(self.calculate_index(x, y)).copied()
    }

    pub fn get_signed(&self, x: isize, y: isize) -> Option<u16> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get(x as usize, y as usize)
        }
    }

    fn calculate_index(&self, x: usize, y: usize) -> usize {
        y * self.columns_count + x
    }
}

fn main() {
    let filename = args().nth(1).expect("USAGE: day9 <input file>");
    let file_content = fs::read_to_string(filename).expect("Files does not exist");

    let map = HeightMap::new(&file_content);
    let low_points = part1(&map);
}

fn part1(map: &HeightMap) -> Vec<(usize, usize)> {
    let mut sum_risk_points = 0;
    let mut low_points = Vec::new();

    for y in 0..map.lines_count {
        for x in 0..map.columns_count {
            let current_depth = map.get(x, y).unwrap_or_else(|| panic!("The point ({}, {}) should exist", x, y));

            let up_shallower = map.get_signed(x as isize, y as isize - 1).map(|d| d > current_depth).unwrap_or(true);
            let down_shallower = map.get_signed(x as isize, y as isize + 1).map(|d| d > current_depth).unwrap_or(true);
            let left_shallower = map.get_signed(x as isize - 1, y as isize).map(|d| d > current_depth).unwrap_or(true);
            let right_shallower = map.get_signed(x as isize + 1, y as isize).map(|d| d > current_depth).unwrap_or(true);

            if up_shallower && down_shallower && left_shallower && right_shallower {
                sum_risk_points += current_depth + 1;
                low_points.push((x, y));
            }
        }
    }

    println!("Sum of risk level of all low points on the map: {}", sum_risk_points);
    low_points
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::HeightMap;

    macro_rules! fetch_data_test {
        ($name:ident, ($x:literal, $y:literal) -> $result:expr) => {
            #[test]
            fn $name(){
                let map = get_map();
                assert_eq!(map.get($x, $y), $result);
            }
        };
    }

    macro_rules! coords_test {
        ($name: ident, ($x:literal, $y:literal) -> $result:literal) => {
            #[test]
            fn $name(){
                let map = get_map();
                assert_eq!(map.calculate_index($x, $y), $result);
            }
        };
    }

    fn get_map() -> HeightMap {
        let file_content = fs::read_to_string("../files/day9/example.txt").expect("Files does not exist");
        HeightMap::new(&file_content)
    }

    coords_test!(idx_zero_zero, (0, 0) -> 0);
    coords_test!(idx_one_zero, (1, 0) -> 1);
    coords_test!(idx_zero_one, (0, 1) -> 10);
    coords_test!(idx_one_one, (1, 1) -> 11);
    coords_test!(idx_one_two, (1, 2) -> 21);
    coords_test!(idx_two_two, (2, 2) -> 22);

    fetch_data_test!(get_zero_zero, (0, 0) -> Some(2));
    fetch_data_test!(get_zero_eol_x, (9, 0) -> Some(0));
    fetch_data_test!(get_zero_eol_y, (0, 4) -> Some(9));
    fetch_data_test!(get_somewhere_grid, (3, 1) -> Some(7));
    fetch_data_test!(get_out_of_bounds_x, (10, 0) -> None);
    fetch_data_test!(get_out_of_bounds_y, (0, 5) -> None);
}
