use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use board::Board;
use board::LINE_COUNT_PER_GRID;

mod board;

fn main() {
    // Read the file and parse the contents of it so we can extract the numbers and the boards
    let filename = args().nth(1).unwrap();
    let file = File::open(&filename).expect("File does not exist");
    let file = BufReader::new(file);

    let mut board_file_content = file
        .split(0x0A)
        .map(Result::unwrap)
        .map(String::from_utf8)
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    // Extract the random numbers
    let numbers = board_file_content
        .remove(0)
        .split(",")
        .filter(|n| !n.is_empty())
        .map(u32::from_str)
        .map(Result::unwrap)
        .collect::<Vec<u32>>();

    // Extract each game grid from the file
    let boards = {
        let mut boards = vec![];

        for board in board_file_content.chunks(LINE_COUNT_PER_GRID) {
            let board = board
                .iter()
                .map(|s| s.split(" "))
                .flatten()
                .filter(|n| !n.is_empty())
                .map(u32::from_str)
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            boards.push(Board::new(&board));
        }

        boards
    };

    part1(&numbers, &boards);
}

fn part1(numbers: &[u32], grids: &[Board]) {
    // Grab a copy of the boards for ourselves since we will modify it through the marking of each number
    let mut grids = grids.to_vec();
    let mut winning_board = None;
    let mut last_called = None;

    // Mark each number in the grids
    'outer: for number in numbers {
        let number = *number;

        for (idx, grid) in grids.iter_mut().enumerate() {
            grid.mark(number);
            if grid.is_win() {
                winning_board = Some(idx);
                last_called = Some(number);
                break 'outer;
            }
        }
    }

    // We are out of the for loops, check if we effectively have a winning board
    if let Some(winning_index) = winning_board {
        let grid = grids.get(winning_index).expect("Somehow, the winning board is not in the array");
        let last_called = last_called.expect("How do you win if you haven't even called a number ?");

        let score = grid.calculate_score();
        println!("Winning grid at {} with grid score of {}, last called number {}, final score: {}", winning_index, score, last_called, last_called * score);
    } else {
        println!("No winning grid :(")
    }
}
