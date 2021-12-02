# Code for the Advent of code 2021

This repository contains the code I have written to solve puzzles from the [Advent of code 2021](https://adventofcode.com/2021), in Rust.

Each day is contained in its own crate and the parts are delimited by functions like `part1` and `part2`. Note that it may not have been possible to cleanly separate the part, in which case I will certainly put some comments in the code.

The input files are in `files/day*`. There is one file for the example given in the instructions and one file for my specific input because why not, for now. 

Also note that some days may have been skipped, if I don't find the time to solve the puzzles or is a bit out of reach for me.

## Running one specific day

Execute `cargo run --bin day<something> -- <input file>`.