use std::env::args;
use std::fmt::{Display, Formatter};
use std::fs;

const ILLEGAL_PARENTHESIS_SCORE: u32 = 3;
const ILLEGAL_SQUARE_BRACKET_SCORE: u32 = 57;
const ILLEGAL_CURLY_BRACKET_SCORE: u32 = 1197;
const ILLEGAL_ANGLE_BRACKET_SCORE: u32 = 25137;

enum SyntaxError {
    IncompleteLine,
    UnexpectedClosingChar { expected: char, got: char }
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
    stack: Vec<char>
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
            stack: Vec::new()
        }
    }

    pub fn check(&mut self) -> Result<(), SyntaxError> {
        for (idx, symbol) in self.line.chars().enumerate() {
            let is_opening = match symbol {
                '<' | '[' | '{' | '(' => true,
                '>' | ']' | '}' | ')' => false,
                _ => unreachable!("Unknown symbol {} in col {}", symbol, idx),
            };

            if is_opening {
                self.stack.push(symbol);
            } else {
                let opening_symbol = self.stack.pop().expect("Unexpected empty stack");
                let expected_closing_symbol = Self::closing_symbol_for(opening_symbol).unwrap();

                if symbol != expected_closing_symbol {
                    return Err(SyntaxError::UnexpectedClosingChar { expected: expected_closing_symbol, got: symbol })
                }
            }
        }

        if self.stack.len() > 0 {
            return Err(SyntaxError::IncompleteLine);
        }

        Ok(())
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
    let mut lines = content
        .lines()
        .map(LineChecker::new)
        .collect::<Vec<_>>();


    part1(&mut lines);
}

fn part1(lines: &mut [LineChecker]) {
    let mut syntax_error_symbols = SyntaxViolationsContainer::new();

    for line in lines {
        print!("{} -> ", line);

        match line.check() {
            Ok(_) => {
                println!("OK");
                true
            },

            Err(error) => match error {
                SyntaxError::IncompleteLine => {
                    println!("WARN: Incomplete line");
                    true
                },
                SyntaxError::UnexpectedClosingChar { expected, got } => {
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