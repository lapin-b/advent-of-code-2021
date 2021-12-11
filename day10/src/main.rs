use std::borrow::Cow;
use std::env::args;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Add, AddAssign};

const ILLEGAL_PARENTHESIS_SCORE: u32 = 3;
const ILLEGAL_SQUARE_BRACKET_SCORE: u32 = 57;
const ILLEGAL_CURLY_BRACKET_SCORE: u32 = 1197;
const ILLEGAL_ANGLE_BRACKET_SCORE: u32 = 25137;

const COMPLETION_PARENTHESIS_SCORE: u64 = 1;
const COMPLETION_SQUARE_BRACKET_SCORE: u64 = 2;
const COMPLETION_CURLY_BRACKET_SCORE: u64 = 3;
const COMPLETION_ANGLE_BRACKET_SCORE: u64 = 4;

enum SyntaxError {
    IncompleteLine { stack: Vec<char> },
    UnexpectedClosingChar { expected: char, got: char, stack: Vec<char> }
}

#[derive(Debug)]
struct SyntaxViolationsContainer {
    pub angle_bracket: u32,
    pub square_bracket: u32,
    pub curly_bracket: u32,
    pub parenthesis: u32,
}

struct LineChecker<'line> {
    line: &'line str,
}

impl SyntaxViolationsContainer {
    fn new() -> Self {
        Self {
            angle_bracket: 0,
            square_bracket: 0,
            curly_bracket: 0,
            parenthesis: 0
        }
    }

    fn increment(&mut self, character: char) {
        match character {
            '>' | '<' => self.angle_bracket += 1,
            '}' | '{' => self.curly_bracket += 1,
            ']' | '[' => self.square_bracket += 1,
            ')' | '(' => self.parenthesis += 1,
            _ => unreachable!("Unknown character {}", character)
        }
    }
}

impl<'line> LineChecker<'line> {
    pub fn new(line: &'line str) -> Self {
        Self {
            line,
        }
    }

    pub fn check(&self) -> Result<(), SyntaxError> {
        let mut stack = Vec::new();

        for (idx, symbol) in self.line.chars().enumerate() {
            let is_opening = match symbol {
                '<' | '[' | '{' | '(' => true,
                '>' | ']' | '}' | ')' => false,
                _ => unreachable!("Unknown symbol {} in col {}", symbol, idx),
            };

            if is_opening {
                stack.push(symbol);
            } else {
                let opening_symbol = stack.pop().expect("Unexpected empty stack");
                let expected_closing_symbol = Self::closing_symbol_for(opening_symbol).unwrap();

                if symbol != expected_closing_symbol {
                    return Err(SyntaxError::UnexpectedClosingChar { expected: expected_closing_symbol, got: symbol, stack })
                }
            }
        }

        if !stack.is_empty() {
            return Err(SyntaxError::IncompleteLine { stack });
        }

        Ok(())
    }

    // Option of the completed line and the score
    pub fn complete(&self, stack: Option<&[char]>) -> Option<(String, u64)> {
        let mut completed_line = self.line.to_string();
        let mut score = 0;

        let mut stack = match stack {
            Some(stack) => stack.to_vec(),
            None => match self.check() {
                Ok(_) => return Some((completed_line, score)),
                Err(e) => match e {
                    SyntaxError::IncompleteLine { stack } => stack,
                    SyntaxError::UnexpectedClosingChar { .. } => return None
                }
            }
        };

        while let Some(remaining_opening_symbol) = stack.pop() {
            let closing_symbol = Self::closing_symbol_for(remaining_opening_symbol)
                .expect("Unexpected empty stack while completing");

            let mut buf = [0; 1];
            completed_line += closing_symbol.encode_utf8(&mut buf);

            let score_to_add = match closing_symbol {
                '>' => COMPLETION_ANGLE_BRACKET_SCORE,
                ']' => COMPLETION_SQUARE_BRACKET_SCORE,
                '}' => COMPLETION_CURLY_BRACKET_SCORE,
                ')' => COMPLETION_PARENTHESIS_SCORE,
                _ => unreachable!()
            };

            score = score * 5 + score_to_add;
        };

        Some((completed_line, score))
    }

    pub fn closing_symbol_for(symbol: char) -> Option<char> {
        match symbol {
            '<' => Some('>'),
            '[' => Some(']'),
            '{' => Some('}'),
            '(' => Some(')'),
            _ => None
        }
    }
}

impl<'line> Display for LineChecker<'line> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.line.fmt(f)
    }
}

fn main() {
    let filename = args().nth(1).expect("USAGE: day8 <input file>");
    let content = fs::read_to_string(filename).unwrap();
    let lines = content
        .lines()
        .map(LineChecker::new)
        .collect::<Vec<_>>();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[LineChecker]) {
    let mut syntax_error_symbols = SyntaxViolationsContainer::new();

    for line in lines {
        print!("{} -> ", line);

        match line.check() {
            Ok(_) => {
                println!("OK");
                true
            },

            Err(error) => match error {
                SyntaxError::IncompleteLine { .. } => {
                    println!("WARN: Incomplete line");
                    true
                },
                SyntaxError::UnexpectedClosingChar { expected, got, .. } => {
                    println!("ERR: expected {}, got {}", expected, got);
                    syntax_error_symbols.increment(got);
                    false
                }
            }
        };
    }

    let syntax_errors_score =
        syntax_error_symbols.parenthesis * ILLEGAL_PARENTHESIS_SCORE +
            syntax_error_symbols.square_bracket * ILLEGAL_SQUARE_BRACKET_SCORE +
            syntax_error_symbols.curly_bracket * ILLEGAL_CURLY_BRACKET_SCORE +
            syntax_error_symbols.angle_bracket * ILLEGAL_ANGLE_BRACKET_SCORE;

    println!("Score for the first part: {}", syntax_errors_score);
}

fn part2(lines: &[LineChecker]) {
    let mut completed_lines_scores = lines
        .iter()
        .filter_map(|line| {
            match line.check() {
                // Complete lines are already complete, no need to autocomplete
                Ok(_) => None,
                Err(e) => match e {
                    // Corrupted lines are thrown out
                    SyntaxError::UnexpectedClosingChar { .. } => None,
                    SyntaxError::IncompleteLine { stack } => Some((line, stack)),
                }
            }
        })
        .map(|(line, stack)| line.complete(Some(&stack)))
        .map(|l| l.unwrap())
        .map(|(complete_line, score)| {
            println!("Completed line: {} -> {}", complete_line, score);
            score
        })
       .collect::<Vec<_>>();

    completed_lines_scores.sort_unstable();
    let idx = completed_lines_scores.len() / 2;
    let completion_score = completed_lines_scores[idx];

    println!(
        "Fetched line {} ({} elements) after sorting with score {}",
        idx,
        completed_lines_scores.len(),
        completion_score
    )
}