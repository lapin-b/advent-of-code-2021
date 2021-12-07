use std::cmp::{max, Ordering};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    pub start: Point,
    pub end: Point,
}

impl Point {
    pub fn from_string(s: &str) -> Self {
        let points = s
            .split(',')
            .map(u32::from_str)
            .collect::<Result<Vec<u32>, _>>()
            .unwrap();

        Point {
            x: points[0],
            y: points[1],
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y {
            return Some(Ordering::Equal);
        }

        if self.x < other.x {
            return if self.y < other.y {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        }

        if self.y < other.y {
            return if self.x < other.x {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        }

        None
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Line {
    pub fn from_slice(points: &[Point]) -> Self {
        let mut p1 = points[0];
        let mut p2 = points[1];

        if p2 > p1 {
            std::mem::swap(&mut p1, &mut p2);
        }

        Self {
            start: p1,
            end: p2,
        }
    }

    pub fn has_point_in_line(&self, point: Point) -> bool {
        // One point is included in a horizontal line if the point's x coordinate match and y coordinate
        // are in the range. For line (0; 5) -> (5; 5), the point (3; 5) would be present while (8;5) is not,
        // nor is (1; 3).

        // One point is included in a vertical line if the point's y coordinate match and the x coordinate
        // are in the range. For line (0; 0) -> (0; 10), the point (0; 5) is in the line while (0; 11) is not,
        // nor is (1; 5).

        (point.y == self.start.y && point.y == self.end.y && point.x >= self.start.x && point.x <= self.end.x) ||
            (point.x == self.start.x && point.x == self.end.x && point.y >= self.start.y && point.y <= self.end.y)
    }
}

fn main() {
    // Read the file and parse the contents of it so we can extract the numbers and the boards
    let filename = args().nth(1).expect("USAGE: day5 <input file>");
    let file = File::open(&filename).expect("File does not exist");
    let file = BufReader::new(file);

    let hydrothermal_vent_lines = file
        .lines()
        .map(Result::unwrap)
        .map(|line: String| line
            .split("->")
            .map(str::trim)
            .map(Point::from_string)
            .collect::<Vec<_>>())
        .map(|points| Line::from_slice(&points))
        .collect::<Vec<_>>();

    part1(&hydrothermal_vent_lines)
}

fn part1(vents: &[Line]) {
    let vertical_horizontal_vents = vents
        .iter()
        .filter(|vent_line| vent_line.start.x == vent_line.end.x || vent_line.start.y == vent_line.end.y)
        .copied()
        .collect::<Vec<_>>();

    let max_x = vertical_horizontal_vents.iter().map(|l| max(l.start.x, l.end.x)).max().unwrap();
    let max_y = vertical_horizontal_vents.iter().map(|l| max(l.start.y, l.end.y)).max().unwrap();

    let mut intersections = 0u32;
    for x in 0..=max_x {
        for y in 0..=max_y {
            let point = Point { x, y };
            let lines_crossing = vertical_horizontal_vents
                .iter()
                .filter(|l| l.has_point_in_line(point))
                .count();

            if lines_crossing > 1 {
                intersections += 1;
            }
        }
    }

    println!("Intersections: {}", intersections);
}

fn _print_map(vents: &[Line]) {
    let max_x = vents.iter().map(|l| max(l.start.x, l.end.x)).max().unwrap();
    let max_y = vents.iter().map(|l| max(l.start.y, l.end.y)).max().unwrap();

    for y in 0..=max_y {
        for x in 0..max_x {
            let point = Point { x, y };
            let crossings = vents
                .iter()
                .filter(|l| l.has_point_in_line(point))
                .count();

            let dot = match crossings {
                0 => ".".to_string(),
                _ => format!("{}", crossings)
            };

            print!("{} ", dot);
        }

        println!()
    }
}

#[cfg(test)]
mod test {

    macro_rules! line_test {
        ($test: ident, ($test_point_x: literal, $test_point_y: literal), ($test_line_pt_start_x: literal, $test_line_pt_start_y: literal) -> ($test_line_pt_end_x: literal, $test_line_pt_end_y: literal), $outcome: literal) => {
            #[test]
            fn $test(){
                let test_point = crate::Point { x: $test_point_x, y: $test_point_y };

                let line = crate::Line {
                    start: crate::Point { x: $test_line_pt_start_x, y: $test_line_pt_start_y },
                    end: crate::Point { x: $test_line_pt_end_x, y: $test_line_pt_end_y }
                };

                let actual = line.has_point_in_line(test_point);
                assert_eq!(actual, $outcome);
            }
        };
    }

    line_test!(on_horizontal_line,          (3, 5), (0, 5) -> (5, 5), true);
    line_test!(on_horizontal_line_start,    (0, 5), (0, 5) -> (5, 5), true);
    line_test!(on_horizontal_line_end,      (5, 5), (0, 5) -> (5, 5), true);

    line_test!(on_vertical_line,            (0, 5), (0, 0) -> (0, 10), true);
    line_test!(on_vertical_line_start,      (0, 0), (0, 0) -> (0, 10), true);
    line_test!(on_vertical_line_end,        (0, 10), (0, 0) -> (0, 10), true);

    line_test!(not_on_horizontal_line,      (8, 5), (0, 5) -> (5, 5), false);
    line_test!(not_on_horizontal_line_start,(0, 5), (1, 5) -> (5, 5), false);
    line_test!(not_on_horizontal_line_end,  (6, 5), (1, 5) -> (5, 5), false);

    line_test!(not_on_vertical_line,         (0, 5), (1, 1) -> (1, 10), false);
    line_test!(not_on_vertical_line_start,   (1, 0), (1, 1) -> (1, 10), false);
    line_test!(not_on_vertical_line_end,     (1, 11), (1, 1) -> (1, 10), false);
}