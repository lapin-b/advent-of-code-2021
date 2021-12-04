pub const COLUMN_COUNT_PER_GRID: usize = 5;
pub const LINE_COUNT_PER_GRID: usize = 5;

#[derive(Debug, Clone)]
pub struct Board {
    // One number that have been marked will be turned into None
    // Lines are placed one directly after the other like so:
    // [ <line 1 items> <line 2 items> ... ]
    numbers: Vec<Option<u32>>,
}

pub struct BoardLineIterator<'board> {
    board: &'board Board,
    current_line: usize,
}

pub struct BoardColumnIterator<'board> {
    board: &'board Board,
    current_column: usize
}

impl<'board> Iterator for BoardLineIterator<'board> {
    type Item = &'board [Option<u32>];

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.board.get_line(self.current_line);
        self.current_line += 1;
        line
    }
}

impl<'board> Iterator for BoardColumnIterator<'board> {
    type Item = Vec<Option<u32>>;

    fn next(&mut self) -> Option<Self::Item> {
        let column = self.board.get_column(self.current_column);
        self.current_column += 1;
        column
    }
}

impl Board {
    pub fn new(numbers: &[u32]) -> Self {
        Self {
            numbers: numbers.iter().map(|n| Some(*n)).collect()
        }
    }

    pub fn mark(&mut self, number_to_mark: u32) {
        let element = self.numbers
            .iter()
            .enumerate()
            .find(
                |(_idx, number)| match **number {
                    None => false,
                    Some(n) => n == number_to_mark
                }
            );

        if let Some((idx, _)) = element {
            self.numbers[idx] = None;
        }
    }

    pub fn is_win(&self) -> bool {

        for line in self.lines() {
            if line == [None; COLUMN_COUNT_PER_GRID] {
                return true;
            }
        }

        for column in self.columns() {
            let column = column.as_slice();
            if column == [None; LINE_COUNT_PER_GRID] {
                return true;
            }
        }

        false
    }

    pub fn lines(&self) -> BoardLineIterator {
        BoardLineIterator {
            board: self,
            current_line: 0
        }
    }

    pub fn columns(&self) -> BoardColumnIterator {
        BoardColumnIterator {
            board: self,
            current_column: 0
        }
    }

    pub fn calculate_score(&self) -> u32 {
        self.numbers
            .iter()
            .filter_map(|n| *n)
            .sum()
    }

    pub fn get_line(&self, n: usize) -> Option<&[Option<u32>]> {
        // Since each line is place one after the other in the numbers vector, grabbing one line is
        // trivial: determine the interval start on the nth line column zero and the interval end
        // at the end of the line (a.k.a the number of columns in the line).

        // Formula: The nth line we want to fetch * number of columns per line
        let interval_start = n * COLUMN_COUNT_PER_GRID;

        // Formula: The nth line we want to fetch * number of columns per line + number of columns per line
        let interval_end = n * COLUMN_COUNT_PER_GRID + COLUMN_COUNT_PER_GRID;

        self.numbers.get(interval_start..interval_end)
    }

    pub fn get_column(&self, n: usize) -> Option<Vec<Option<u32>>> {
        let mut column = Vec::with_capacity(COLUMN_COUNT_PER_GRID);

        // Getting one column gets a bit trickier: we want the nth column on each line
        for current_line in 0..LINE_COUNT_PER_GRID {
            // Formula: current line we're looking at * the number of columns per line + the nth column we're
            // looking for in the line

            // `current line we're looking at * the number of columns per line` allows us to point ourselves
            // on a specific line and `the nth column we're looking for in the line` fetches the the nth column.
            let idx = current_line * COLUMN_COUNT_PER_GRID + n;

            // In case the column does not exist or is incomplete, independently of whenever the whole column
            // has been marked or not, we return None. Otherwise, put the number we find there on the vector
            let number_at_that_line = match self.numbers.get(idx) {
                Some(n) => n,
                None => return None,
            };

            column.push(*number_at_that_line);
        }

        Some(column)
    }
}
