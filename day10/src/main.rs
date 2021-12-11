use std::env::args;
use std::fs;
use std::io::BufReader;

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

fn get_closing_symbol(symbol: char) -> Option<char> {
    match symbol {
        '<' => Some('>'),
        '[' => Some(']'),
        '{' => Some('}'),
        '(' => Some(')'),
        _ => None
    }
}

fn check_line(line: &str) -> Result<(), SyntaxError> {
    let mut stack = Vec::new();

    for (idx, symbol) in line.chars().enumerate() {
        let is_opening = match symbol {
            '<' | '[' | '{' | '(' => true,
            '>' | ']' | '}' | ')' => false,
            _ => unreachable!("Unknown symbol {} in col {}", symbol, idx),
        };

        if is_opening {
            stack.push(symbol);
        } else {
            let opening_symbol = stack.pop().expect("Unexpected empty stack");
            let expected_closing_symbol = get_closing_symbol(opening_symbol).unwrap();

            if symbol != expected_closing_symbol {
                return Err(SyntaxError::UnexpectedClosingChar { expected: expected_closing_symbol, got: symbol })
            }
        }
    }

    if stack.len() > 0 {
        return Err(SyntaxError::IncompleteLine);
    }

    Ok(())
}

fn main() {
    let filename = args().nth(1).expect("USAGE: day8 <input file>");
    let content = fs::read_to_string(filename).unwrap();
    let lines = content
        .lines()
        .collect::<Vec<_>>();


    part1(&lines);
}

fn part1(lines: &[&str]) {
    let mut syntax_error_symbols = SyntaxViolationsContainer::new();

    for line in lines {
        print!("{} -> ", line);

        match check_line(*line) {
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