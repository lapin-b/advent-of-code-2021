pub const COLUMN_COUNT_PER_GRID: usize = 5;
pub const LINE_COUNT_PER_GRID: usize = 5;

#[derive(Debug, Clone)]
pub struct Board {
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
            if line == &[None; COLUMN_COUNT_PER_GRID] {
                return true;
            }
        }

        for column in self.columns() {
            let column = column.as_slice();
            if column == &[None; LINE_COUNT_PER_GRID] {
                return true;
            }
        }

        return false;
    }

    pub fn lines(&self) -> BoardLineIterator {
        BoardLineIterator {
            board: &self,
            current_line: 0
        }
    }

    pub fn columns(&self) -> BoardColumnIterator {
        BoardColumnIterator {
            board: &self,
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
        let interval_start = n * COLUMN_COUNT_PER_GRID;
        let interval_end = n * COLUMN_COUNT_PER_GRID + COLUMN_COUNT_PER_GRID;

        self.numbers.get(interval_start..interval_end)
    }

    pub fn get_column(&self, n: usize) -> Option<Vec<Option<u32>>> {

        let mut column = Vec::with_capacity(LINE_COUNT_PER_GRID);

        for col_line in 0..LINE_COUNT_PER_GRID {
            let idx = col_line * LINE_COUNT_PER_GRID + n;

            let number_at_that_line = match self.numbers.get(idx) {
                Some(n) => n,
                None => return None,
            };

            column.push(*number_at_that_line);
        }

        return Some(column);
    }
}
