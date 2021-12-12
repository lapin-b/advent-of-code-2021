mod heightmap;
mod bassin;

use std::env::args;
use std::fs;
use std::ops::Mul;

use heightmap::HeightMap;
use crate::bassin::Bassin;

fn main() {
    let filename = args().nth(1).expect("USAGE: day9 <input file>");
    let file_content = fs::read_to_string(filename).expect("Files does not exist");

    let map = HeightMap::new(&file_content);
    let low_points = part1(&map);
    part2(&low_points, &map);
}

fn part1(map: &HeightMap) -> Vec<(usize, usize)> {
    let mut sum_risk_points = 0;
    let mut low_points = Vec::new();

    for y in 0..map.lines_count() {
        for x in 0..map.columns_count() {
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

fn part2(low_points: &[(usize, usize)], map: &HeightMap) {
    let mut bassins = low_points
        .iter()
        .map(|low_point| Bassin::new(map, *low_point))
        .collect::<Vec<_>>();

    for bassin in bassins.iter_mut() {
        bassin.discover_terrain();
    }

    let mut largest_sizes = bassins.iter()
        .map(|bassin| bassin.terrain_size())
        .collect::<Vec<_>>();

    largest_sizes.sort_by(|a, b| b.cmp(a));

    let largest_sizes_score = largest_sizes.iter().take(3).fold(1, |acc, cur| acc.mul(*cur));
    println!("Largest three bassins score: {}", largest_sizes_score);
}
